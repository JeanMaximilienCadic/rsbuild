mod cli;
mod commands;
mod error;
mod executor;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { target } => commands::build::run(target)?,
        Commands::Pull { target } => commands::pull::run(target)?,
        Commands::Clean => commands::clean::run()?,
        Commands::Cython { package } => commands::cython::run(&package)?,
        Commands::Glances => {
            executor::exec("glances", true)?;
        }
        Commands::External(args) => {
            let command = args.join(" ");
            executor::exec(&command, true)?;
        }
    }

    Ok(())
}
