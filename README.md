# LyricSync 🎵

A high-performance Rust tool for embedding LRC lyrics into audio files (FLAC,
MP3, M4A). Perfect for organizing your music library with synchronized lyrics.

## Features ✨

- **Multi-format Support**: FLAC, MP3, and M4A files
- **Smart Processing**: Skip files that already have embedded lyrics
- **Batch Operations**: Process entire directories recursively
- **File Management**: Optionally delete LRC files after embedding
- **Progress Tracking**: Real-time progress bar with detailed statistics
- **Shell Completion**: Full shell completion support for bash, zsh, fish, and more
- **High Performance**: Built in Rust for blazing fast processing

## Installation 🔧

### Pre-built Binaries

Download the latest release from the [releases page](https://github.com/dantescur/lyricsync/releases).

### From Source

```bash
git clone https://github.com/dantescur/lyricsync.git
cd lyricsync
cargo install --path .
```

### Cargo Install

```bash
cargo install lyricsync
```

## Usage 🚀

### Basic Usage

```bash
# Embed lyrics for all audio files in a directory
lyricsync -d /path/to/music

# Skip files that already have lyrics and delete LRC files after embedding
lyricsync -d /path/to/music -s -r

# Process subdirectories recursively
lyricsync -d /path/to/music -R
```

### Command Line Options

| Option            | Short | Long                    | Description                                                    |
| ----------------- | ----- | ----------------------- | -------------------------------------------------------------- |
| **Directory**     | `-d`  | `--directory`           | **Required**. Path to directory containing audio and LRC files |
| **Skip Existing** | `-s`  | `--skip`                | Skip files that already have embedded lyrics                   |
| **Reduce**        | `-r`  | `--reduce`              | Delete LRC files after successful embedding                    |
| **Recursive**     | `-R`  | `--recursive`           | Process subdirectories recursively                             |
| **Completion**    |       | `--generate-completion` | Generate shell completion script                               |

### Shell Completion

Generate completion scripts for your shell:

```bash
# Bash
lyricsync --generate-completion bash > ~/.config/bash_completion/lyricsync

# Zsh
lyricsync --generate-completion zsh > ~/.zsh/completions/_lyricsync

# Fish
lyricsync --generate-completion fish > ~/.config/fish/completions/lyricsync.fish

# PowerShell
lyricsync --generate-completion powershell > lyricsync.ps1
```

## File Structure Requirements 📁

Your music directory should be organized like this:

```sh
music_folder/
├── song1.flac
├── song1.lrc
├── song2.mp3
├── song2.lrc
├── song3.m4a
├── song3.lrc
├── album1/
│   ├── track1.flac
│   ├── track1.lrc
│   └── track2.flac
└── album2/
    ├── song1.m4a
    └── song1.lrc
```

**Note**: LRC files must have the same base name as their corresponding audio files.

## Examples 💡

### Organize Your Music Library

```bash
# Process your entire music library
lyricsync -d "~/Music" -s -r -R
```

### Process Specific Albums

```bash
# Process a single album, keeping LRC files
lyricsync -d "~/Music/My_Favorite_Album"

# Process and clean up LRC files
lyricsync -d "~/Music/My_Favorite_Album" -r
```

## Supported Formats 🔍

| Format   | Lyrics Storage Method                    |
| -------- | ---------------------------------------- |
| **FLAC** | Vorbis Comment with "LYRICS" field       |
| **MP3**  | ID3v2 USLT (Unsynchronized Lyrics) frame |
| **M4A**  | iTunes metadata with `©lyr` atom        |

## Performance 📊

LyricSync is built in Rust for maximum performance:

- **Fast Processing**: Processes thousands of files in seconds
- **Low Memory Usage**: Efficient streaming and metadata handling

## Error Handling ⚠️

- Failed embeddings are clearly reported
- Original LRC files are preserved as `.lrc.failed`
- Detailed error messages for troubleshooting
- Progress tracking with file-specific status

## Building from Source 🛠️

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))

### Build Steps

```bash
# Debug build
cargo build

# Release build (recommended)
cargo build --release

# Check code quality
cargo clippy --all-targets --all-features --workspace -- -D warnings
```

## Contributing 🤝

Contributions are welcome! Please feel free to submit pull requests, open issues
, or suggest new features.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.

## Acknowledgments 🙏

- Highly inspired by [lrcput](https://github.com/JustOptimize/lrcput)

## Support 💬

If you encounter any issues or have questions:

1. Check the [issues page](https://github.com/dantescur/lyricsync/issues)
2. Create a new issue with detailed information
3. Include your operating system and LyricSync version

---

**Made with ❤️ by [dantescur](https://github.com/dantescur)**

_Sync your lyrics, enhance your music experience!_
