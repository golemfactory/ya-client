use crate::activity::ExeScriptCommand;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeEvent {
    pub batch_id: String,
    pub index: usize,
    pub timestamp: NaiveDateTime,
    pub kind: RuntimeEventKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RuntimeEventKind {
    Started {
        command: ExeScriptCommand
    },
    Finished {
        return_code: i32,
        message: Option<String>,
    },
    StdOut(String),
    StdErr(String),
}
