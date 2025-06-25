# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based log file analyzer that extracts and displays error messages from log files.

## Build and Development Commands

### Building
- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build the project in release mode

### Running
- `cargo run` - Run the application (reads from logs.txt)

### Development
- `cargo check` - Check code for errors without building
- `cargo clippy` - Run Rust linter for code quality checks
- `cargo fmt` - Format code according to Rust style guidelines

### Testing
- `cargo test` - Run tests (Note: No tests currently exist in the project)

## Code Architecture

The application consists of a single `main.rs` file with the following key functions:

1. **`main()`** - Entry point that reads logs.txt and processes errors
2. **`extract_errors(text: &str) -> Vec<String>`** - Extracts lines starting with "ERROR" from log text
3. **`valdidate_email(email: &str) -> Result<(), Error>`** - Email validation (Note: typo in function name)
4. **`divide(a: f64, b: f64) -> Result<f64, Error>`** - Safe division with zero-check

## Important Notes

- The application expects a `logs.txt` file in the root directory
- Error messages are identified by lines starting with "ERROR"
- Uses Rust 2024 edition
- No external dependencies