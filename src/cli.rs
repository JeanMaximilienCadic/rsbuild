use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rsbuild")]
#[command(author = "Jean Maximilien Cadic")]
#[command(version)]
#[command(about = "A self-sufficient runtime to build projects", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build artifacts (wheel, docker, cargo)
    Build {
        #[command(subcommand)]
        target: BuildTarget,
    },

    /// Pull docker images
    Pull {
        #[command(subcommand)]
        target: PullTarget,
    },

    /// Clean build artifacts
    Clean,

    /// Compile Cython modules
    Cython {
        /// Package name to compile
        package: String,
    },

    /// Run glances system monitor
    Glances,

    /// Execute arbitrary shell command
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Subcommand)]
pub enum BuildTarget {
    /// Build Python wheel
    Wheel,

    /// Build all targets (wheel, vanilla, sandbox)
    All,

    /// Build vanilla Docker container
    Vanilla,

    /// Build sandbox Docker container
    Sandbox,

    /// Build Rust binary
    Cargo {
        #[command(subcommand)]
        mode: CargoBuildMode,
    },

    /// Build a Docker Compose service
    Docker {
        /// Service name
        service: String,
    },
}

#[derive(Subcommand)]
pub enum CargoBuildMode {
    /// Build in debug mode
    Debug,

    /// Build in release mode
    Release,
}

#[derive(Subcommand)]
pub enum PullTarget {
    /// Pull all images
    All,

    /// Pull vanilla image
    Vanilla,

    /// Pull sandbox image
    Sandbox,
}
