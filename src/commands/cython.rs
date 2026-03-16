//! Cython compilation command.

use crate::cli::ExecContext;
use crate::error::Result;
use crate::executor::{check_tool, exec, exec_commands, exec_ignore_error, print_status};
use std::path::Path;

/// Temporary build directory prefix.
const BUILD_DIR_PREFIX: &str = "/tmp/rsbuild-cython";

/// Execute the cython command.
pub fn run(package: &str, ctx: &ExecContext) -> Result<()> {
    // Check required tools
    check_tool("cythonize")?;
    check_tool("rsync")?;
    check_tool("pip")?;

    // Validate package exists
    if !Path::new(package).is_dir() {
        return Err(crate::error::RsbuildError::PathNotFound {
            path: package.into(),
        });
    }

    let build_dir = format!("{}-{}", BUILD_DIR_PREFIX, package);

    print_status(&format!("Compiling Cython package: {}", package), ctx);

    // Setup build directory
    setup_build_dir(&build_dir, ctx)?;

    // Compile Cython modules
    compile_cython(package, &build_dir, ctx)?;

    // Package and cleanup
    finalize_build(package, &build_dir, ctx)?;

    print_status(&format!("Cython package '{}' built successfully", package), ctx);
    Ok(())
}

/// Set up the temporary build directory.
fn setup_build_dir(build_dir: &str, ctx: &ExecContext) -> Result<()> {
    // Clean any previous build
    exec_ignore_error(&format!("rm -rf {}", build_dir), ctx);

    // Create directory structure
    exec(&format!("mkdir -p {}/dist/legacy", build_dir), ctx)?;

    // Copy required files if they exist
    for file in &["requirements.txt", "setup.cfg", "setup.py", "pyproject.toml"] {
        if Path::new(file).exists() {
            exec_ignore_error(&format!("cp {} {}", file, build_dir), ctx);
        }
    }

    Ok(())
}

/// Compile Cython modules.
fn compile_cython(package: &str, build_dir: &str, ctx: &ExecContext) -> Result<()> {
    exec_commands(
        &[
            // Compile Cython to .so files
            &format!("cythonize -a -i {}", package),
            // Clean intermediate files
            "rsbuild clean",
            // Remove generated C files
            &format!("find ./{} -type f -name '*.c' -delete 2>/dev/null || true", package),
            // List .so files for packaging
            &format!("find {} -type f -name '*.so' > /tmp/rsbuild_so_files", package),
            // Copy .so files to build directory
            &format!("rsync -av --files-from=/tmp/rsbuild_so_files ./ {}", build_dir),
            // Remove .so files from source
            &format!("find ./{} -type f -name '*.so' -delete 2>/dev/null || true", package),
        ],
        ctx,
    )
}

/// Build wheel and clean up.
fn finalize_build(_package: &str, build_dir: &str, ctx: &ExecContext) -> Result<()> {
    // Build wheel in the build directory
    exec(&format!("cd {} && rsbuild build wheel", build_dir), ctx)?;

    // Clean up
    exec_commands(&["rsbuild clean", "rm -f /tmp/rsbuild_so_files"], ctx)?;

    // Remove HTML annotation files
    exec_ignore_error("find . -type f -name '*.html' -delete 2>/dev/null", ctx);

    // Move wheel to dist
    exec(&format!("mkdir -p dist && mv {}/dist/*.whl dist/", build_dir), ctx)?;

    // Clean up build directory
    exec_ignore_error(&format!("rm -rf {}", build_dir), ctx);

    Ok(())
}
