//! Docker run command implementation.

use crate::cli::ExecContext;
use crate::error::Result;
use crate::executor::{check_tool, exec, print_status};

/// Execute the run command.
pub fn run(service: &str, args: &[String], ctx: &ExecContext) -> Result<()> {
    check_tool("docker")?;

    let extra_args = if args.is_empty() {
        String::new()
    } else {
        format!(" {}", args.join(" "))
    };

    print_status(&format!("Running Docker service: {}", service), ctx);
    exec(&format!("docker compose run --rm {}{}", service, extra_args), ctx)?;
    Ok(())
}
