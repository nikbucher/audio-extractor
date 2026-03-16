# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1] - 2026-03-16

### Fixed

- Native drag-and-drop now loads video files directly instead of falling back to file dialog
- Enhanced drop zone visual feedback with accent color highlighting during drag-over

## [0.4.0] - 2026-03-16

### Added

- Windows ARM (aarch64) build target and Linux RPM packaging
- Screenshot and CI badge in README

### Changed

- Renamed app from "Audio Extractor" to "AudioSnip"
- Reworked UI with waveform timeline and draggable range handles
- New custom app icons
- Introduced `AppError` enum for unified error handling
- Cached FFmpeg/FFprobe command paths
- Streamed waveform data instead of loading all at once

### Fixed

- Restricted media server to registered file paths for security
- Used fast FFmpeg seeking (`-ss` before `-i`) for better performance
- Improved atomic operations and deduplicated constants

## [0.3.0] - 2026-02-10

### Added

- Async audio extraction with progress reporting and cancellation support

## [0.2.0] - 2026-02-09

### Added

- Input validation and improved UX
- Local media server for in-app video preview with audio support
- Deduplicated codec logic
- Synced release artifact versions from git tag

## [0.1.0] - 2025-05-20

### Added

- Initial release
- Extract audio from video files using FFmpeg
- Support for AAC, MP3, and OGG output formats
- Time range selection for partial extraction
- Cross-platform builds for macOS, Windows, and Linux

[Unreleased]: https://github.com/nikbucher/audio-snip/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/nikbucher/audio-snip/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/nikbucher/audio-snip/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/nikbucher/audio-snip/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/nikbucher/audio-snip/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/nikbucher/audio-snip/releases/tag/v0.1.0
