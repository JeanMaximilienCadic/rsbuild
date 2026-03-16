//! Command-line interface definitions.

use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;
use indicatif::MultiProgress;
use std::sync::Arc;

/// Output format for command results.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OutputFormat {
    #[default]
    Human,
    Json,
}

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

    /// Output results in JSON format (implies --quiet)
    #[arg(long, global = true)]
    pub json: bool,

    /// Number of parallel jobs to run (default: 1)
    #[arg(short = 'j', long, global = true, default_value = "1")]
    pub jobs: usize,

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

    /// Watch files and rebuild on changes
    Watch {
        #[command(subcommand)]
        target: WatchTarget,

        /// Debounce delay in milliseconds
        #[arg(short, long, default_value = "500")]
        debounce: u64,

        /// Additional paths to watch (can be specified multiple times)
        #[arg(short, long)]
        path: Vec<String>,
    },

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

/// Watch targets for the watch command.
#[derive(Subcommand, Clone, Copy, Debug)]
pub enum WatchTarget {
    /// Watch and rebuild Python wheel
    Wheel,
    /// Watch and rebuild Docker service
    Docker,
    /// Watch and recompile Cython modules
    Cython,
    /// Watch and rebuild Rust binary
    Cargo,
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
#[derive(Clone)]
pub struct ExecContext {
    pub verbose: bool,
    pub quiet: bool,
    pub dry_run: bool,
    pub yes: bool,
    pub output_format: OutputFormat,
    pub jobs: usize,
    pub progress: Option<Arc<MultiProgress>>,
}

impl ExecContext {
    /// Create context from CLI flags.
    pub fn from_cli(cli: &Cli) -> Self {
        let output_format = if cli.json {
            OutputFormat::Json
        } else {
            OutputFormat::Human
        };

        // JSON mode implies quiet
        let quiet = cli.quiet || cli.json;

        // Create progress bar manager only if not in quiet/json mode
        let progress = if !quiet {
            Some(Arc::new(MultiProgress::new()))
        } else {
            None
        };

        Self {
            verbose: cli.verbose,
            quiet,
            dry_run: cli.dry_run,
            yes: cli.yes,
            output_format,
            jobs: cli.jobs.max(1),
            progress,
        }
    }

    /// Check if output should be shown.
    pub fn should_print(&self) -> bool {
        !self.quiet
    }

    /// Check if JSON output mode is enabled.
    pub fn is_json(&self) -> bool {
        self.output_format == OutputFormat::Json
    }

    /// Check if parallel execution is enabled.
    pub fn is_parallel(&self) -> bool {
        self.jobs > 1
    }
}
