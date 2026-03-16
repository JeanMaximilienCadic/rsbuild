//! Docker pull command implementations.

use crate::cli::{ExecContext, PullTarget};
use crate::error::Result;
use crate::executor::{check_tool, exec, print_status, print_warning};

/// Execute the pull command.
pub fn run(target: PullTarget, ctx: &ExecContext) -> Result<()> {
    check_tool("docker")?;

    match target {
        PullTarget::All => pull_all(ctx),
        PullTarget::Service { name } => pull_service(&name, ctx),
    }
}

/// Pull all configured Docker images.
fn pull_all(ctx: &ExecContext) -> Result<()> {
    print_status("Pulling all Docker images", ctx);

    // Pull common services, continuing on failure
    for service in &["vanilla", "sandbox"] {
        if let Err(e) = pull_service(service, ctx) {
            print_warning(&format!("Failed to pull '{}': {}", service, e), ctx);
        }
    }

    print_status("Pull all completed", ctx);
    Ok(())
}

/// Pull a specific Docker Compose service image.
fn pull_service(name: &str, ctx: &ExecContext) -> Result<()> {
    print_status(&format!("Pulling Docker image: {}", name), ctx);
    exec(&format!("docker compose pull {}", name), ctx)?;
    Ok(())
}
