//! Command-line interface definitions.

use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

/// A self-sufficient runtime to build projects.
///
/// rsbuild provides commands for building Python wheels, Docker containers,
/// Rust binaries, and managing Cython compilation workflows.
#[derive(Parser)]
#[command(name = "rsbuild")]
#[command(version)]
#[command(about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Increase output verbosity
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-essential output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Preview commands without executing them
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Skip confirmation prompts (answer yes to all)
    #[arg(short = 'y', long, global = true)]
    pub yes: bool,

    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Build artifacts (wheel, docker, cargo)
    Build {
        #[command(subcommand)]
        target: BuildTarget,
    },

    /// Pull Docker images
    Pull {
        #[command(subcommand)]
        target: PullTarget,
    },

    /// Run Docker Compose services
    Run {
        /// Service name to run
        service: String,

        /// Additional arguments to pass to docker compose run
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Clean build artifacts and caches
    Clean {
        /// Also remove Rust target directory
        #[arg(long)]
        all: bool,
    },

    /// Compile Cython modules and package into wheel
    Cython {
        /// Package name to compile
        package: String,
    },

    /// Python project management
    Python {
        #[command(subcommand)]
        action: PythonAction,
    },

    /// Run glances system monitor
    Glances,

    /// Generate shell completion scripts
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Check if required tools are installed
    Doctor,

    /// Execute arbitrary shell command
    #[command(external_subcommand)]
    External(Vec<String>),
}

/// Build targets for the build command.
#[derive(Subcommand)]
pub enum BuildTarget {
    /// Build Python wheel using uv
    Wheel,

    /// Build all configured targets
    All,

    /// Build Rust binary with cargo
    Cargo {
        /// Build mode
        #[arg(value_enum, default_value = "release")]
        mode: CargoBuildMode,
    },

    /// Build a Docker Compose service
    Docker {
        /// Service name (e.g., vanilla, sandbox, or any service in docker-compose.yml)
        service: String,

        /// Build without cache
        #[arg(long)]
        no_cache: bool,
    },
}

/// Cargo build modes.
#[derive(Clone, Debug, ValueEnum)]
pub enum CargoBuildMode {
    /// Debug build (faster compilation, slower runtime)
    Debug,
    /// Release build (optimized, slower compilation)
    Release,
}

/// Pull targets for the pull command.
#[derive(Subcommand)]
pub enum PullTarget {
    /// Pull all configured images
    All,

    /// Pull a specific Docker Compose service image
    Service {
        /// Service name from docker-compose.yml
        name: String,
    },
}

/// Python project actions.
#[derive(Subcommand)]
pub enum PythonAction {
    /// Initialize a new Python project with best practices
    Init {
        /// Project name (defaults to current directory name)
        #[arg(short, long)]
        name: Option<String>,

        /// Skip creating tests directory
        #[arg(long)]
        no_tests: bool,

        /// Skip creating devcontainer
        #[arg(long)]
        no_devcontainer: bool,
    },

    /// Sync version from pyproject.toml to package __init__.py
    SyncVersion,
}

/// Execution context passed to commands.
#[derive(Clone, Copy)]
pub struct ExecContext {
    pub verbose: bool,
    pub quiet: bool,
    pub dry_run: bool,
    pub yes: bool,
}

impl ExecContext {
    /// Create context from CLI flags.
    pub fn from_cli(cli: &Cli) -> Self {
        Self {
            verbose: cli.verbose,
            quiet: cli.quiet,
            dry_run: cli.dry_run,
            yes: cli.yes,
        }
    }

    /// Check if output should be shown.
    pub fn should_print(&self) -> bool {
        !self.quiet
    }
}
