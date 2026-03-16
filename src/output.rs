//! JSON output formatting utilities.

use serde::Serialize;
use std::time::Duration;

/// Status of a command execution.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommandStatus {
    Success,
    Failed,
    Skipped,
}

/// Result of a command execution for JSON output.
#[derive(Clone, Debug, Serialize)]
pub struct CommandResult {
    pub command: String,
    pub status: CommandStatus,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl CommandResult {
    /// Create a successful command result.
    pub fn success(command: impl Into<String>, duration: Duration, output: Option<String>) -> Self {
        Self {
            command: command.into(),
            status: CommandStatus::Success,
            duration_ms: duration.as_millis() as u64,
            output,
            error: None,
        }
    }

    /// Create a failed command result.
    pub fn failed(command: impl Into<String>, duration: Duration, error: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            status: CommandStatus::Failed,
            duration_ms: duration.as_millis() as u64,
            output: None,
            error: Some(error.into()),
        }
    }

    /// Create a skipped command result.
    pub fn skipped(command: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            status: CommandStatus::Skipped,
            duration_ms: 0,
            output: None,
            error: Some(reason.into()),
        }
    }
}

/// Emit a JSON result to stdout.
pub fn emit_json<T: Serialize>(value: &T) {
    if let Ok(json) = serde_json::to_string_pretty(value) {
        println!("{}", json);
    }
}

/// Emit a single command result to stdout.
pub fn emit_result(result: &CommandResult) {
    emit_json(result);
}

/// Batch result for multiple command executions.
#[derive(Clone, Debug, Serialize)]
pub struct BatchResult {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub skipped: usize,
    pub duration_ms: u64,
    pub results: Vec<CommandResult>,
}

impl BatchResult {
    /// Create a new batch result from individual results.
    pub fn from_results(results: Vec<CommandResult>, total_duration: Duration) -> Self {
        let succeeded = results
            .iter()
            .filter(|r| matches!(r.status, CommandStatus::Success))
            .count();
        let failed = results
            .iter()
            .filter(|r| matches!(r.status, CommandStatus::Failed))
            .count();
        let skipped = results
            .iter()
            .filter(|r| matches!(r.status, CommandStatus::Skipped))
            .count();

        Self {
            total: results.len(),
            succeeded,
            failed,
            skipped,
            duration_ms: total_duration.as_millis() as u64,
            results,
        }
    }
}
