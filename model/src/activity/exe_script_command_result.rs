use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the result of an single [crate::activity::ExeScriptCommand]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExeScriptCommandResult {
    /// The index of the command.
    pub index: u32,
    /// The result of the command.
    pub result: CommandResult,
    /// The stdout output of the command, if any.
    pub stdout: Option<CommandOutput>,
    /// The stderr output of the command, if any.
    pub stderr: Option<CommandOutput>,
    /// Error message related to the command result, if any.
    pub message: Option<String>,
    /// Indicates whether the command is part of a finished batch.
    pub is_batch_finished: bool,
    /// The event date of the command execution.
    pub event_date: DateTime<Utc>,
}

/// Represents the output of a command.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandOutput {
    /// Output represented as a string.
    Str(String),
    /// Output represented as binary data.
    Bin(Vec<u8>),
}

/// Represents the result of a command execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommandResult {
    /// The command executed successfully.
    Ok,
    /// An error occurred during command execution.
    Error,
}
