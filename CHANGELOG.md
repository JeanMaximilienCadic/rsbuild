# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2026-03-16

### Added

- `rsbuild python init` command to scaffold Python projects with:
  - pyproject.toml (hatchling build system)
  - Pre-commit configuration (ruff, mypy)
  - Taskfile.yml with common tasks (works on macOS and Linux)
  - Package structure with `__version__` and `__build__`
  - Tests directory with sample test
  - Devcontainer configuration
  - Dockerfile and docker-compose.yml
- `rsbuild python sync-version` to sync version from pyproject.toml to package
- `-y, --yes` global flag to skip confirmation prompts
- Confirmation prompts before overwriting existing files
- `uv` and `task` to doctor command checks

### Changed

- `rsbuild build wheel` now uses `uv build --wheel` instead of pip
- Improved doctor command output formatting

## [0.3.0] - 2026-03-16

### Added

- `--dry-run` global flag to preview commands without executing
- `--verbose` and `--quiet` global flags for output control
- `doctor` command to check system for required tools
- `completions` command to generate shell completions (bash, zsh, fish, etc.)
- `run` command to execute Docker Compose services
- `--no-cache` flag for docker builds
- `--all` flag for clean command to also remove Rust target
- Tool existence checks with helpful installation hints
- Documentation comments throughout codebase
- `which` crate for reliable tool detection

### Changed

- Removed PII (author name) from Cargo.toml and CLI metadata
- Improved error messages with command, exit code, and error details
- Refactored executor module with better output handling
- Build commands now show status messages
- Docker service names are now configurable (not hardcoded)
- Pull command now uses `pull service <name>` for specific services
- Cargo build defaults to release mode
- Cython command validates package directory exists

### Fixed

- Clean command patterns for better artifact removal

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

[Unreleased]: https://github.com/JeanMaximilienCadic/rsbuild/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/JeanMaximilienCadic/rsbuild/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/JeanMaximilienCadic/rsbuild/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/JeanMaximilienCadic/rsbuild/compare/v0.1.10...v0.2.0
[0.1.10]: https://github.com/JeanMaximilienCadic/rsbuild/releases/tag/v0.1.10
