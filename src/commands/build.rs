//! Build command implementations.

use crate::cli::{BuildTarget, CargoBuildMode, ExecContext};
use crate::error::Result;
use crate::executor::{check_tool, exec, exec_ignore_error, print_status, read_output_str};
use crate::output::{self, BatchResult, CommandResult};
use crate::parallel::{self, TaskResult};
use std::time::Instant;

/// Execute the build command.
pub fn run(target: BuildTarget, ctx: &ExecContext) -> Result<()> {
    match target {
        BuildTarget::Wheel => build_wheel(ctx),
        BuildTarget::All => build_all(ctx),
        BuildTarget::Cargo { mode } => cargo_build(mode, ctx),
        BuildTarget::Docker { service, no_cache } => build_docker(service, no_cache, ctx),
    }
}

/// Build a Rust binary using cargo.
fn cargo_build(mode: CargoBuildMode, ctx: &ExecContext) -> Result<()> {
    check_tool("cargo")?;

    let os = read_output_str("uname")?;
    let arch = read_output_str("uname -m")?;
    let target_dir = format!("target/{}/{}", os, arch);

    let mode_flag = match mode {
        CargoBuildMode::Debug => "",
        CargoBuildMode::Release => "--release",
    };

    print_status(&format!("Building Rust binary ({:?} mode)", mode), ctx);

    let cmd = if mode_flag.is_empty() {
        format!("cargo build --target-dir {}", target_dir)
    } else {
        format!("cargo build {} --target-dir {}", mode_flag, target_dir)
    };

    exec(&cmd, ctx)?;
    print_status(&format!("Binary built in {}", target_dir), ctx);
    Ok(())
}

/// Build a Python wheel using uv.
fn build_wheel(ctx: &ExecContext) -> Result<()> {
    check_tool("uv")?;

    print_status("Building Python wheel with uv", ctx);

    // Create dist directory if it doesn't exist
    exec_ignore_error("mkdir -p dist/legacy", ctx);

    // Move existing wheels to legacy (ignore if none exist)
    exec_ignore_error("mv dist/*.whl dist/legacy/ 2>/dev/null", ctx);

    // Build wheel using uv
    exec("uv build --wheel", ctx)?;

    // Clean build artifacts
    exec_ignore_error("rsbuild clean", ctx);

    print_status("Wheel built in dist/", ctx);
    Ok(())
}

/// Build a Docker Compose service.
fn build_docker(service: String, no_cache: bool, ctx: &ExecContext) -> Result<()> {
    check_tool("docker")?;

    print_status(&format!("Building Docker service: {}", service), ctx);

    let cmd = if no_cache {
        format!("docker compose build --no-cache {}", service)
    } else {
        format!("docker compose build {}", service)
    };

    exec(&cmd, ctx)?;
    print_status(&format!("Docker service '{}' built successfully", service), ctx);
    Ok(())
}

/// Build all configured targets.
fn build_all(ctx: &ExecContext) -> Result<()> {
    let start = Instant::now();

    // Collect tasks
    let mut tasks: Vec<(String, Box<dyn Fn() -> std::result::Result<String, String> + Send + Sync>)> = Vec::new();

    // Add wheel build task
    let ctx_wheel = ctx.clone();
    tasks.push((
        "wheel".to_string(),
        Box::new(move || {
            build_wheel_internal(&ctx_wheel).map(|_| "Wheel built".to_string())
        }),
    ));

    // Add docker service tasks if compose file exists
    let has_compose = std::path::Path::new("docker-compose.yml").exists()
        || std::path::Path::new("compose.yml").exists();

    if has_compose {
        for service in ["vanilla", "sandbox"] {
            let ctx_docker = ctx.clone();
            let service_name = service.to_string();
            tasks.push((
                format!("docker:{}", service),
                Box::new(move || {
                    build_docker_internal(&service_name, false, &ctx_docker)
                        .map(|_| format!("Docker {} built", service_name))
                }),
            ));
        }
    }

    if ctx.is_parallel() {
        // Parallel execution
        let results = parallel::execute(tasks, ctx, "Building all targets");
        let duration = start.elapsed();

        if ctx.is_json() {
            let batch = results_to_batch(&results, duration);
            output::emit_json(&batch);
        } else {
            // Print summary
            let failed: Vec<_> = results.iter().filter(|r| r.result.is_err()).collect();
            if !failed.is_empty() {
                for r in failed {
                    if let Err(e) = &r.result {
                        crate::executor::print_warning(&format!("{} failed: {}", r.name, e), ctx);
                    }
                }
            }
            print_status("Build all completed", ctx);
        }
    } else {
        // Sequential execution
        print_status("Building all targets", ctx);

        let mut results = Vec::new();

        // Build wheel first
        match build_wheel(ctx) {
            Ok(_) => results.push(CommandResult::success("wheel", std::time::Duration::ZERO, None)),
            Err(e) => {
                crate::executor::print_warning(&format!("Wheel build failed: {}", e), ctx);
                results.push(CommandResult::failed("wheel", std::time::Duration::ZERO, e.to_string()));
            }
        }

        // Build docker services
        if has_compose {
            for service in &["vanilla", "sandbox"] {
                match build_docker(service.to_string(), false, ctx) {
                    Ok(_) => results.push(CommandResult::success(
                        format!("docker:{}", service),
                        std::time::Duration::ZERO,
                        None,
                    )),
                    Err(e) => {
                        crate::executor::print_warning(
                            &format!("Docker build '{}' failed: {}", service, e),
                            ctx,
                        );
                        results.push(CommandResult::failed(
                            format!("docker:{}", service),
                            std::time::Duration::ZERO,
                            e.to_string(),
                        ));
                    }
                }
            }
        }

        if ctx.is_json() {
            let batch = BatchResult::from_results(results, start.elapsed());
            output::emit_json(&batch);
        } else {
            print_status("Build all completed", ctx);
        }
    }

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

/// Internal wheel build that returns a simpler error type for parallel execution.
fn build_wheel_internal(ctx: &ExecContext) -> std::result::Result<(), String> {
    build_wheel(ctx).map_err(|e| e.to_string())
}

/// Internal docker build that returns a simpler error type for parallel execution.
fn build_docker_internal(service: &str, no_cache: bool, ctx: &ExecContext) -> std::result::Result<(), String> {
    build_docker(service.to_string(), no_cache, ctx).map_err(|e| e.to_string())
}
