//! Progress bar utilities.

use crate::cli::ExecContext;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Default spinner style.
const SPINNER_TEMPLATE: &str = "{spinner:.cyan} {msg}";

/// Default progress bar style.
const BAR_TEMPLATE: &str = "{spinner:.cyan} [{bar:30.cyan/dim}] {pos}/{len} {msg}";

/// Spinner tick interval in milliseconds.
const TICK_INTERVAL_MS: u64 = 80;

/// Create an indeterminate spinner for a task.
/// Returns None if progress is disabled (quiet/JSON mode).
pub fn create_spinner(ctx: &ExecContext, message: impl Into<String>) -> Option<ProgressBar> {
    let mp = ctx.progress.as_ref()?;

    let pb = mp.add(ProgressBar::new_spinner());
    pb.set_style(
        ProgressStyle::default_spinner()
            .template(SPINNER_TEMPLATE)
            .expect("valid template")
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    pb.set_message(message.into());
    pb.enable_steady_tick(Duration::from_millis(TICK_INTERVAL_MS));

    Some(pb)
}

/// Create a progress bar with known length.
/// Returns None if progress is disabled (quiet/JSON mode).
pub fn create_bar(ctx: &ExecContext, len: u64, message: impl Into<String>) -> Option<ProgressBar> {
    let mp = ctx.progress.as_ref()?;

    let pb = mp.add(ProgressBar::new(len));
    pb.set_style(
        ProgressStyle::default_bar()
            .template(BAR_TEMPLATE)
            .expect("valid template")
            .progress_chars("=> "),
    );
    pb.set_message(message.into());
    pb.enable_steady_tick(Duration::from_millis(TICK_INTERVAL_MS));

    Some(pb)
}

/// Finish a progress bar with success status.
pub fn finish_success(pb: Option<ProgressBar>, message: impl Into<String>) {
    if let Some(pb) = pb {
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{msg}")
                .expect("valid template"),
        );
        pb.finish_with_message(format!("\u{2714} {}", message.into()));
    }
}

/// Finish a progress bar with error status.
pub fn finish_error(pb: Option<ProgressBar>, message: impl Into<String>) {
    if let Some(pb) = pb {
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{msg}")
                .expect("valid template"),
        );
        pb.finish_with_message(format!("\u{2718} {}", message.into()));
    }
}

/// Finish a progress bar with warning/skip status.
pub fn finish_warning(pb: Option<ProgressBar>, message: impl Into<String>) {
    if let Some(pb) = pb {
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{msg}")
                .expect("valid template"),
        );
        pb.finish_with_message(format!("\u{26A0} {}", message.into()));
    }
}

/// Update the progress bar message.
pub fn set_message(pb: &Option<ProgressBar>, message: impl Into<String>) {
    if let Some(pb) = pb {
        pb.set_message(message.into());
    }
}

/// Increment the progress bar by one.
pub fn inc(pb: &Option<ProgressBar>) {
    if let Some(pb) = pb {
        pb.inc(1);
    }
}
