# Audio Extractor

A simple Tauri app to extract audio from video files using FFmpeg. This is a personal try-out project I created to learn
Rust and Tauri development.

## About This Project

This project serves as my learning playground for:

- Rust programming language fundamentals
- Tauri framework for building cross-platform desktop applications
- Integrating external tools (FFmpeg) with Rust
- Cross-platform development for Windows and macOS

## Features

- Extract audio from video files using FFmpeg
- Support for different audio formats (AAC, MP3, OGG)
- Extract entire audio track or specific time ranges
- Simple and intuitive user interface

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Getting Started (Windows & macOS)

1. **Install FFmpeg and FFprobe:**
   - **macOS:** Install via [Homebrew](https://brew.sh/):
     ```sh
     brew install ffmpeg
     ```
   - **Windows:** Download from [FFmpeg.org](https://ffmpeg.org/download.html), extract, and add the `bin` folder (containing `ffmpeg.exe` and `ffprobe.exe`) to your system `PATH`.
   - Verify installation by running `ffmpeg -version` and `ffprobe -version` in your terminal or command prompt.
   - **Note:** FFmpeg is licensed under the LGPL/GPL. When using this application, you are responsible for complying
     with FFmpeg's license terms. See [FFmpeg Legal](https://ffmpeg.org/legal.html) for more information.

2. **Run the App:**
   - Using Cargo:
     ```sh
     cargo tauri dev
     ```

## Building for Production

1. **Build the App:**
   - Using Cargo:
     ```sh
     cargo tauri build
     ```

2. **Distribute the App:**
   - The app will be built in the `src-tauri/target/release` directory.
   - For bundled applications (installers, DMG, etc.), check the `src-tauri/target/release/bundle` directory.
   - You can distribute the app as a standalone executable or bundle it with an installer.

## Learning Resources

If you're also interested in learning Rust and Tauri, here are some resources I found helpful:

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Tauri Documentation](https://tauri.app/v1/guides/)
- [FFmpeg Documentation](https://ffmpeg.org/documentation.html)
