# rsbuild

[![Crates.io](https://img.shields.io/crates/v/rsbuild.svg)](https://crates.io/crates/rsbuild)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A self-sufficient runtime to build projects.

rsbuild provides commands for building Python wheels, Docker containers, Rust binaries, and managing Cython compilation workflows.

## Installation

### From crates.io

```bash
cargo install rsbuild
```

### From source

```bash
git clone https://github.com/JeanMaximilienCadic/rsbuild
cd rsbuild
cargo install --path .
```

## Usage

```bash
rsbuild [OPTIONS] <COMMAND>
```

### Global Options

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Increase output verbosity |
| `-q, --quiet` | Suppress non-essential output |
| `--dry-run` | Preview commands without executing them |
| `-h, --help` | Print help information |
| `-V, --version` | Print version |

### Commands

| Command | Description |
|---------|-------------|
| `build` | Build artifacts (wheel, docker, cargo) |
| `pull` | Pull Docker images |
| `run` | Run Docker Compose services |
| `clean` | Clean build artifacts and caches |
| `cython` | Compile Cython modules and package into wheel |
| `glances` | Run glances system monitor |
| `completions` | Generate shell completion scripts |
| `doctor` | Check if required tools are installed |

### Build Subcommands

```bash
# Build Python wheel
rsbuild build wheel

# Build all configured targets
rsbuild build all

# Build Rust binary (release mode by default)
rsbuild build cargo
rsbuild build cargo release
rsbuild build cargo debug

# Build Docker Compose service
rsbuild build docker <service>
rsbuild build docker <service> --no-cache
```

### Pull Subcommands

```bash
# Pull all configured images
rsbuild pull all

# Pull a specific service image
rsbuild pull service <name>
```

### Run Docker Services

```bash
# Run a Docker Compose service
rsbuild run <service>

# Run with additional arguments
rsbuild run <service> -- --env FOO=bar
```

### Clean

```bash
# Remove build artifacts, egg-info, pycache, and notebook checkpoints
rsbuild clean

# Also remove Rust target directory
rsbuild clean --all
```

### Cython

```bash
# Compile Cython modules for a package
rsbuild cython <package>
```

### Doctor

```bash
# Check system for required tools
rsbuild doctor
```

### Shell Completions

```bash
# Generate completions for your shell
rsbuild completions bash > ~/.local/share/bash-completion/completions/rsbuild
rsbuild completions zsh > ~/.zfunc/_rsbuild
rsbuild completions fish > ~/.config/fish/completions/rsbuild.fish
```

### Dry Run Mode

Preview what commands would be executed without running them:

```bash
rsbuild --dry-run build wheel
rsbuild --dry-run clean --all
```

## Project Structure

```
src/
├── main.rs           # Entry point
├── cli.rs            # CLI definitions (clap)
├── error.rs          # Error types (thiserror)
├── executor.rs       # Command execution utilities
└── commands/
    ├── mod.rs        # Module exports
    ├── build.rs      # Build commands
    ├── clean.rs      # Clean command
    ├── cython.rs     # Cython compilation
    ├── doctor.rs     # System diagnostics
    ├── pull.rs       # Docker pull commands
    └── run.rs        # Docker run command
```

## License

MIT
