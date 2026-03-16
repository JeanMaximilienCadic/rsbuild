# rsbuild

[![Crates.io](https://img.shields.io/crates/v/rsbuild.svg)](https://crates.io/crates/rsbuild)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A self-sufficient runtime to build projects.

## What is rsbuild?

**rsbuild** is a unified command-line tool that streamlines the build process for polyglot projects. It eliminates the need to remember different build commands for different ecosystems by providing a consistent interface for:

- **Python wheels** - Build distributable packages using `uv`
- **Docker containers** - Build and manage Docker Compose services
- **Rust binaries** - Compile Rust projects with cargo
- **Cython modules** - Compile `.pyx` files and package them into wheels
- **Project scaffolding** - Initialize new Python projects with best practices

### Why rsbuild?

Modern projects often combine multiple technologies. A typical project might have:
- Python code for the application logic
- Cython for performance-critical components
- Docker for deployment
- Rust for CLI tools or native extensions

Without rsbuild, you'd need to remember:
```bash
uv build --wheel                    # Python
cythonize -a -i mypackage           # Cython
docker compose build vanilla        # Docker
cargo build --release               # Rust
```

With rsbuild, it's simply:
```bash
rsbuild build all
```

### Key Features

| Feature | Description |
|---------|-------------|
| **Unified Interface** | One command for all build tasks |
| **Watch Mode** | Automatically rebuild on file changes |
| **Parallel Execution** | Build multiple targets simultaneously |
| **JSON Output** | Machine-readable output for CI/CD integration |
| **Progress Indicators** | Visual feedback during long operations |
| **Dry Run** | Preview commands without executing them |
| **Project Scaffolding** | Initialize Python projects with modern best practices |

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

### Verify installation

```bash
rsbuild doctor
```

This checks that all required and optional tools are available on your system.

## Quick Start

```bash
# Initialize a new Python project
rsbuild python init --name myproject

# Build everything
rsbuild build all

# Watch for changes and rebuild
rsbuild watch wheel

# Check system dependencies
rsbuild doctor --json
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
| `--json` | Output results in JSON format (implies `--quiet`) |
| `-j, --jobs <N>` | Number of parallel jobs (default: 1) |
| `-h, --help` | Print help information |
| `-V, --version` | Print version |

### Commands Overview

| Command | Description |
|---------|-------------|
| `build` | Build artifacts (wheel, docker, cargo) |
| `pull` | Pull Docker images |
| `run` | Run Docker Compose services |
| `clean` | Clean build artifacts and caches |
| `cython` | Compile Cython modules and package into wheel |
| `python` | Python project management |
| `watch` | Watch files and rebuild on changes |
| `glances` | Run glances system monitor |
| `completions` | Generate shell completion scripts |
| `doctor` | Check if required tools are installed |

## Build Commands

The `build` command is the core of rsbuild, providing a unified interface for building different types of artifacts.

### Build Python Wheel

```bash
rsbuild build wheel
```

This command:
1. Creates a `dist/legacy` directory and moves existing wheels there
2. Builds a new wheel using `uv build --wheel`
3. Cleans up build artifacts
4. Places the wheel in `dist/`

**Requirements:** `uv` must be installed.

### Build Rust Binary

```bash
# Release build (default, optimized)
rsbuild build cargo
rsbuild build cargo release

# Debug build (faster compilation)
rsbuild build cargo debug
```

The binary is placed in `target/<OS>/<ARCH>/release/` or `target/<OS>/<ARCH>/debug/`.

**Requirements:** `cargo` must be installed.

### Build Docker Service

```bash
# Build a specific service
rsbuild build docker myservice

# Build without cache
rsbuild build docker myservice --no-cache
```

This wraps `docker compose build` with consistent output formatting.

**Requirements:** `docker` must be installed.

### Build All Targets

```bash
# Build everything sequentially
rsbuild build all

# Build in parallel (4 jobs)
rsbuild build all -j4

# Get JSON output
rsbuild build all --json
```

This builds:
1. Python wheel (if `uv` is available)
2. Docker services `vanilla` and `sandbox` (if `docker-compose.yml` exists)

When using `-j` for parallel execution, builds run concurrently with a progress bar showing completion status.

## Watch Mode

Watch mode monitors your files and automatically triggers rebuilds when changes are detected.

```bash
# Watch Python files and rebuild wheel
rsbuild watch wheel

# Watch Rust files and rebuild binary
rsbuild watch cargo

# Watch Dockerfile and rebuild container
rsbuild watch docker

# Watch Cython files and recompile
rsbuild watch cython
```

### Watch Options

| Option | Description |
|--------|-------------|
| `-d, --debounce <MS>` | Debounce delay in milliseconds (default: 500) |
| `-p, --path <PATH>` | Additional paths to watch (can be repeated) |

### File Patterns

Each watch target monitors specific file patterns:

| Target | Patterns |
|--------|----------|
| `wheel` | `*.py`, `pyproject.toml`, `setup.py`, `setup.cfg` |
| `cargo` | `*.rs`, `Cargo.toml`, `Cargo.lock` |
| `docker` | `Dockerfile`, `docker-compose.yml`, `compose.yml`, `*.dockerfile` |
| `cython` | `*.pyx`, `*.pxd`, `*.pxi` |

### Example

```bash
# Watch with custom debounce and additional paths
rsbuild watch wheel -d 1000 -p src/ -p lib/

# Press Ctrl+C to stop watching
```

## Parallel Execution

Many commands support parallel execution using the `-j` flag:

```bash
# Build all targets with 4 parallel jobs
rsbuild build all -j4

# Pull all images in parallel
rsbuild pull all -j4
```

Parallel execution:
- Uses a thread pool sized to the specified job count
- Shows a progress bar with completion status
- Collects all results (doesn't fail fast)
- Falls back to sequential execution when `-j1`

## JSON Output

The `--json` flag outputs machine-readable JSON, useful for CI/CD pipelines and scripting:

```bash
# Check system status as JSON
rsbuild doctor --json | jq .

# Build and capture results
rsbuild build all --json > build-results.json
```

### Doctor JSON Output

```json
{
  "tools": [
    {"name": "cargo", "purpose": "Rust builds", "installed": true, "required": true},
    {"name": "docker", "purpose": "Docker builds", "installed": true, "required": true},
    {"name": "uv", "purpose": "Python package management", "installed": true, "required": false}
  ],
  "files": [
    {"name": "pyproject.toml", "found": true},
    {"name": "Cargo.toml", "found": true}
  ],
  "all_required_installed": true
}
```

### Build JSON Output

```json
{
  "total": 3,
  "succeeded": 2,
  "failed": 1,
  "skipped": 0,
  "duration_ms": 5432,
  "results": [
    {"command": "wheel", "status": "success", "duration_ms": 0, "output": "Wheel built"},
    {"command": "docker:vanilla", "status": "success", "duration_ms": 0},
    {"command": "docker:sandbox", "status": "failed", "duration_ms": 0, "error": "Service not found"}
  ]
}
```

## Pull Commands

Pull Docker images defined in your compose file:

```bash
# Pull all configured images
rsbuild pull all

# Pull in parallel
rsbuild pull all -j4

# Pull a specific service image
rsbuild pull service myservice
```

## Run Docker Services

Run Docker Compose services with a simplified interface:

```bash
# Run a service
rsbuild run myservice

# Run with additional arguments
rsbuild run myservice -- --env DEBUG=1 --rm
```

## Cython Compilation

Compile Cython modules and package them into distributable wheels:

```bash
rsbuild cython mypackage
```

This command:
1. Validates the package directory exists
2. Creates a temporary build directory
3. Compiles `.pyx` files to `.so` shared objects using `cythonize`
4. Cleans up intermediate `.c` files
5. Packages the compiled modules into a wheel
6. Moves the wheel to `dist/`

**Requirements:** `cythonize`, `rsync`, and `pip` must be installed.

## Python Project Management

### Initialize a New Project

```bash
# Initialize in current directory
rsbuild python init

# Specify a custom package name
rsbuild python init --name mypackage

# Skip optional components
rsbuild python init --no-tests --no-devcontainer

# Preview what would be created
rsbuild --dry-run python init
```

This creates a complete Python project structure:

```
mypackage/
├── pyproject.toml          # Modern Python packaging (hatchling)
├── README.md               # Project documentation
├── .pre-commit-config.yaml # Pre-commit hooks (ruff, mypy)
├── .gitignore              # Python-specific ignores
├── Taskfile.yml            # Task runner commands
├── Dockerfile              # Container definition
├── docker-compose.yml      # Container orchestration
├── mypackage/
│   ├── __init__.py         # Package with __version__
│   └── tests/
│       └── test_example.py # Sample test
└── .devcontainer/
    └── devcontainer.json   # VS Code dev container
```

### Sync Version

Keep your package's `__version__` in sync with `pyproject.toml`:

```bash
rsbuild python sync-version
```

This reads the version from `pyproject.toml` and updates `__version__` and `__build__` in your package's `__init__.py`.

### Generated Taskfile Commands

After initialization, use the task runner:

```bash
task install      # Install dependencies with uv
task build        # Build wheel (syncs version first)
task test         # Run tests
task lint         # Run linter (ruff)
task format       # Format code (ruff)
task typecheck    # Run mypy
task clean        # Clean build artifacts
task sync-version # Sync version from pyproject.toml
task docker-build # Build Docker image
task ci           # Run all CI checks
```

## Clean

Remove build artifacts and caches:

```bash
# Clean Python artifacts (egg-info, pycache, notebooks)
rsbuild clean

# Also remove Rust target directory
rsbuild clean --all
```

## Doctor

Diagnose your system for required and optional tools:

```bash
# Human-readable output
rsbuild doctor

# Machine-readable JSON
rsbuild doctor --json
```

### Required Tools

| Tool | Purpose |
|------|---------|
| `cargo` | Rust builds |
| `docker` | Docker builds |

### Optional Tools

| Tool | Purpose |
|------|---------|
| `uv` | Python package management & wheel builds |
| `task` | Task runner (Taskfile) |
| `pip` | Python package installer (legacy) |
| `cythonize` | Cython compilation |
| `rsync` | Cython packaging |
| `glances` | System monitoring |
| `pre-commit` | Git hooks |

## Shell Completions

Generate shell completions for tab completion:

```bash
# Bash
rsbuild completions bash > ~/.local/share/bash-completion/completions/rsbuild

# Zsh
rsbuild completions zsh > ~/.zfunc/_rsbuild

# Fish
rsbuild completions fish > ~/.config/fish/completions/rsbuild.fish

# PowerShell
rsbuild completions powershell > _rsbuild.ps1
```

## Project Structure

```
src/
├── main.rs           # Entry point and command dispatch
├── cli.rs            # CLI definitions (clap derive)
├── error.rs          # Error types (thiserror)
├── executor.rs       # Command execution with progress support
├── output.rs         # JSON output formatting
├── progress.rs       # Progress bar utilities (indicatif)
├── parallel.rs       # Parallel execution (rayon)
├── watch.rs          # File watching (notify)
└── commands/
    ├── mod.rs        # Module exports
    ├── build.rs      # Build commands (wheel, cargo, docker)
    ├── clean.rs      # Clean command
    ├── cython.rs     # Cython compilation workflow
    ├── doctor.rs     # System diagnostics
    ├── pull.rs       # Docker pull commands
    ├── python.rs     # Python project scaffolding
    └── run.rs        # Docker run command
```

## Examples

### CI/CD Integration

```yaml
# GitHub Actions example
- name: Check dependencies
  run: rsbuild doctor --json | jq -e '.all_required_installed'

- name: Build all artifacts
  run: rsbuild build all -j4 --json > build-report.json

- name: Upload build report
  uses: actions/upload-artifact@v3
  with:
    name: build-report
    path: build-report.json
```

### Development Workflow

```bash
# Terminal 1: Watch and rebuild on changes
rsbuild watch wheel

# Terminal 2: Run tests after each rebuild
task test --watch
```

### Building a Release

```bash
# Ensure clean state
rsbuild clean --all

# Build everything in parallel
rsbuild build all -j4

# Verify the builds
ls dist/*.whl
ls target/Linux/x86_64/release/
```

## License

MIT
