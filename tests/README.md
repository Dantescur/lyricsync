# Test Suite

This directory contains integration tests for lyricsync.

## Structure

- `integration_test.rs` - Integration tests that test the CLI interface
- `fixtures/` - Test audio and LRC files used by the tests

## Running Tests

Run all tests:
```bash
cargo test
```

Run only integration tests:
```bash
cargo test --test integration_test
```

Run a specific test:
```bash
cargo test test_embed_mp3_lyrics
```

## Test Coverage

The test suite covers:
- Basic MP3 lyrics embedding
- Dry-run mode
- Reduce flag (LRC file deletion)
- Skip existing lyrics
- Missing LRC files
- Recursive directory processing
- Error handling (invalid directories)

## Test Files

Test files are located in `tests/fixtures/`:
- `04 Avril Lavigne - I'm With You.mp3` - Sample MP3 file
- `04 Avril Lavigne - I'm With You.lrc` - Corresponding LRC file

These files are copied to temporary directories during test execution, so the originals are never modified.
