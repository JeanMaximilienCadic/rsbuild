# rsbuild

[![Crates.io](https://img.shields.io/crates/v/rsbuild.svg)](https://crates.io/crates/rsbuild)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A self-sufficient runtime to build projects.

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
rsbuild <COMMAND>
```

### Commands

| Command | Description |
|---------|-------------|
| `build` | Build artifacts (wheel, docker, cargo) |
| `pull` | Pull docker images |
| `clean` | Clean build artifacts |
| `cython` | Compile Cython modules |
| `glances` | Run glances system monitor |
| `help` | Print help information |

### Build Subcommands

```bash
# Build Python wheel
rsbuild build wheel

# Build all targets (wheel, vanilla, sandbox)
rsbuild build all

# Build Docker containers
rsbuild build vanilla
rsbuild build sandbox
rsbuild build docker <service>

# Build Rust binary
rsbuild build cargo debug
rsbuild build cargo release
```

### Pull Subcommands

```bash
# Pull all images
rsbuild pull all

# Pull specific images
rsbuild pull vanilla
rsbuild pull sandbox
```

### Clean

```bash
# Remove build artifacts, egg-info, pycache, and notebook checkpoints
rsbuild clean
```

### Cython

```bash
# Compile Cython modules for a package
rsbuild cython <package>
```

## Project Structure

```
src/
├── main.rs          # Entry point
├── cli.rs           # CLI argument definitions (clap)
├── error.rs         # Error types (thiserror)
├── executor.rs      # Command execution utilities
└── commands/
    ├── mod.rs       # Module exports
    ├── build.rs     # Build commands
    ├── clean.rs     # Clean command
    ├── cython.rs    # Cython compilation
    └── pull.rs      # Docker pull commands
```

## License

MIT
