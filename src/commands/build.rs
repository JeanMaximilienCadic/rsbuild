//! Build command implementations.

use crate::cli::{BuildTarget, CargoBuildMode, ExecContext};
use crate::error::Result;
use crate::executor::{check_tool, exec, exec_commands, exec_ignore_error, print_status, read_output_str};

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

/// Build a Python wheel.
fn build_wheel(ctx: &ExecContext) -> Result<()> {
    check_tool("pip")?;

    print_status("Building Python wheel", ctx);

    // Move existing wheels to legacy (ignore if none exist)
    exec_ignore_error("mkdir -p dist/legacy", ctx);
    exec_ignore_error("mv dist/*.whl dist/legacy/ 2>/dev/null", ctx);

    exec_commands(
        &[
            "pip wheel . -w dist --no-deps",
            "rsbuild clean",
        ],
        ctx,
    )?;

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
    print_status("Building all targets", ctx);

    // Build wheel first
    if let Err(e) = build_wheel(ctx) {
        crate::executor::print_warning(&format!("Wheel build failed: {}", e), ctx);
    }

    // Build common docker services if docker-compose.yml exists
    if std::path::Path::new("docker-compose.yml").exists()
        || std::path::Path::new("compose.yml").exists()
    {
        for service in &["vanilla", "sandbox"] {
            if let Err(e) = build_docker(service.to_string(), false, ctx) {
                crate::executor::print_warning(&format!("Docker build '{}' failed: {}", service, e), ctx);
            }
        }
    }

    print_status("Build all completed", ctx);
    Ok(())
}
