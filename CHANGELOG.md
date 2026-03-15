# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-03-16

### Added

- Proper CLI argument parsing with `clap`
- Modular code structure with separate command modules
- Error handling with `thiserror` and `anyhow`
- GitHub Actions workflow for automated releases
- GitHub Actions workflow for automated publishing to crates.io
- Multi-platform binary releases (Linux x86_64/ARM64, macOS x86_64/ARM64)

### Changed

- Refactored from single-file to modular architecture
- Improved command structure with proper subcommands
- Updated dependencies to latest versions

### Removed

- Legacy symlinks to local build artifacts

## [0.1.10] - 2023-03-20

### Added

- Initial release
- Basic build, clean, pull, and cython commands
- Colored console output

[Unreleased]: https://github.com/JeanMaximilienCadic/rsbuild/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/JeanMaximilienCadic/rsbuild/compare/v0.1.10...v0.2.0
[0.1.10]: https://github.com/JeanMaximilienCadic/rsbuild/releases/tag/v0.1.10
