use clap::{Arg, Command, ValueHint};
use clap_complete::{Generator, Shell, generate};
use indicatif::{ProgressBar, ProgressStyle};
use lofty::{
  TextEncoding,
  config::{ParseOptions, WriteOptions},
  file::AudioFile,
  flac::FlacFile,
  id3::v2::FrameId,
  mp4::Mp4File,
  mp4::{Atom, AtomData},
  mpeg::MpegFile,
};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum LrcError {
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),
  #[error("Audio file error: {0}")]
  Audio(#[from] lofty::error::LoftyError),
  #[error("Unsupported file format: {0}")]
  UnsupportedFormat(String),
}

type Result<T> = std::result::Result<T, LrcError>;

struct EmbedStats {
  total_audio_files: usize,
  embedded_lyrics: usize,
  failed_files: Vec<PathBuf>,
}

fn has_embedded_lyrics(audio_path: &Path) -> Result<bool> {
  let mut file_content = OpenOptions::new().read(true).open(audio_path)?;

  if audio_path.extension().is_some_and(|ext| ext == "flac") {
    let flac_file = FlacFile::read_from(&mut file_content, ParseOptions::new())?;
    if let Some(vorbis_comments) = flac_file.vorbis_comments() {
      return Ok(vorbis_comments.get("LYRICS").is_some() || vorbis_comments.get("UNSYNCEDLYRICS").is_some());
    }
  } else if audio_path.extension().is_some_and(|ext| ext == "mp3") {
    let mp3_file = MpegFile::read_from(&mut file_content, ParseOptions::new())?;
    if let Some(id3v2) = mp3_file.id3v2() {
      // Check for USLT (unsynchronized lyrics) or SYLT (synchronized lyrics) frames
      let uslt_frame_id = FrameId::new("USLT").unwrap();
      let sylt_frame_id = FrameId::new("SYLT").unwrap();
      return Ok(id3v2.get(&uslt_frame_id).is_some() || id3v2.get(&sylt_frame_id).is_some());
    }
  } else if audio_path.extension().is_some_and(|ext| ext == "m4a") {
    let mp4_file = Mp4File::read_from(&mut file_content, ParseOptions::new())?;
    if let Some(ilst) = mp4_file.ilst() {
      // Check for lyrics in MP4 metadata
      let lyrics_ident = lofty::mp4::AtomIdent::Fourcc(*b"\xa9lyr");
      return Ok(ilst.get(&lyrics_ident).is_some());
    }
  }

  Ok(false)
}

fn embed_lrc_to_file(audio_path: &Path, lrc_path: &Path, reduce_lrc: bool) -> Result<()> {
  let lyrics_content = fs::read_to_string(lrc_path)?;

  if audio_path.extension().is_some_and(|ext| ext == "flac") {
    embed_lrc_to_flac(audio_path, &lyrics_content)?;
  } else if audio_path.extension().is_some_and(|ext| ext == "mp3") {
    embed_lrc_to_mp3(audio_path, &lyrics_content)?;
  } else if audio_path.extension().is_some_and(|ext| ext == "m4a") {
    embed_lrc_to_m4a(audio_path, &lyrics_content)?;
  } else {
    return Err(LrcError::UnsupportedFormat(audio_path.extension().unwrap_or_default().to_string_lossy().to_string()));
  }

  if reduce_lrc {
    fs::remove_file(lrc_path)?;
  }

  Ok(())
}

fn embed_lrc_to_flac(audio_path: &Path, lyrics: &str) -> Result<()> {
  let mut file_content = OpenOptions::new().read(true).write(true).open(audio_path)?;
  let mut flac_file = FlacFile::read_from(&mut file_content, ParseOptions::new())?;

  if let Some(vorbis_comments) = flac_file.vorbis_comments_mut() {
    vorbis_comments.insert("LYRICS".to_string(), lyrics.to_string());
    flac_file.save_to_path(audio_path, WriteOptions::default())?;
  }

  Ok(())
}

fn embed_lrc_to_mp3(audio_path: &Path, lyrics: &str) -> Result<()> {
  let mut file_content = OpenOptions::new().read(true).write(true).open(audio_path)?;
  let mut mp3_file = MpegFile::read_from(&mut file_content, ParseOptions::new())?;

  if let Some(id3v2) = mp3_file.id3v2_mut() {
    use lofty::id3::v2::{Frame, UnsynchronizedTextFrame};

    let uslt_frame = UnsynchronizedTextFrame::new(
      TextEncoding::UTF8,
      [b'e', b'n', b'g'], // Language: eng
      "".to_string(),     // Description
      lyrics.to_string(),
    );
    id3v2.insert(Frame::UnsynchronizedText(uslt_frame));

    mp3_file.save_to_path(audio_path, WriteOptions::default())?;
  }

  Ok(())
}

fn embed_lrc_to_m4a(audio_path: &Path, lyrics: &str) -> Result<()> {
  let mut file_content = OpenOptions::new().read(true).write(true).open(audio_path)?;
  let mut mp4_file = Mp4File::read_from(&mut file_content, ParseOptions::new())?;

  if let Some(ilst) = mp4_file.ilst_mut() {
    // Create lyrics atom for MP4
    let lyrics_ident = lofty::mp4::AtomIdent::Fourcc(*b"\xa9lyr");
    let lyrics_atom = Atom::new(lyrics_ident, AtomData::UTF8(lyrics.to_string()));
    ilst.insert(lyrics_atom);

    mp4_file.save_to_path(audio_path, WriteOptions::default())?;
  }

  Ok(())
}

fn embed_lrc(directory: &Path, skip_existing: bool, reduce_lrc: bool, recursive: bool) -> Result<EmbedStats> {
  let mut stats = EmbedStats { total_audio_files: 0, embedded_lyrics: 0, failed_files: Vec::new() };

  let walker = if recursive { WalkDir::new(directory) } else { WalkDir::new(directory).max_depth(1) };

  let audio_files: Vec<PathBuf> = walker
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|entry| {
      entry.file_type().is_file()
        && entry.path().extension().is_some_and(|ext| matches!(ext.to_str(), Some("flac" | "mp3" | "m4a")))
    })
    .map(|entry| entry.into_path())
    .collect();

  stats.total_audio_files = audio_files.len();

  let pb = ProgressBar::new(audio_files.len() as u64);
  pb.set_style(
    ProgressStyle::default_bar()
      .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
      .unwrap()
      .progress_chars("#>-"),
  );

  for audio_path in audio_files {
    let file_name = audio_path.file_stem().unwrap_or_default();
    let lrc_path = audio_path.with_file_name(format!("{}.lrc", file_name.to_string_lossy()));

    if !lrc_path.exists() {
      pb.inc(1);
      continue;
    }

    if skip_existing {
      match has_embedded_lyrics(&audio_path) {
        Ok(true) => {
          pb.set_message(format!("Skipped: {}", audio_path.display()));
          pb.inc(1);
          continue;
        },
        Ok(false) => {}, // Continue with embedding
        Err(e) => {
          eprintln!("Error checking lyrics for {}: {}", audio_path.display(), e);
        },
      }
    }

    match embed_lrc_to_file(&audio_path, &lrc_path, reduce_lrc) {
      Ok(()) => {
        stats.embedded_lyrics += 1;
        pb.set_message(format!("Embedded: {}", audio_path.display()));
      },
      Err(e) => {
        eprintln!("Error embedding LRC for {}: {}", audio_path.display(), e);
        stats.failed_files.push(audio_path.clone());

        // Rename failed LRC file
        let failed_lrc_path = lrc_path.with_extension("lrc.failed");
        if let Err(e) = fs::rename(&lrc_path, &failed_lrc_path) {
          eprintln!("Error renaming failed LRC file: {}", e);
        }
      },
    }

    pb.inc(1);
  }

  pb.finish_with_message("Completed!");
  Ok(stats)
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
  generate(generator, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn main() -> Result<()> {
  let banner = r#"
██      ██    ██ ██████  ██  ██████     ███████ ██    ██ ███    ██  ██████ 
██       ██  ██  ██   ██ ██ ██          ██       ██  ██  ████   ██ ██      
██        ████   ██████  ██ ██          ███████   ████   ██ ██  ██ ██      
██         ██    ██   ██ ██ ██               ██    ██    ██  ██ ██ ██      
███████    ██    ██   ██ ██  ██████     ███████    ██    ██   ████  ██████ 
                            Created by Daniel
"#;

  let mut cmd = Command::new("lyricsync")
    .version("1.0.0")
    .author("Daniel")
    .about("Embed LRC lyrics into audio files (FLAC, MP3, M4A)")
    .arg(
      Arg::new("directory")
        .short('d')
        .long("directory")
        .value_name("DIRECTORY")
        .help("Directory containing audio and LRC files")
        .required(true)
        .value_hint(ValueHint::DirPath),
    )
    .arg(
      Arg::new("skip")
        .short('s')
        .long("skip")
        .help("Skip files that already have embedded lyrics")
        .action(clap::ArgAction::SetTrue),
    )
    .arg(
      Arg::new("reduce")
        .short('r')
        .long("reduce")
        .help("Delete LRC files after successful embedding")
        .action(clap::ArgAction::SetTrue),
    )
    .arg(
      Arg::new("recursive")
        .short('R')
        .long("recursive")
        .help("Process subdirectories recursively")
        .action(clap::ArgAction::SetTrue),
    )
    .arg(
      Arg::new("generate-completion")
        .long("generate-completion")
        .value_name("SHELL")
        .value_parser(["bash", "zsh", "fish", "powershell", "elvish"])
        .help("Generate shell completion script"),
    );

  let matches = cmd.clone().get_matches();

  // Handle completion generation first
  if let Some(generator) = matches.get_one::<String>("generate-completion") {
    match generator.as_str() {
      "bash" => print_completions(Shell::Bash, &mut cmd),
      "zsh" => print_completions(Shell::Zsh, &mut cmd),
      "fish" => print_completions(Shell::Fish, &mut cmd),
      "powershell" => print_completions(Shell::PowerShell, &mut cmd),
      "elvish" => print_completions(Shell::Elvish, &mut cmd),
      _ => unreachable!(),
    }
    return Ok(());
  }

  println!("{}", banner);

  let directory = matches.get_one::<String>("directory").unwrap();
  let skip_existing = matches.get_flag("skip");
  let reduce_lrc = matches.get_flag("reduce");
  let recursive = matches.get_flag("recursive");

  let stats = embed_lrc(Path::new(directory), skip_existing, reduce_lrc, recursive)?;

  let percentage = if stats.total_audio_files > 0 {
    (stats.embedded_lyrics as f64 / stats.total_audio_files as f64) * 100.0
  } else {
    0.0
  };

  println!("\nSummary:");
  println!("Total audio files: {}", stats.total_audio_files);
  println!("Embedded lyrics in {} audio files", stats.embedded_lyrics);
  println!("Success rate: {:.2}%", percentage);

  if !stats.failed_files.is_empty() {
    println!("\nFailed to embed LRC for the following files:");
    for file in stats.failed_files {
      println!("  {}", file.display());
    }
  }

  Ok(())
}
