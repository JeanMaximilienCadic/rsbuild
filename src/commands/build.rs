use crate::cli::{BuildTarget, CargoBuildMode};
use crate::error::Result;
use crate::executor::{exec, exec_commands, read_output_str};

pub fn run(target: BuildTarget) -> Result<()> {
    match target {
        BuildTarget::Wheel => build_wheel(),
        BuildTarget::All => build_all(),
        BuildTarget::Vanilla => build_docker_compose("vanilla"),
        BuildTarget::Sandbox => build_docker_compose("sandbox"),
        BuildTarget::Cargo { mode } => cargo_build(mode),
        BuildTarget::Docker { service } => build_docker_compose(&service),
    }
}

fn cargo_build(mode: CargoBuildMode) -> Result<()> {
    let os = read_output_str("uname")?;
    let arch = read_output_str("uname -m")?;
    let target_dir = format!("target/{}/{}", os, arch);

    let arg = match mode {
        CargoBuildMode::Debug => "",
        CargoBuildMode::Release => "--release",
    };

    let cmd = format!("cargo build {} --target-dir {}", arg, target_dir);
    exec(&cmd, true)?;
    Ok(())
}

fn build_wheel() -> Result<()> {
    exec("mv dist/*.whl dist/legacy", false).ok();
    exec_commands(&["pip wheel . -w dist --no-deps", "rsbuild clean"])
}

fn build_docker_compose(service: &str) -> Result<()> {
    exec_commands(&[&format!("docker compose build {}", service)])
}

fn build_all() -> Result<()> {
    exec_commands(&[
        "rsbuild build wheel",
        "rsbuild build vanilla",
        "rsbuild build sandbox",
    ])
}
