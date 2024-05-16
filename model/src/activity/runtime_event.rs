use std::time::Duration;

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

    pub fn deploy_progress(batch_id: String, idx: usize, progress: DeployProgress) -> Self {
        Self::new(batch_id, idx, RuntimeEventKind::DeployProgressUpdate(progress))
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
    DeployProgressUpdate(DeployProgress),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeployProgress {
    DeployFromCache,
    DownloadingImage,
    DownloadProgress(u64, Option<u64>),
    DownloadError(String),
    DownloadRetry(String, Duration),
    DownloadFinished,
}
