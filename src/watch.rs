//! File watching with automatic rebuild.

use crate::cli::{BuildTarget, ExecContext, WatchTarget};
use crate::commands;
use crate::error::{Result, RsbuildError};
use crate::executor::print_status;
use colored::Colorize;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// File patterns to watch for each target type.
fn file_patterns_for_target(target: WatchTarget) -> &'static [&'static str] {
    match target {
        WatchTarget::Wheel => &[".py", "pyproject.toml", "setup.py", "setup.cfg"],
        WatchTarget::Docker => &["Dockerfile", "docker-compose.yml", "compose.yml", ".dockerfile"],
        WatchTarget::Cython => &[".pyx", ".pxd", ".pxi"],
        WatchTarget::Cargo => &[".rs", "Cargo.toml", "Cargo.lock"],
    }
}

/// Default paths to watch for each target type.
fn default_paths_for_target(target: WatchTarget) -> Vec<String> {
    match target {
        WatchTarget::Wheel => vec![".".to_string()],
        WatchTarget::Docker => vec![".".to_string()],
        WatchTarget::Cython => vec![".".to_string()],
        WatchTarget::Cargo => vec!["src".to_string(), "Cargo.toml".to_string()],
    }
}

/// Check if a file change should trigger a rebuild.
fn should_trigger(path: &Path, target: WatchTarget) -> bool {
    let path_str = path.to_string_lossy();
    let patterns = file_patterns_for_target(target);

    // Skip hidden files and common ignore patterns
    if path_str.contains("/.") || path_str.contains("__pycache__") || path_str.contains("/target/")
    {
        return false;
    }

    patterns.iter().any(|pattern| {
        if pattern.starts_with('.') {
            // Extension match
            path_str.ends_with(pattern)
        } else {
            // Filename match
            path.file_name()
                .map(|n| n.to_string_lossy().contains(pattern))
                .unwrap_or(false)
        }
    })
}

/// Execute the appropriate build command for the target.
fn execute_build(target: WatchTarget, ctx: &ExecContext) -> Result<()> {
    match target {
        WatchTarget::Wheel => commands::build::run(BuildTarget::Wheel, ctx),
        WatchTarget::Docker => {
            // Build default docker service
            commands::build::run(
                BuildTarget::Docker {
                    service: "vanilla".to_string(),
                    no_cache: false,
                },
                ctx,
            )
        }
        WatchTarget::Cython => {
            // Look for common package names
            for pkg in &["src", "lib"] {
                if Path::new(pkg).is_dir() {
                    return commands::cython::run(pkg, ctx);
                }
            }
            Err(RsbuildError::PathNotFound {
                path: "package directory".into(),
            })
        }
        WatchTarget::Cargo => commands::build::run(
            BuildTarget::Cargo {
                mode: crate::cli::CargoBuildMode::Release,
            },
            ctx,
        ),
    }
}

/// Run the watch loop.
pub fn run(
    target: WatchTarget,
    debounce_ms: u64,
    additional_paths: Vec<String>,
    ctx: &ExecContext,
) -> Result<()> {
    let (tx, rx) = channel();

    // Create watcher with config
    let config = Config::default().with_poll_interval(Duration::from_millis(100));

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        config,
    )
    .map_err(|e| RsbuildError::ExecutionFailed(format!("Failed to create watcher: {}", e)))?;

    // Determine paths to watch
    let mut paths = additional_paths;
    if paths.is_empty() {
        paths = default_paths_for_target(target);
    }

    // Register paths with watcher
    for path in &paths {
        let path = Path::new(path);
        if path.exists() {
            watcher
                .watch(path, RecursiveMode::Recursive)
                .map_err(|e| {
                    RsbuildError::ExecutionFailed(format!(
                        "Failed to watch {}: {}",
                        path.display(),
                        e
                    ))
                })?;
        }
    }

    // Set up Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .map_err(|e| RsbuildError::ExecutionFailed(format!("Failed to set Ctrl+C handler: {}", e)))?;

    println!(
        "{} Watching for {:?} changes in: {}",
        "[watch]".bold().cyan(),
        target,
        paths.join(", ")
    );
    println!(
        "{} Press {} to stop",
        "[watch]".bold().cyan(),
        "Ctrl+C".bold()
    );

    // Initial build
    print_status("Running initial build...", ctx);
    if let Err(e) = execute_build(target, ctx) {
        eprintln!("{} Initial build failed: {}", "[error]".bold().red(), e);
    }

    let debounce = Duration::from_millis(debounce_ms);
    let mut last_build = Instant::now();

    // Watch loop
    while running.load(Ordering::SeqCst) {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => {
                // Check if any relevant file changed
                let relevant_paths: Vec<_> = event
                    .paths
                    .iter()
                    .filter(|p| should_trigger(p, target))
                    .collect();

                if !relevant_paths.is_empty() && last_build.elapsed() >= debounce {
                    println!();
                    for path in &relevant_paths {
                        println!(
                            "{} Changed: {}",
                            "[watch]".bold().yellow(),
                            path.display()
                        );
                    }

                    print_status("Rebuilding...", ctx);
                    match execute_build(target, ctx) {
                        Ok(_) => {
                            println!("{} Build completed", "[watch]".bold().green());
                        }
                        Err(e) => {
                            eprintln!("{} Build failed: {}", "[error]".bold().red(), e);
                        }
                    }

                    last_build = Instant::now();
                }
            }
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }

    println!("\n{} Watch mode stopped", "[watch]".bold().cyan());
    Ok(())
}
