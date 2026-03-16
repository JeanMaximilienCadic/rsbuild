//! Python project management commands.

use crate::cli::{ExecContext, PythonAction};
use crate::error::{Result, RsbuildError};
use crate::executor::{confirm_overwrite, print_status, print_warning};
use std::fs;
use std::path::Path;

/// Execute Python command.
pub fn run(action: PythonAction, ctx: &ExecContext) -> Result<()> {
    match action {
        PythonAction::Init { name, no_tests, no_devcontainer } => {
            init_project(name, no_tests, no_devcontainer, ctx)
        }
        PythonAction::SyncVersion => sync_version(ctx),
    }
}

/// Get the project name from the current directory or provided name.
fn get_project_name(name: Option<String>) -> Result<String> {
    if let Some(n) = name {
        return Ok(sanitize_package_name(&n));
    }

    let cwd = std::env::current_dir().map_err(|e| RsbuildError::ExecutionFailed(e.to_string()))?;
    let dir_name = cwd
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| RsbuildError::ExecutionFailed("Cannot determine project name".to_string()))?;

    Ok(sanitize_package_name(dir_name))
}

/// Sanitize a string to be a valid Python package name.
fn sanitize_package_name(name: &str) -> String {
    name.to_lowercase()
        .replace('-', "_")
        .replace(' ', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// Write a file if it doesn't exist or user confirms overwrite.
fn write_file(path: &Path, content: &str, ctx: &ExecContext) -> Result<bool> {
    if !confirm_overwrite(path, ctx) {
        print_warning(&format!("Skipping: {}", path.display()), ctx);
        return Ok(false);
    }

    if ctx.dry_run {
        print_status(&format!("Would create: {}", path.display()), ctx);
        return Ok(true);
    }

    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| RsbuildError::Io(e))?;
    }

    fs::write(path, content).map_err(|e| RsbuildError::Io(e))?;
    print_status(&format!("Created: {}", path.display()), ctx);
    Ok(true)
}

/// Initialize a new Python project.
fn init_project(name: Option<String>, no_tests: bool, no_devcontainer: bool, ctx: &ExecContext) -> Result<()> {
    let project_name = get_project_name(name)?;
    print_status(&format!("Initializing Python project: {}", project_name), ctx);

    // Create pyproject.toml
    write_file(
        Path::new("pyproject.toml"),
        &generate_pyproject_toml(&project_name),
        ctx,
    )?;

    // Create README.md
    write_file(
        Path::new("README.md"),
        &generate_readme(&project_name),
        ctx,
    )?;

    // Create .pre-commit-config.yaml
    write_file(
        Path::new(".pre-commit-config.yaml"),
        PRECOMMIT_CONFIG,
        ctx,
    )?;

    // Create .gitignore
    write_file(
        Path::new(".gitignore"),
        GITIGNORE,
        ctx,
    )?;

    // Create Taskfile.yml
    write_file(
        Path::new("Taskfile.yml"),
        &generate_taskfile(&project_name),
        ctx,
    )?;

    // Create package directory
    let pkg_dir = Path::new(&project_name);
    fs::create_dir_all(pkg_dir).ok();

    // Create __init__.py
    write_file(
        &pkg_dir.join("__init__.py"),
        &generate_init_py(&project_name),
        ctx,
    )?;

    // Create py.typed marker
    write_file(
        &pkg_dir.join("py.typed"),
        "",
        ctx,
    )?;

    // Create tests if not disabled
    if !no_tests {
        let tests_dir = pkg_dir.join("tests");
        fs::create_dir_all(&tests_dir).ok();

        write_file(
            &tests_dir.join("__init__.py"),
            "",
            ctx,
        )?;

        write_file(
            &tests_dir.join("test_version.py"),
            &generate_test_version(&project_name),
            ctx,
        )?;
    }

    // Create devcontainer if not disabled
    if !no_devcontainer {
        let devcontainer_dir = Path::new(".devcontainer");
        fs::create_dir_all(devcontainer_dir).ok();

        write_file(
            &devcontainer_dir.join("devcontainer.json"),
            &generate_devcontainer(&project_name),
            ctx,
        )?;

        write_file(
            &devcontainer_dir.join("Dockerfile"),
            DEVCONTAINER_DOCKERFILE,
            ctx,
        )?;
    }

    // Create Dockerfile
    write_file(
        Path::new("Dockerfile"),
        &generate_dockerfile(&project_name),
        ctx,
    )?;

    // Create docker-compose.yml
    write_file(
        Path::new("docker-compose.yml"),
        &generate_docker_compose(&project_name),
        ctx,
    )?;

    print_status("Python project initialized successfully!", ctx);
    print_status("Next steps:", ctx);
    println!("  1. Review and customize pyproject.toml");
    println!("  2. Install dependencies: uv sync");
    println!("  3. Install pre-commit hooks: pre-commit install");
    println!("  4. Run tests: task test");

    Ok(())
}

/// Sync version from pyproject.toml to package __init__.py.
fn sync_version(ctx: &ExecContext) -> Result<()> {
    print_status("Syncing version from pyproject.toml", ctx);

    // Read pyproject.toml
    let pyproject_path = Path::new("pyproject.toml");
    if !pyproject_path.exists() {
        return Err(RsbuildError::PathNotFound {
            path: pyproject_path.to_path_buf(),
        });
    }

    let pyproject_content = fs::read_to_string(pyproject_path)
        .map_err(|e| RsbuildError::Io(e))?;

    // Extract version and name
    let version = extract_toml_value(&pyproject_content, "version")
        .ok_or_else(|| RsbuildError::ExecutionFailed("Cannot find version in pyproject.toml".to_string()))?;
    let name = extract_toml_value(&pyproject_content, "name")
        .ok_or_else(|| RsbuildError::ExecutionFailed("Cannot find name in pyproject.toml".to_string()))?;

    let pkg_name = sanitize_package_name(&name);

    // Generate new __init__.py content
    let init_path = Path::new(&pkg_name).join("__init__.py");
    if !init_path.exists() {
        return Err(RsbuildError::PathNotFound {
            path: init_path.clone(),
        });
    }

    // Read current init.py to preserve any custom code after the header
    let current_content = fs::read_to_string(&init_path)
        .map_err(|e| RsbuildError::Io(e))?;

    // Find the end of the managed section (after __all__)
    let new_header = generate_init_py_header(&pkg_name, &version);

    // Check if there's custom code after __all__
    let custom_code = if let Some(pos) = current_content.find("__all__") {
        if let Some(end_pos) = current_content[pos..].find('\n') {
            let after_all = &current_content[pos + end_pos + 1..];
            if after_all.trim().is_empty() {
                String::new()
            } else {
                format!("\n{}", after_all)
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let new_content = format!("{}{}", new_header, custom_code);

    if ctx.dry_run {
        print_status(&format!("Would update {} to version {}", init_path.display(), version), ctx);
        return Ok(());
    }

    fs::write(&init_path, new_content).map_err(|e| RsbuildError::Io(e))?;
    print_status(&format!("Updated {} to version {}", init_path.display(), version), ctx);

    Ok(())
}

/// Extract a value from TOML content (simple parser for common cases).
fn extract_toml_value(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(&format!("{} ", key)) || line.starts_with(&format!("{}=", key)) {
            if let Some(value) = line.split('=').nth(1) {
                let value = value.trim().trim_matches('"').trim_matches('\'');
                return Some(value.to_string());
            }
        }
    }
    None
}

/// Generate the header portion of __init__.py.
fn generate_init_py_header(project_name: &str, version: &str) -> String {
    // Generate build timestamp
    let build = chrono_lite_now();

    format!(
        r#""""{project_name} - A Python package."""

__version__ = "{version}"
__build__ = "{build}"

__all__ = ["__version__", "__build__"]
"#,
        project_name = project_name,
        version = version,
        build = build
    )
}

/// Get current timestamp in a simple format (without external deps).
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();

    let secs = duration.as_secs();

    // Simple date calculation (approximate, good enough for build timestamps)
    let days = secs / 86400;
    let years_since_1970 = days / 365;
    let year = 1970 + years_since_1970;

    let remaining_days = days % 365;
    let month = (remaining_days / 30) + 1;
    let day = (remaining_days % 30) + 1;

    let day_secs = secs % 86400;
    let hour = day_secs / 3600;
    let minute = (day_secs % 3600) / 60;
    let second = day_secs % 60;

    format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}",
        year, month.min(12), day.min(31), hour, minute, second
    )
}

// =============================================================================
// TEMPLATES
// =============================================================================

fn generate_pyproject_toml(project_name: &str) -> String {
    format!(
        r#"[project]
name = "{project_name}"
version = "0.1.0"
description = "A Python package"
readme = "README.md"
requires-python = ">=3.10"
license = "MIT"
keywords = []
classifiers = [
    "Development Status :: 3 - Alpha",
    "Intended Audience :: Developers",
    "License :: OSI Approved :: MIT License",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3.10",
    "Programming Language :: Python :: 3.11",
    "Programming Language :: Python :: 3.12",
]
dependencies = []

[project.optional-dependencies]
dev = [
    "pytest>=8.0",
    "pytest-cov>=4.0",
    "ruff>=0.4",
    "mypy>=1.0",
    "pre-commit>=3.0",
]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.hatch.build.targets.wheel]
packages = ["{project_name}"]

[tool.ruff]
line-length = 120
target-version = "py310"

[tool.ruff.lint]
select = ["E", "F", "I", "N", "W", "UP", "B", "C4", "SIM"]
ignore = ["E501"]

[tool.mypy]
python_version = "3.10"
strict = true
warn_return_any = true
warn_unused_configs = true

[tool.pytest.ini_options]
testpaths = ["{project_name}/tests"]
addopts = "-v --cov={project_name} --cov-report=term-missing"

[tool.coverage.run]
source = ["{project_name}"]
omit = ["{project_name}/tests/*"]
"#,
        project_name = project_name
    )
}

fn generate_readme(project_name: &str) -> String {
    format!(
        r#"# {project_name}

A Python package.

## Installation

```bash
pip install {project_name}
```

## Development

```bash
# Install dependencies
uv sync --all-extras

# Run tests
task test

# Build wheel
task build

# Lint and format
task lint
```

## License

MIT
"#,
        project_name = project_name
    )
}

fn generate_init_py(project_name: &str) -> String {
    generate_init_py_header(project_name, "0.1.0")
}

fn generate_test_version(project_name: &str) -> String {
    format!(
        r#"""Tests for version information."""

from {project_name} import __build__, __version__


def test_version_exists() -> None:
    """Test that version is defined."""
    assert __version__ is not None
    assert isinstance(__version__, str)


def test_build_exists() -> None:
    """Test that build is defined."""
    assert __build__ is not None
    assert isinstance(__build__, str)


def test_version_format() -> None:
    """Test that version follows semver format."""
    parts = __version__.split(".")
    assert len(parts) >= 2
    assert all(part.isdigit() for part in parts[:2])
"#,
        project_name = project_name
    )
}

fn generate_taskfile(project_name: &str) -> String {
    format!(
        r##"# https://taskfile.dev

version: "3"

vars:
  PACKAGE: {project_name}

tasks:
  default:
    desc: Show available tasks
    cmds:
      - task --list

  install:
    desc: Install dependencies
    cmds:
      - uv sync --all-extras

  build:
    desc: Build wheel
    cmds:
      - task: sync-version
      - uv build --wheel

  test:
    desc: Run tests
    cmds:
      - uv run pytest

  test-cov:
    desc: Run tests with coverage
    cmds:
      - uv run pytest --cov={{{{.PACKAGE}}}} --cov-report=html

  lint:
    desc: Run linter
    cmds:
      - uv run ruff check {{{{.PACKAGE}}}}
      - uv run ruff format --check {{{{.PACKAGE}}}}

  format:
    desc: Format code
    cmds:
      - uv run ruff format {{{{.PACKAGE}}}}
      - uv run ruff check --fix {{{{.PACKAGE}}}}

  typecheck:
    desc: Run type checker
    cmds:
      - uv run mypy {{{{.PACKAGE}}}}

  clean:
    desc: Clean build artifacts
    cmds:
      - rm -rf dist build *.egg-info .pytest_cache .mypy_cache .ruff_cache htmlcov .coverage
      - find . -type d -name __pycache__ -exec rm -rf {{{{}}}} + 2>/dev/null || true

  sync-version:
    desc: Sync version from pyproject.toml to package
    cmds:
      - |
        VERSION=$(grep '^version' pyproject.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
        BUILD=$(date -u +"%Y%m%d%H%M%S")
        cat > {{{{.PACKAGE}}}}/__init__.py << EOF
        """{{{{.PACKAGE}}}} - A Python package."""

        __version__ = "$VERSION"
        __build__ = "$BUILD"

        __all__ = ["__version__", "__build__"]
        EOF
        echo "Synced version $VERSION with build $BUILD"

  docker-build:
    desc: Build Docker image
    cmds:
      - docker compose build

  docker-run:
    desc: Run Docker container
    cmds:
      - docker compose run --rm app

  pre-commit:
    desc: Run pre-commit hooks
    cmds:
      - pre-commit run --all-files

  ci:
    desc: Run all CI checks
    cmds:
      - task: lint
      - task: typecheck
      - task: test
"##,
        project_name = project_name
    )
}

fn generate_devcontainer(project_name: &str) -> String {
    format!(
        r#"{{
    "name": "{project_name}",
    "build": {{
        "dockerfile": "Dockerfile"
    }},
    "features": {{
        "ghcr.io/devcontainers/features/common-utils:2": {{}},
        "ghcr.io/devcontainers/features/git:1": {{}}
    }},
    "customizations": {{
        "vscode": {{
            "extensions": [
                "ms-python.python",
                "ms-python.vscode-pylance",
                "charliermarsh.ruff",
                "tamasfe.even-better-toml",
                "eamodio.gitlens"
            ],
            "settings": {{
                "python.defaultInterpreterPath": "/usr/local/bin/python",
                "python.analysis.typeCheckingMode": "basic",
                "[python]": {{
                    "editor.defaultFormatter": "charliermarsh.ruff",
                    "editor.formatOnSave": true,
                    "editor.codeActionsOnSave": {{
                        "source.fixAll.ruff": "explicit",
                        "source.organizeImports.ruff": "explicit"
                    }}
                }}
            }}
        }}
    }},
    "postCreateCommand": "uv sync --all-extras && pre-commit install",
    "remoteUser": "vscode"
}}
"#,
        project_name = project_name
    )
}

fn generate_dockerfile(project_name: &str) -> String {
    format!(
        r#"FROM python:3.12-slim

WORKDIR /app

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv

# Copy project files
COPY pyproject.toml uv.lock* ./
COPY {project_name}/ ./{project_name}/

# Install dependencies
RUN uv sync --frozen --no-dev

# Default command
CMD ["uv", "run", "python", "-c", "import {project_name}; print({project_name}.__version__)"]
"#,
        project_name = project_name
    )
}

fn generate_docker_compose(_project_name: &str) -> String {
    format!(
        r#"services:
  app:
    build: .
    volumes:
      - .:/app
    working_dir: /app

  dev:
    build:
      context: .
      dockerfile: Dockerfile
    volumes:
      - .:/app
    working_dir: /app
    command: ["uv", "run", "python"]
    stdin_open: true
    tty: true
"#
    )
}

const PRECOMMIT_CONFIG: &str = r#"repos:
  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v5.0.0
    hooks:
      - id: trailing-whitespace
      - id: end-of-file-fixer
      - id: check-yaml
      - id: check-toml
      - id: check-added-large-files
      - id: check-merge-conflict

  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.8.6
    hooks:
      - id: ruff
        args: [--fix]
      - id: ruff-format

  - repo: https://github.com/pre-commit/mirrors-mypy
    rev: v1.14.1
    hooks:
      - id: mypy
        additional_dependencies: []
"#;

const GITIGNORE: &str = r#"# Python
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
build/
develop-eggs/
dist/
downloads/
eggs/
.eggs/
lib/
lib64/
parts/
sdist/
var/
wheels/
*.egg-info/
.installed.cfg
*.egg

# Virtual environments
.venv/
venv/
ENV/

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# Testing
.tox/
.nox/
.coverage
.coverage.*
htmlcov/
.pytest_cache/
.hypothesis/

# Type checking
.mypy_cache/
.dmypy.json
dmypy.json

# Ruff
.ruff_cache/

# Build
*.manifest
*.spec

# Jupyter
.ipynb_checkpoints/

# OS
.DS_Store
Thumbs.db

# Project specific
*.log
.env
.env.*
"#;

const DEVCONTAINER_DOCKERFILE: &str = r#"FROM mcr.microsoft.com/devcontainers/python:3.12

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv

# Install task
RUN sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b /usr/local/bin

# Set up uv environment
ENV UV_LINK_MODE=copy
"#;
