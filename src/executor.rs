use crate::error::{Result, RsbuildError};
use colored::Colorize;
use std::io::{self, Write};
use std::process::{Command, Output};

pub fn read_output(command: &str) -> Result<Output> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| RsbuildError::CommandFailed(e.to_string()))
}

pub fn read_output_str(command: &str) -> Result<String> {
    let output = read_output(command)?;
    let mut output_str = String::from_utf8(output.stdout)?;
    output_str.pop(); // Remove trailing newline
    Ok(output_str)
}

pub fn exec(command: &str, print_command: bool) -> Result<String> {
    if print_command {
        println!("{} `{}`", "[rsbuild]".bold().yellow(), command);
    }

    let output = read_output(command)?;

    let mut output_str = String::from_utf8(output.stdout)?;
    let mut error_str = String::from_utf8(output.stderr)?;

    output_str = output_str
        .replace("[output] ", "")
        .replace("[rsbuild] ", "");
    error_str = error_str
        .replace("[error] ", "")
        .replace("[rsbuild] ", "");

    let output_formatted = format!("{} {}", "[output]".bold().blue(), output_str);
    let error_formatted = format!("{} {}", "[error]".bold().red(), error_str);

    if !output_str.is_empty() && print_command {
        io::stdout().write_all(output_formatted.as_bytes())?;
    }
    if !error_str.is_empty() && print_command {
        io::stderr().write_all(error_formatted.as_bytes())?;
    }

    Ok(output_formatted)
}

pub fn exec_commands(commands: &[&str]) -> Result<()> {
    for command in commands {
        exec(command, true)?;
    }
    Ok(())
}
