//! Clean command implementation.

use crate::cli::ExecContext;
use crate::error::Result;
use crate::executor::{exec_ignore_error, print_status};

/// Execute the clean command.
pub fn run(all: bool, ctx: &ExecContext) -> Result<()> {
    print_status("Cleaning build artifacts", ctx);

    // Remove standard build directories
    exec_ignore_error("rm -rf build", ctx);

    // Remove Python artifacts using find
    exec_ignore_error("find . -type d -name '*egg-info*' -exec rm -rf {} + 2>/dev/null", ctx);
    exec_ignore_error("find . -type d -name '__pycache__' -exec rm -rf {} + 2>/dev/null", ctx);
    exec_ignore_error("find . -type d -name '.ipynb_checkpoints' -exec rm -rf {} + 2>/dev/null", ctx);
    exec_ignore_error("find . -type f -name '*.pyc' -delete 2>/dev/null", ctx);

    if all {
        print_status("Cleaning Rust target directory", ctx);
        exec_ignore_error("rm -rf target", ctx);
    }

    print_status("Clean completed", ctx);
    Ok(())
}
