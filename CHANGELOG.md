clip/CHANGELOG.md
```

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026

### Added

- Initial release
- File reading functionality via `file::read_file()`
- Clipboard copy functionality via `clipboard::copy_to_clipboard()`
- Command-line interface that accepts a filename argument
- Uses macOS `pbcopy` command for clipboard operations

### Project Structure

- Modular design with separate `clipboard` and `file` modules
- Clean error handling with descriptive error messages
- No external dependencies (uses only Rust standard library)
