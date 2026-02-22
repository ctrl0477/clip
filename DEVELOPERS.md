clip/DEVELOPERS.md
```

# Developer Documentation

This document provides information for developers who want to contribute to or extend the Clip project.

## Project Structure

```
clip/
├── src/
│   ├── main.rs          # Entry point and CLI argument handling
│   ├── lib.rs           # Module declarations
│   ├── clipboard/
│   │   └── mod.rs       # Clipboard operations (pbcopy)
│   └── file/
│       └── mod.rs       # File reading operations
├── Cargo.toml           # Project manifest
└── Cargo.lock           # Dependency lock file
```

## Building the Project

### Debug Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

The binary will be located at `target/release/clip`.

### Running

```bash
cargo run -- <filename
```

## Code Overview

### Main Entry Point (`src/main.rs`)

The `main` function handles:
- Command-line argument parsing
- File reading via the `file` module
- Clipboard operations via the `clipboard` module
- Error handling and user feedback

### File Module (`src/file/mod.rs`)

Provides a single function:
- `read_file(path)` - Reads a file's contents and returns them as a String

### Clipboard Module (`src/clipboard/mod.rs`)

Provides a single function:
- `copy_to_clipboard(contents)` - Copies text to the system clipboard using `pbcopy`

## Adding New Features

### Adding a New Module

1. Create a new directory under `src/`
2. Add a `mod.rs` file with your functionality
3. Declare the module in `src/lib.rs`
4. Import and use it in `src/main.rs`

### Example: Adding a Verbose Flag

To add verbose output:

1. Add to `src/main.rs`:

```rust
let verbose = args.iter().any(|arg| arg == "-v" || arg == "--verbose");
```

2. Use the flag to control debug output

## Testing

Run tests with:

```bash
cargo test
```

## Code Style

- Follow standard Rust conventions
- Use `rustfmt` for code formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`

## Dependencies

This project has no external dependencies. It uses only:
- Rust standard library
- macOS `pbcopy` command (via `std::process::Command`)

## Platform Notes

This tool currently relies on macOS-specific commands:
- `pbcopy` - For clipboard operations

To add Linux/Windows support, you would need to modify `src/clipboard/mod.rs` to use platform-specific clipboard libraries.
