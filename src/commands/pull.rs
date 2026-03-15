use crate::cli::PullTarget;
use crate::error::Result;
use crate::executor::exec_commands;

pub fn run(target: PullTarget) -> Result<()> {
    match target {
        PullTarget::All => pull_all(),
        PullTarget::Vanilla => pull_vanilla(),
        PullTarget::Sandbox => pull_sandbox(),
    }
}

fn pull_all() -> Result<()> {
    exec_commands(&["rsbuild pull vanilla", "rsbuild pull sandbox"])
}

fn pull_vanilla() -> Result<()> {
    exec_commands(&["docker compose pull vanilla"])
}

fn pull_sandbox() -> Result<()> {
    exec_commands(&["docker compose pull sandbox"])
}
