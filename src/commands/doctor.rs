//! Doctor command - check system for required tools.

use crate::cli::ExecContext;
use crate::error::Result;
use crate::executor::check_tool;
use colored::Colorize;

/// Tools to check, grouped by functionality.
const REQUIRED_TOOLS: &[(&str, &str)] = &[
    ("cargo", "Rust builds"),
    ("docker", "Docker builds"),
];

const OPTIONAL_TOOLS: &[(&str, &str)] = &[
    ("uv", "Python package management & wheel builds"),
    ("task", "Task runner (Taskfile)"),
    ("pip", "Python package installer (legacy)"),
    ("cythonize", "Cython compilation"),
    ("rsync", "Cython packaging"),
    ("glances", "System monitoring"),
    ("pre-commit", "Git hooks"),
];

/// Execute the doctor command.
pub fn run(_ctx: &ExecContext) -> Result<()> {
    println!("{}", "rsbuild doctor".bold());
    println!("{}", "=".repeat(50));
    println!();

    let mut all_ok = true;

    println!("{}", "Required tools:".bold());
    for (tool, purpose) in REQUIRED_TOOLS {
        let status = if check_tool(tool).is_ok() {
            format!("{:>8}", "OK".green())
        } else {
            all_ok = false;
            format!("{:>8}", "MISSING".red())
        };
        println!("  {} {} - {}", status, tool.bold(), purpose);
    }
    println!();

    println!("{}", "Optional tools:".bold());
    for (tool, purpose) in OPTIONAL_TOOLS {
        let status = if check_tool(tool).is_ok() {
            format!("{:>8}", "OK".green())
        } else {
            format!("{:>8}", "MISSING".yellow())
        };
        println!("  {} {} - {}", status, tool.bold(), purpose);
    }
    println!();

    // Check for project files
    println!("{}", "Project files:".bold());

    let files_to_check = [
        ("docker-compose.yml", vec!["docker-compose.yml", "compose.yml"]),
        ("pyproject.toml", vec!["pyproject.toml", "setup.py"]),
        ("Cargo.toml", vec!["Cargo.toml"]),
        ("Taskfile.yml", vec!["Taskfile.yml", "Taskfile.yaml", "taskfile.yml"]),
        (".pre-commit-config.yaml", vec![".pre-commit-config.yaml"]),
    ];

    for (label, paths) in files_to_check {
        let found = paths.iter().any(|p| std::path::Path::new(p).exists());
        let status = if found {
            format!("{:>8}", "FOUND".green())
        } else {
            format!("{:>8}", "NOT FOUND".yellow())
        };
        println!("  {} {}", status, label);
    }
    println!();

    if all_ok {
        println!("{}", "All required tools are installed!".green().bold());
    } else {
        println!("{}", "Some required tools are missing.".red().bold());
        println!();
        println!("Installation hints:");
        println!("  cargo:  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh");
        println!("  docker: https://docs.docker.com/get-docker/");
        println!("  uv:     curl -LsSf https://astral.sh/uv/install.sh | sh");
        println!("  task:   https://taskfile.dev/installation/");
    }

    Ok(())
}
