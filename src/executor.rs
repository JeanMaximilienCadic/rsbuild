//! Command execution utilities.

use crate::cli::ExecContext;
use crate::error::{Result, RsbuildError};
use colored::Colorize;
use std::io::{self, Write};
use std::process::{Command, Output};

/// Prefix used for rsbuild output messages.
const PREFIX_RSBUILD: &str = "[rsbuild]";
const PREFIX_OUTPUT: &str = "[output]";
const PREFIX_ERROR: &str = "[error]";
const PREFIX_DRY_RUN: &str = "[dry-run]";

/// Check if a required tool is available in PATH.
pub fn check_tool(name: &str) -> Result<()> {
    which::which(name).map_err(|_| RsbuildError::ToolNotFound {
        tool: name.to_string(),
        hint: get_install_hint(name),
    })?;
    Ok(())
}

/// Get installation hint for common tools.
fn get_install_hint(tool: &str) -> String {
    match tool {
        "docker" => "Install Docker: https://docs.docker.com/get-docker/".to_string(),
        "cargo" => "Install Rust: https://rustup.rs/".to_string(),
        "pip" => "Install pip: https://pip.pypa.io/en/stable/installation/".to_string(),
        "cythonize" => "Install Cython: pip install cython".to_string(),
        "rsync" => "Install rsync using your package manager".to_string(),
        "glances" => "Install glances: pip install glances".to_string(),
        _ => format!("Please install '{}' and ensure it's in your PATH", tool),
    }
}

/// Execute a shell command and capture output.
pub fn read_output(command: &str) -> Result<Output> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| RsbuildError::ExecutionFailed(e.to_string()))
}

/// Execute a command and return stdout as a string.
pub fn read_output_str(command: &str) -> Result<String> {
    let output = read_output(command)?;
    let mut output_str = String::from_utf8(output.stdout)?;
    // Remove trailing newline if present
    if output_str.ends_with('\n') {
        output_str.pop();
    }
    Ok(output_str)
}

/// Execute a command with output formatting.
pub fn exec(command: &str, ctx: &ExecContext) -> Result<String> {
    // Handle dry-run mode
    if ctx.dry_run {
        if ctx.should_print() {
            println!(
                "{} {}",
                PREFIX_DRY_RUN.bold().cyan(),
                command.italic()
            );
        }
        return Ok(String::new());
    }

    // Print command being executed
    if ctx.should_print() {
        println!("{} `{}`", PREFIX_RSBUILD.bold().yellow(), command);
    }

    let output = read_output(command)?;
    let status = output.status;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    // Clean output by removing any nested prefixes
    let stdout_clean = stdout
        .replace(&format!("{} ", PREFIX_OUTPUT), "")
        .replace(&format!("{} ", PREFIX_RSBUILD), "");
    let stderr_clean = stderr
        .replace(&format!("{} ", PREFIX_ERROR), "")
        .replace(&format!("{} ", PREFIX_RSBUILD), "");

    // Print output if not quiet
    if ctx.should_print() || ctx.verbose {
        if !stdout_clean.is_empty() {
            let formatted = format!("{} {}", PREFIX_OUTPUT.bold().blue(), stdout_clean);
            io::stdout().write_all(formatted.as_bytes())?;
        }
        if !stderr_clean.is_empty() {
            let formatted = format!("{} {}", PREFIX_ERROR.bold().red(), stderr_clean);
            io::stderr().write_all(formatted.as_bytes())?;
        }
    }

    // Check exit status
    if !status.success() {
        return Err(RsbuildError::CommandFailed {
            command: command.to_string(),
            code: status.code().unwrap_or(-1),
            message: stderr_clean.lines().next().unwrap_or("Unknown error").to_string(),
        });
    }

    Ok(stdout_clean)
}

/// Execute a command silently (no output unless error).
pub fn exec_silent(command: &str, ctx: &ExecContext) -> Result<String> {
    if ctx.dry_run {
        return Ok(String::new());
    }

    let output = read_output(command)?;
    let stdout = String::from_utf8(output.stdout)?;

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(RsbuildError::CommandFailed {
            command: command.to_string(),
            code: output.status.code().unwrap_or(-1),
            message: stderr.lines().next().unwrap_or("Unknown error").to_string(),
        });
    }

    Ok(stdout)
}

/// Execute multiple commands in sequence.
pub fn exec_commands(commands: &[&str], ctx: &ExecContext) -> Result<()> {
    for command in commands {
        exec(command, ctx)?;
    }
    Ok(())
}

/// Execute a command, ignoring failures (useful for cleanup).
pub fn exec_ignore_error(command: &str, ctx: &ExecContext) {
    let _ = exec_silent(command, ctx);
}

/// Print a status message.
pub fn print_status(message: &str, ctx: &ExecContext) {
    if ctx.should_print() {
        println!("{} {}", PREFIX_RSBUILD.bold().green(), message);
    }
}

/// Print a warning message.
pub fn print_warning(message: &str, ctx: &ExecContext) {
    if ctx.should_print() {
        eprintln!("{} {}", "[warning]".bold().yellow(), message);
    }
}
