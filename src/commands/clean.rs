use crate::error::Result;
use crate::executor::exec;

pub fn run() -> Result<()> {
    let commands = [
        "rm -rf build",
        "rm -rf $(find . -type d -iname '*egg-info*' 2>/dev/null)",
        "rm -rf $(find . -type d -iname '*pycache*' 2>/dev/null)",
        "rm -rf $(find . -type d -iname '*.ipynb_checkpoints*' 2>/dev/null)",
    ];

    for cmd in commands {
        // Ignore errors for clean commands as files may not exist
        let _ = exec(cmd, false);
    }

    Ok(())
}
