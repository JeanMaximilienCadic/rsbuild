# rsbuild

[![Crates.io](https://img.shields.io/crates/v/rsbuild.svg)](https://crates.io/crates/rsbuild)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A self-sufficient runtime to build projects.

rsbuild provides commands for building Python wheels, Docker containers, Rust binaries, and managing Python project scaffolding.

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
| `-y, --yes` | Skip confirmation prompts |
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
| `python` | Python project management |
| `glances` | Run glances system monitor |
| `completions` | Generate shell completion scripts |
| `doctor` | Check if required tools are installed |

## Python Project Management

### Initialize a New Python Project

```bash
# Initialize in current directory (uses directory name as package name)
rsbuild python init

# Specify a custom package name
rsbuild python init --name mypackage

# Skip tests or devcontainer
rsbuild python init --no-tests --no-devcontainer

# Preview what would be created
rsbuild --dry-run python init
```

This creates a complete Python project with:

- `pyproject.toml` - Modern Python packaging with hatchling
- `README.md` - Project documentation
- `.pre-commit-config.yaml` - Pre-commit hooks (ruff, mypy)
- `.gitignore` - Python-specific ignores
- `Taskfile.yml` - Task runner commands (works on macOS and Linux)
- `<package>/__init__.py` - Package with `__version__` and `__build__`
- `<package>/tests/` - Test directory with sample test
- `.devcontainer/` - VS Code devcontainer configuration
- `Dockerfile` & `docker-compose.yml` - Container setup

### Sync Version

```bash
# Update __version__ and __build__ in __init__.py from pyproject.toml
rsbuild python sync-version
```

### Taskfile Commands

After initializing, use these task commands:

```bash
task install      # Install dependencies with uv
task build        # Build wheel (syncs version first)
task test         # Run tests
task lint         # Run linter
task format       # Format code
task typecheck    # Run mypy
task clean        # Clean build artifacts
task sync-version # Sync version from pyproject.toml
task docker-build # Build Docker image
task ci           # Run all CI checks
```

## Build Commands

```bash
# Build Python wheel using uv
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

## Pull Commands

```bash
# Pull all configured images
rsbuild pull all

# Pull a specific service image
rsbuild pull service <name>
```

## Run Docker Services

```bash
# Run a Docker Compose service
rsbuild run <service>

# Run with additional arguments
rsbuild run <service> -- --env FOO=bar
```

## Clean

```bash
# Remove build artifacts, egg-info, pycache, and notebook checkpoints
rsbuild clean

# Also remove Rust target directory
rsbuild clean --all
```

## Doctor

```bash
# Check system for required tools
rsbuild doctor
```

## Shell Completions

```bash
# Generate completions for your shell
rsbuild completions bash > ~/.local/share/bash-completion/completions/rsbuild
rsbuild completions zsh > ~/.zfunc/_rsbuild
rsbuild completions fish > ~/.config/fish/completions/rsbuild.fish
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
    ├── python.rs     # Python project management
    └── run.rs        # Docker run command
```

## License

MIT
