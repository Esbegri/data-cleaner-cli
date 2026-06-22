# data-cleaner-cli

A fast, configurable CLI tool for cleaning and validating emails, phone numbers, and URLs — built in Rust.

## Features

- Validate emails, phone numbers, or URLs via `--mode` flag
- Process multiple input files at once
- Remove duplicates automatically
- Detailed summary report with timestamps
- Logs to both terminal and file (`app.log`)
- `--dry-run` and `--stats-only` modes for safe inspection

## Installation

```bash
git clone https://github.com/Esbegri/data-cleaner-cli
cd data-cleaner-cli
cargo build --release
```

## Usage

### Basic
```bash
./target/release/email-cleaner-cli --input emails.txt --output clean.txt
```

### Multiple input files
```bash
./target/release/email-cleaner-cli --input file1.txt --input file2.txt --output clean.txt
```

### Phone number validation
```bash
./target/release/email-cleaner-cli --input phones.txt --output clean.txt --mode phone
```

### URL validation
```bash
./target/release/email-cleaner-cli --input urls.txt --output clean.txt --mode url
```

### Stats only (no output written)
```bash
./target/release/email-cleaner-cli --input emails.txt --output clean.txt --stats-only
```

### Dry run
```bash
./
