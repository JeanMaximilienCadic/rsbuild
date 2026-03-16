//! Docker pull command implementations.

use crate::cli::{ExecContext, PullTarget};
use crate::error::Result;
use crate::executor::{check_tool, exec, print_status, print_warning};
use crate::output::{self, BatchResult, CommandResult};
use crate::parallel::{self, TaskResult};
use std::time::Instant;

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
    let start = Instant::now();
    let services = vec!["vanilla", "sandbox"];

    if ctx.is_parallel() {
        // Parallel execution
        let tasks: Vec<_> = services
            .iter()
            .map(|&service| {
                let ctx = ctx.clone();
                let name = service.to_string();
                (
                    name.clone(),
                    move || -> std::result::Result<String, String> {
                        pull_service_internal(&name, &ctx).map(|_| format!("Pulled {}", name))
                    },
                )
            })
            .collect();

        let results = parallel::execute(tasks, ctx, "Pulling Docker images");
        let duration = start.elapsed();

        if ctx.is_json() {
            let batch = results_to_batch(&results, duration);
            output::emit_json(&batch);
        } else {
            let failed: Vec<_> = results.iter().filter(|r| r.result.is_err()).collect();
            if !failed.is_empty() {
                for r in failed {
                    if let Err(e) = &r.result {
                        print_warning(&format!("Failed to pull '{}': {}", r.name, e), ctx);
                    }
                }
            }
            print_status("Pull all completed", ctx);
        }
    } else {
        // Sequential execution
        print_status("Pulling all Docker images", ctx);

        let mut results = Vec::new();

        for service in &services {
            match pull_service(service, ctx) {
                Ok(_) => {
                    results.push(CommandResult::success(*service, std::time::Duration::ZERO, None));
                }
                Err(e) => {
                    print_warning(&format!("Failed to pull '{}': {}", service, e), ctx);
                    results.push(CommandResult::failed(
                        *service,
                        std::time::Duration::ZERO,
                        e.to_string(),
                    ));
                }
            }
        }

        if ctx.is_json() {
            let batch = BatchResult::from_results(results, start.elapsed());
            output::emit_json(&batch);
        } else {
            print_status("Pull all completed", ctx);
        }
    }

    Ok(())
}

/// Pull a specific Docker Compose service image.
fn pull_service(name: &str, ctx: &ExecContext) -> Result<()> {
    print_status(&format!("Pulling Docker image: {}", name), ctx);
    exec(&format!("docker compose pull {}", name), ctx)?;
    Ok(())
}

/// Internal pull for parallel execution.
fn pull_service_internal(name: &str, ctx: &ExecContext) -> std::result::Result<(), String> {
    exec(&format!("docker compose pull {}", name), ctx).map_err(|e| e.to_string())?;
    Ok(())
}

/// Convert task results to batch result for JSON output.
fn results_to_batch(results: &[TaskResult<String>], duration: std::time::Duration) -> BatchResult {
    let command_results: Vec<CommandResult> = results
        .iter()
        .map(|r| match &r.result {
            Ok(msg) => CommandResult::success(&r.name, std::time::Duration::ZERO, Some(msg.clone())),
            Err(e) => CommandResult::failed(&r.name, std::time::Duration::ZERO, e),
        })
        .collect();

    BatchResult::from_results(command_results, duration)
}
