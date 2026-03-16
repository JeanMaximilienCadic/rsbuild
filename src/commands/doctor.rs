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
    ("pip", "Python wheel builds"),
    ("cythonize", "Cython compilation"),
    ("rsync", "Cython packaging"),
    ("glances", "System monitoring"),
];

/// Execute the doctor command.
pub fn run(_ctx: &ExecContext) -> Result<()> {
    println!("{}", "rsbuild doctor".bold());
    println!("{}", "=".repeat(40));
    println!();

    let mut all_ok = true;

    println!("{}", "Required tools:".bold());
    for (tool, purpose) in REQUIRED_TOOLS {
        let status = if check_tool(tool).is_ok() {
            format!("{}", "OK".green())
        } else {
            all_ok = false;
            format!("{}", "MISSING".red())
        };
        println!("  {} {} - {}", status, tool.bold(), purpose);
    }
    println!();

    println!("{}", "Optional tools:".bold());
    for (tool, purpose) in OPTIONAL_TOOLS {
        let status = if check_tool(tool).is_ok() {
            format!("{}", "OK".green())
        } else {
            format!("{}", "MISSING".yellow())
        };
        println!("  {} {} - {}", status, tool.bold(), purpose);
    }
    println!();

    // Check for docker-compose.yml
    println!("{}", "Project files:".bold());
    let compose_exists = std::path::Path::new("docker-compose.yml").exists()
        || std::path::Path::new("compose.yml").exists();
    let compose_status = if compose_exists {
        format!("{}", "FOUND".green())
    } else {
        format!("{}", "NOT FOUND".yellow())
    };
    println!("  {} docker-compose.yml", compose_status);

    let setup_exists = std::path::Path::new("setup.py").exists()
        || std::path::Path::new("pyproject.toml").exists();
    let setup_status = if setup_exists {
        format!("{}", "FOUND".green())
    } else {
        format!("{}", "NOT FOUND".yellow())
    };
    println!("  {} setup.py/pyproject.toml", setup_status);

    let cargo_exists = std::path::Path::new("Cargo.toml").exists();
    let cargo_status = if cargo_exists {
        format!("{}", "FOUND".green())
    } else {
        format!("{}", "NOT FOUND".yellow())
    };
    println!("  {} Cargo.toml", cargo_status);
    println!();

    if all_ok {
        println!("{}", "All required tools are installed!".green().bold());
    } else {
        println!("{}", "Some required tools are missing.".red().bold());
    }

    Ok(())
}
