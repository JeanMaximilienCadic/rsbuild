//! rsbuild - A self-sufficient runtime to build projects.
//!
//! This tool provides commands for building Python wheels, Docker containers,
//! Rust binaries, and managing Cython compilation workflows.

mod cli;
mod commands;
mod error;
mod executor;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::{Cli, Commands, ExecContext};
use std::io;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let ctx = ExecContext::from_cli(&cli);

    match cli.command {
        Commands::Build { target } => {
            commands::build::run(target, &ctx)?;
        }
        Commands::Pull { target } => {
            commands::pull::run(target, &ctx)?;
        }
        Commands::Run { service, args } => {
            commands::run::run(&service, &args, &ctx)?;
        }
        Commands::Clean { all } => {
            commands::clean::run(all, &ctx)?;
        }
        Commands::Cython { package } => {
            commands::cython::run(&package, &ctx)?;
        }
        Commands::Glances => {
            executor::check_tool("glances")?;
            executor::exec("glances", &ctx)?;
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "rsbuild", &mut io::stdout());
        }
        Commands::Doctor => {
            commands::doctor::run(&ctx)?;
        }
        Commands::External(args) => {
            let command = args.join(" ");
            executor::exec(&command, &ctx)?;
        }
    }

    Ok(())
}
