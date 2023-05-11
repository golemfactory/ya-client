use crate::activity::{CommandOutput, ExeScriptCommand};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents an event related to the runtime.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeEvent {
    /// The batch ID associated with the event.
    pub batch_id: String,

    /// The index of the event.
    pub index: usize,

    /// The timestamp of the event.
    pub timestamp: NaiveDateTime,

    /// The kind of the runtime event.
    pub kind: RuntimeEventKind,
}

impl RuntimeEvent {
    /// Creates a new `RuntimeEvent` with the specified parameters.
    pub fn new(batch_id: String, index: usize, kind: RuntimeEventKind) -> Self {
        RuntimeEvent {
            batch_id,
            index,
            kind,
            timestamp: Utc::now().naive_utc(),
        }
    }

    /// Creates a new `RuntimeEvent` for a started command.
    pub fn started(batch_id: String, idx: usize, command: ExeScriptCommand) -> Self {
        Self::new(batch_id, idx, RuntimeEventKind::Started { command })
    }

    /// Creates a new `RuntimeEvent` for a finished command.
    pub fn finished(
        batch_id: String,
        idx: usize,
        return_code: i32,
        message: Option<String>,
    ) -> Self {
        Self::new(
            batch_id,
            idx,
            RuntimeEventKind::Finished {
                return_code,
                message,
            },
        )
    }

    /// Creates a new `RuntimeEvent` for stdout output.
    pub fn stdout(batch_id: String, idx: usize, out: CommandOutput) -> Self {
        Self::new(batch_id, idx, RuntimeEventKind::StdOut(out))
    }

    /// Creates a new `RuntimeEvent` for stderr output.
    pub fn stderr(batch_id: String, idx: usize, out: CommandOutput) -> Self {
        Self::new(batch_id, idx, RuntimeEventKind::StdErr(out))
    }
}

/// Represents the kind of a runtime event.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeEventKind {
    /// Indicates a started command.
    Started {
        /// The executed command.
        command: ExeScriptCommand,
    },
    /// Indicates a finished command.
    Finished {
        /// The return code of the command.
        return_code: i32,
        /// Additional message related to the command execution.
        message: Option<String>,
    },
    /// Indicates stdout output.
    StdOut(CommandOutput),
    /// Indicates stderr output.
    StdErr(CommandOutput),
}
