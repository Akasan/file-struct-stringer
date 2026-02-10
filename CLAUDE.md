# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`file-struct-stringer` is a Rust CLI tool that converts folder structures into readable text format, useful for displaying project organization in technical blogs and documentation.

**Core Requirements** (from README.md):
- Display folder structures in a clean, hierarchical text format
- Option to list only folders
- Option to filter by specific file formats (multiple formats supported)
- Configurable indent size (default: 2 spaces)

## Development Commands

### Initial Setup
```bash
# Initialize Rust project (if not done)
cargo init --name file-struct-stringer

# Build the project
cargo build

# Build optimized release binary
cargo build --release
```

### Development Workflow
```bash
# Run the CLI tool
cargo run -- [args]

# Run with arguments example
cargo run -- --folders-only
cargo run -- --format rs,toml --indent 4

# Run tests
cargo test

# Run specific test
cargo test test_name

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Installation
```bash
# Install locally
cargo install --path .
```

## Architecture Guidance

### Expected Structure
- **CLI parsing**: Use `clap` crate for argument parsing with options for:
  - `--folders-only` or `-f`: List only directories
  - `--format <EXTENSIONS>` or `-e`: Filter by file extensions (comma-separated)
  - `--indent <SIZE>` or `-i`: Set indentation size (default: 2)

- **Directory traversal**: Recursively walk directory tree, respecting filters
- **Output formatting**: Generate tree-like text representation with proper indentation
- **Error handling**: Handle permission errors, symlinks, and invalid paths gracefully

### Implementation Notes
- Use `std::fs` or `walkdir` crate for directory traversal
- Consider `.gitignore` patterns to exclude common non-essential directories (e.g., `node_modules`, `target`, `.git`)
- Output should be UTF-8 compatible with tree-drawing characters (├──, └──, │) for visual clarity
