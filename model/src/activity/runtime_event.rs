use crate::activity::{CommandOutput, ExeScriptCommand};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeEvent {
    pub batch_id: String,
    pub index: usize,
    pub timestamp: NaiveDateTime,
    pub kind: RuntimeEventKind,
}

impl RuntimeEvent {
    pub fn new(batch_id: String, index: usize, kind: RuntimeEventKind) -> Self {
        RuntimeEvent {
            batch_id,
            index,
            kind,
            timestamp: Utc::now().naive_utc(),
        }
    }

    pub fn started(batch_id: String, idx: usize, command: ExeScriptCommand) -> Self {
        Self::new(batch_id, idx, RuntimeEventKind::Started { command })
    }

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

    pub fn stdout(batch_id: String, idx: usize, out: CommandOutput) -> Self {
        Self::new(batch_id, idx, RuntimeEventKind::StdOut(out))
    }

    pub fn stderr(batch_id: String, idx: usize, out: CommandOutput) -> Self {
        Self::new(batch_id, idx, RuntimeEventKind::StdErr(out))
    }

    pub fn progress(batch_id: String, idx: usize, progress: CommandProgress) -> Self {
        Self::new(batch_id, idx, RuntimeEventKind::Progress(progress))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeEventKind {
    Started {
        command: ExeScriptCommand,
    },
    Finished {
        return_code: i32,
        message: Option<String>,
    },
    StdOut(CommandOutput),
    StdErr(CommandOutput),
    Progress(CommandProgress),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct CommandProgress {
    /// Steps are counted starting from 0. That means that first step from 4-steps tasks
    /// will report 0/4. Task is finished when counter reaches 4/4.
    pub step: (usize, usize),
    /// May contain additional arbitrary information about, what is happening with the task
    /// like retrying transfer or that image was deployed from cache.
    pub message: Option<String>,
    /// Granular progress of currently executed step. The first element describes current
    /// progress, the second the size of the whole task, which can be unknown.
    pub progress: (u64, Option<u64>),
    pub unit: Option<String>,
}
