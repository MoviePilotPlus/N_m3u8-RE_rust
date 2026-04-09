# N_m3u8-RE_rust

[查看中文版](README.md)

Cross-platform DASH/HLS/MSS download tool, reimplemented in Rust. Supports on-demand and live streaming (DASH/HLS).

[![img](https://img.shields.io/github/stars/MoviePilotPlus/N_m3u8-RE_rust?label=Stars)](https://github.com/MoviePilotPlus/N_m3u8-RE_rust)  [![img](https://img.shields.io/github/last-commit/MoviePilotPlus/N_m3u8-RE_rust?label=Last%20Commit)](https://github.com/MoviePilotPlus/N_m3u8-RE_rust)  [![img](https://img.shields.io/github/license/MoviePilotPlus/N_m3u8-RE_rust?label=License)](https://github.com/MoviePilotPlus/N_m3u8-RE_rust)

---

## Features

- Fully compatible with the command line parameters of the original N_m3u8DL-RE project
- High-performance Rust implementation
- Supports DASH, HLS, and MSS protocols
- On-demand and live download
- Cross-platform support: Windows, macOS, Linux

## Build and Install

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager

### Build

```bash
git clone https://github.com/MoviePilotPlus/N_m3u8-RE_rust.git
cd N_m3u8-RE_rust
cargo build --release
```

After building, the executable is located at `target/release/N_m3u8DL-RE`.

## Usage

Basic usage is the same as the original project:

```bash
# Download on-demand video
./N_m3u8DL-RE <input_url>

# View help
./N_m3u8DL-RE --help
```

## Project Structure

```
N_m3u8-RE_rust/
├── src/
│   ├── commandline/    # Command line argument parsing
│   ├── crypto/         # Encryption and decryption
│   ├── downloader/     # Download management
│   ├── entity/         # Data structures
│   ├── i18n/           # Internationalization
│   ├── muxer/          # Media muxing
│   ├── parser/         # Media stream parsing
│   ├── utils/          # Utility functions
│   └── main.rs         # Main entry point
├── tests/              # Test cases
├── Cargo.toml          # Project configuration
└── README.md           # Project documentation
```

## Acknowledgments

- Original project: [nilaoda/N_m3u8DL-RE](https://github.com/nilaoda/N_m3u8DL-RE)
