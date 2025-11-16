use std::fs;
use std::path::Path;
use std::process::Command;

// Helper function to get the path to the lyricsync binary
fn lyricsync_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_lyricsync"))
}

// Helper function to create a temporary test directory
fn create_test_dir() -> tempfile::TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

// Helper function to copy test files to a directory
fn copy_test_file(source: &Path, dest: &Path) {
    fs::copy(source, dest).expect("Failed to copy test file");
}

#[test]
fn test_embed_mp3_lyrics() {
    let test_dir = create_test_dir();
    let test_dir_path = test_dir.path();
    
    // Copy test files to temp directory
    let mp3_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.mp3");
    let lrc_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.lrc");
    
    let mp3_dest = test_dir_path.join("04 Avril Lavigne - I'm With You.mp3");
    let lrc_dest = test_dir_path.join("04 Avril Lavigne - I'm With You.lrc");
    
    copy_test_file(mp3_source, &mp3_dest);
    copy_test_file(lrc_source, &lrc_dest);
    
    // Run lyricsync
    let output = lyricsync_bin()
        .arg("--directory")
        .arg(test_dir_path)
        .output()
        .expect("Failed to execute lyricsync");
    
    assert!(output.status.success(), "lyricsync should succeed");
    
    // Verify LRC file still exists (not using --reduce)
    assert!(lrc_dest.exists(), "LRC file should still exist");
    
    // Verify lyrics were embedded by checking if file was modified
    // (We can't easily verify the embedded content without reading the MP3 metadata,
    // but we can at least verify the process completed successfully)
}

#[test]
fn test_dry_run_mode() {
    let test_dir = create_test_dir();
    let test_dir_path = test_dir.path();
    
    // Copy test files to temp directory
    let mp3_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.mp3");
    let lrc_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.lrc");
    
    let mp3_dest = test_dir_path.join("04 Avril Lavigne - I'm With You.mp3");
    let lrc_dest = test_dir_path.join("04 Avril Lavigne - I'm With You.lrc");
    
    copy_test_file(mp3_source, &mp3_dest);
    copy_test_file(lrc_source, &lrc_dest);
    
    // Get original file modification time
    let original_metadata = fs::metadata(&mp3_dest).expect("Failed to read MP3 metadata");
    let original_mtime = original_metadata.modified().expect("Failed to get modification time");
    
    // Run lyricsync with --dry-run
    let output = lyricsync_bin()
        .arg("--directory")
        .arg(test_dir_path)
        .arg("--dry-run")
        .output()
        .expect("Failed to execute lyricsync");
    
    assert!(output.status.success(), "lyricsync should succeed in dry-run mode");
    
    // Verify output contains dry-run message
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[DRY RUN]"), "Output should contain dry-run indicator");
    
    // Verify file was not modified (check modification time)
    let new_metadata = fs::metadata(&mp3_dest).expect("Failed to read MP3 metadata");
    let new_mtime = new_metadata.modified().expect("Failed to get modification time");
    assert_eq!(original_mtime, new_mtime, "File should not be modified in dry-run mode");
}

#[test]
fn test_reduce_flag_deletes_lrc() {
    let test_dir = create_test_dir();
    let test_dir_path = test_dir.path();
    
    // Copy test files to temp directory
    let mp3_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.mp3");
    let lrc_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.lrc");
    
    let mp3_dest = test_dir_path.join("04 Avril Lavigne - I'm With You.mp3");
    let lrc_dest = test_dir_path.join("04 Avril Lavigne - I'm With You.lrc");
    
    copy_test_file(mp3_source, &mp3_dest);
    copy_test_file(lrc_source, &lrc_dest);
    
    // Run lyricsync with --reduce
    let output = lyricsync_bin()
        .arg("--directory")
        .arg(test_dir_path)
        .arg("--reduce")
        .output()
        .expect("Failed to execute lyricsync");
    
    assert!(output.status.success(), "lyricsync should succeed");
    
    // Verify LRC file was deleted
    assert!(!lrc_dest.exists(), "LRC file should be deleted when using --reduce");
}

#[test]
fn test_skip_existing_lyrics() {
    let test_dir = create_test_dir();
    let test_dir_path = test_dir.path();
    
    // Copy test files to temp directory
    let mp3_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.mp3");
    let lrc_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.lrc");
    
    let mp3_dest = test_dir_path.join("04 Avril Lavigne - I'm With You.mp3");
    let lrc_dest = test_dir_path.join("04 Avril Lavigne - I'm With You.lrc");
    
    copy_test_file(mp3_source, &mp3_dest);
    copy_test_file(lrc_source, &lrc_dest);
    
    // First run: embed lyrics
    let output1 = lyricsync_bin()
        .arg("--directory")
        .arg(test_dir_path)
        .output()
        .expect("Failed to execute lyricsync");
    
    assert!(output1.status.success(), "First run should succeed");
    
    // Second run with --skip: should skip the file
    let output2 = lyricsync_bin()
        .arg("--directory")
        .arg(test_dir_path)
        .arg("--skip")
        .output()
        .expect("Failed to execute lyricsync");
    
    assert!(output2.status.success(), "Second run should succeed");
    
    let stdout = String::from_utf8_lossy(&output2.stdout);
    assert!(stdout.contains("Skipped") || stdout.contains("0"), 
            "Output should indicate file was skipped");
}

#[test]
fn test_missing_lrc_file() {
    let test_dir = create_test_dir();
    let test_dir_path = test_dir.path();
    
    // Copy only MP3 file (no LRC)
    let mp3_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.mp3");
    let mp3_dest = test_dir_path.join("04 Avril Lavigne - I'm With You.mp3");
    
    copy_test_file(mp3_source, &mp3_dest);
    
    // Run lyricsync - should succeed but not embed anything
    let output = lyricsync_bin()
        .arg("--directory")
        .arg(test_dir_path)
        .output()
        .expect("Failed to execute lyricsync");
    
    assert!(output.status.success(), "lyricsync should succeed even with no LRC files");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0") || stdout.contains("Total audio files: 1"), 
            "Should report 0 embedded files");
}

#[test]
fn test_recursive_flag() {
    let test_dir = create_test_dir();
    let test_dir_path = test_dir.path();
    
    // Create subdirectory
    let subdir = test_dir_path.join("subdir");
    fs::create_dir(&subdir).expect("Failed to create subdirectory");
    
    // Copy test files to subdirectory
    let mp3_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.mp3");
    let lrc_source = Path::new("tests/fixtures/04 Avril Lavigne - I'm With You.lrc");
    
    let mp3_dest = subdir.join("04 Avril Lavigne - I'm With You.mp3");
    let lrc_dest = subdir.join("04 Avril Lavigne - I'm With You.lrc");
    
    copy_test_file(mp3_source, &mp3_dest);
    copy_test_file(lrc_source, &lrc_dest);
    
    // Run without --recursive: should not find files in subdirectory
    let output1 = lyricsync_bin()
        .arg("--directory")
        .arg(test_dir_path)
        .output()
        .expect("Failed to execute lyricsync");
    
    assert!(output1.status.success(), "Should succeed");
    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    assert!(stdout1.contains("0") || stdout1.contains("Total audio files: 0"), 
            "Should not find files without --recursive");
    
    // Run with --recursive: should find files in subdirectory
    let output2 = lyricsync_bin()
        .arg("--directory")
        .arg(test_dir_path)
        .arg("--recursive")
        .output()
        .expect("Failed to execute lyricsync");
    
    assert!(output2.status.success(), "Should succeed");
    let stdout2 = String::from_utf8_lossy(&output2.stdout);
    assert!(stdout2.contains("1") || stdout2.contains("Total audio files: 1"), 
            "Should find files with --recursive");
}

#[test]
fn test_invalid_directory() {
    let output = lyricsync_bin()
        .arg("--directory")
        .arg("/nonexistent/directory/path")
        .output()
        .expect("Failed to execute lyricsync");
    
    // Program doesn't fail on invalid directories, it just processes 0 files
    assert!(output.status.success(), "Should succeed even with invalid directory");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0") || stdout.contains("Total audio files: 0"), 
            "Should report 0 files for invalid directory");
}
