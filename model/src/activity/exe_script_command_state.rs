use serde::{Deserialize, Serialize};

/// Represents the state of an `ExeScriptCommand`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExeScriptCommandState {
    #[serde(rename = "command")]
    /// The command name.
    pub command: String,

    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    /// The progress of the command execution, if available.
    pub progress: Option<String>,

    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    /// The parameters of the command.
    pub params: Option<Vec<String>>,
}

impl ExeScriptCommandState {
    /// Creates a new `ExeScriptCommandState` with the specified command name.
    pub fn new(command: String) -> ExeScriptCommandState {
        ExeScriptCommandState {
            command,
            progress: None,
            params: None,
        }
    }
}
