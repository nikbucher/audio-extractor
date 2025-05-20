# Audio Extractor

A simple Tauri app to extract audio from video files using FFmpeg. This is a personal try-out project I created to learn
Rust and Tauri development.

## About This Project

This project serves as my learning playground for:

- Rust programming language fundamentals
- Tauri framework for building cross-platform desktop applications
- Integrating external tools (FFmpeg) with Rust
- Cross-platform development for macOS, Windows and Linux.

## Features

- Extract audio from video files using FFmpeg
- Support for different audio formats (AAC, MP3, OGG)
- Extract entire audio track or specific time ranges
- Simple and intuitive user interface

## Installation

The easiest way to install Audio Extractor is to download the pre-built binaries from
the [Releases page](https://github.com/nikbucher/audio-extractor/releases).

**Note:** This application requires FFmpeg to be installed on your system. See
the [FFmpeg Installation](#ffmpeg-installation) section below.

1. Go to the [Releases page](https://github.com/nikbucher/audio-extractor/releases)
2. Download the appropriate version for your operating system:
   - **Windows**: `.msi` installer or `.exe` executable
   - **macOS**: `.dmg` disk image (available for both Intel and Apple Silicon)
   - **Linux**: `.AppImage`, `.deb`, or `.tar.gz` archive

## FFmpeg Installation

Audio Extractor requires FFmpeg to be installed on your system:

### macOS

Install via [Homebrew](https://brew.sh/):

```sh
brew install ffmpeg
```

### Windows

1. Download FFmpeg from [https://ffmpeg.org/download.html](https://ffmpeg.org/download.html)
2. Extract the ZIP file and move the folder to a location like `C:\ffmpeg`
3. Add FFmpeg to your system PATH:
   - Open Start Menu and search for "Environment Variables"
   - Click «Environment Variables» and find `Path` under System variables
   - Add the path to the `bin` folder (e.g., `C:\ffmpeg\bin`)
   - Click OK and restart any Command Prompt windows
4. Verify by typing `ffmpeg -version` in Command Prompt

### Linux

Install via your package manager:

```sh
# Ubuntu/Debian
sudo apt update
sudo apt install ffmpeg

# Fedora
sudo dnf install ffmpeg

# Arch Linux
sudo pacman -S ffmpeg
```

Verify installation by running `ffmpeg -version` and `ffprobe -version` in your terminal or command prompt.

**Note:** FFmpeg is licensed under the LGPL/GPL. When using this application, you are responsible for complying
with FFmpeg's license terms. See [FFmpeg Legal](https://ffmpeg.org/legal.html) for more information.

## Development

### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Node.js](https://nodejs.org/) (LTS version recommended)
- Install FFmpeg (see [FFmpeg Installation](#ffmpeg-installation) section)

### Getting Started

1. **Clone the repository:**
   ```sh
   git clone https://github.com/nikbucher/audio-extractor.git
   cd audio-extractor
   ```

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

### Running in Development Mode

```sh
cargo tauri dev
```

## Building for Production

1. **Build the App:**
   ```sh
   cargo tauri build
   ```

2. **Distribute the App:**
   - The app will be built in the `src-tauri/target/release` directory.
   - For bundled applications (installers, DMG, etc.), check the `src-tauri/target/release/bundle` directory.

## Learning Resources

If you're also interested in learning Rust and Tauri, here are some resources I found helpful:

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Tauri Documentation](https://tauri.app/start/)
- [FFmpeg Documentation](https://ffmpeg.org/documentation.html)
