use crate::error::Result;
use crate::executor::{exec, exec_commands};

pub fn run(package: &str) -> Result<()> {
    let build_dir = format!("/tmp/bin/._rsbuild-{}", package);

    // Cleanup and setup
    let _ = exec(&format!("rm -rf {}", build_dir), false);
    exec(&format!("mkdir -p {}/dist/legacy", build_dir), false)?;
    exec(&format!("cp requirements.txt {}", build_dir), false)?;
    exec(&format!("cp setup.cfg {}", build_dir), false)?;
    exec(&format!("cp setup.py {}", build_dir), false)?;

    // Cythonize and package
    exec_commands(&[
        &format!("cythonize -a -i {}", package),
        "rsbuild clean",
        &format!("rm -f $(find ./{} -type f -iname '*.c' 2>/dev/null)", package),
        &format!("find {} -type f -iname '*.so' > so_files", package),
        &format!("rsync -av --files-from=so_files ./ {}", build_dir),
        &format!("rm -f $(find ./{} -type f -iname '*.so' 2>/dev/null)", package),
        &format!("cd {} && rsbuild build wheel", build_dir),
        "rsbuild clean",
        "rm -f so_files",
    ])?;

    // Cleanup artifacts
    let _ = exec("rm -f $(find . -type f -iname '*.html' 2>/dev/null)", false);
    exec(&format!("mv {}/dist/*.whl dist/", build_dir), false)?;
    let _ = exec(&format!("rm -rf {}", build_dir), false);

    Ok(())
}
