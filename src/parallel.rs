//! Parallel execution utilities using rayon.

use crate::cli::ExecContext;
use crate::output::{BatchResult, CommandResult};
use crate::progress::{self, finish_error, finish_success};
use rayon::prelude::*;
use std::sync::Mutex;

/// Result from executing a single task.
pub struct TaskResult<T> {
    pub name: String,
    pub result: Result<T, String>,
}

/// Execute tasks in parallel and collect results.
///
/// The `tasks` iterator yields `(name, task_fn)` pairs where `task_fn` is a closure
/// that performs the work and returns `Result<T, E>`.
///
/// Progress is tracked via a progress bar if enabled.
pub fn parallel_execute<T, I, F>(
    tasks: I,
    ctx: &ExecContext,
    description: &str,
) -> Vec<TaskResult<T>>
where
    T: Send,
    I: IntoIterator<Item = (String, F)>,
    F: Fn() -> Result<T, String> + Send + Sync,
{
    let tasks: Vec<_> = tasks.into_iter().collect();
    let total = tasks.len();

    if total == 0 {
        return Vec::new();
    }

    // Configure rayon thread pool based on jobs setting
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(ctx.jobs)
        .build()
        .expect("Failed to create thread pool");

    // Create progress bar
    let pb = progress::create_bar(ctx, total as u64, description.to_string());
    let results = Mutex::new(Vec::with_capacity(total));

    pool.install(|| {
        tasks.par_iter().for_each(|(name, task_fn)| {
            let result = task_fn();
            let task_result = TaskResult {
                name: name.clone(),
                result: result.map_err(|e| e.to_string()),
            };

            // Update progress
            if let Some(ref pb) = pb {
                pb.inc(1);
            }

            results.lock().unwrap().push(task_result);
        });
    });

    // Finish progress bar
    let collected: Vec<_> = results.into_inner().unwrap();
    let failed = collected.iter().filter(|r| r.result.is_err()).count();

    if failed > 0 {
        finish_error(pb, format!("{}: {} failed", description, failed));
    } else {
        finish_success(pb, format!("{}: {} completed", description, total));
    }

    collected
}

/// Execute tasks sequentially (fallback for jobs=1).
pub fn sequential_execute<T, I, F>(
    tasks: I,
    ctx: &ExecContext,
    description: &str,
) -> Vec<TaskResult<T>>
where
    T: Send,
    I: IntoIterator<Item = (String, F)>,
    F: Fn() -> Result<T, String>,
{
    let tasks: Vec<_> = tasks.into_iter().collect();
    let total = tasks.len();

    if total == 0 {
        return Vec::new();
    }

    // Create progress bar
    let pb = progress::create_bar(ctx, total as u64, description.to_string());
    let mut results = Vec::with_capacity(total);

    for (name, task_fn) in tasks {
        let result = task_fn();
        let task_result = TaskResult {
            name,
            result: result.map_err(|e| e.to_string()),
        };

        // Update progress
        progress::inc(&pb);
        results.push(task_result);
    }

    // Finish progress bar
    let failed = results.iter().filter(|r| r.result.is_err()).count();

    if failed > 0 {
        finish_error(pb, format!("{}: {} failed", description, failed));
    } else {
        finish_success(pb, format!("{}: {} completed", description, total));
    }

    results
}

/// Execute tasks using parallel or sequential strategy based on context.
pub fn execute<T, I, F>(tasks: I, ctx: &ExecContext, description: &str) -> Vec<TaskResult<T>>
where
    T: Send,
    I: IntoIterator<Item = (String, F)>,
    F: Fn() -> Result<T, String> + Send + Sync,
{
    if ctx.is_parallel() {
        parallel_execute(tasks, ctx, description)
    } else {
        sequential_execute(tasks, ctx, description)
    }
}

/// Convert task results to JSON batch result.
pub fn to_batch_result(
    results: &[TaskResult<String>],
    total_duration: std::time::Duration,
) -> BatchResult {
    let command_results: Vec<CommandResult> = results
        .iter()
        .map(|r| match &r.result {
            Ok(output) => CommandResult::success(
                &r.name,
                std::time::Duration::ZERO,
                if output.is_empty() {
                    None
                } else {
                    Some(output.clone())
                },
            ),
            Err(e) => CommandResult::failed(&r.name, std::time::Duration::ZERO, e),
        })
        .collect();

    BatchResult::from_results(command_results, total_duration)
}
