# Clip

A simple command-line utility to copy file contents to the system clipboard.

## Overview

Clip is a lightweight CLI tool that reads the contents of a file and copies them to your system clipboard. It uses macOS's built-in `pbcopy` command for clipboard operations.

## Requirements

- Rust (latest stable version)
- macOS (for `pbcopy` command)

## Installation

```bash
cargo install clip
```

Or build from source:

```bash
git clone <repository-url>
cd clip
cargo build --release
```

## Usage

```bash
clip 
```

### Examples

Copy the contents of a text file to the clipboard:

```bash
clip myfile.txt
```

Copy a configuration file:

```bash
clip config.json
```

## How It Works

1. Reads the specified file from disk
2. Passes the file contents to macOS's `pbcopy` command via stdin
3. The contents are now available in your clipboard

## License

MIT License

---

**Note**: This is a personal tool written with AI assistance. While functional, it should not be considered production-ready software. Use at your own risk.
