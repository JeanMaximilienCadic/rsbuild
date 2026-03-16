//! Error types for rsbuild operations.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during rsbuild operations.
#[derive(Error, Debug)]
pub enum RsbuildError {
    /// A required external tool is not installed.
    #[error("Required tool '{tool}' is not installed. {hint}")]
    ToolNotFound {
        tool: String,
        hint: String,
    },

    /// Command execution failed with details.
    #[error("Command failed: {command}\n  Exit code: {code}\n  Error: {message}")]
    CommandFailed {
        command: String,
        code: i32,
        message: String,
    },

    /// Command execution failed to start.
    #[error("Failed to execute command: {0}")]
    ExecutionFailed(String),

    /// File or directory not found.
    #[error("Path not found: {path}")]
    PathNotFound {
        path: PathBuf,
    },

    /// IO error with context.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// UTF-8 conversion error.
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

/// Result type alias for rsbuild operations.
pub type Result<T> = std::result::Result<T, RsbuildError>;
