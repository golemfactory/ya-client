/*
 * Yagna Activity API
 *
 * It conforms with capability level 1 of the [Activity API specification](https://docs.google.com/document/d/1BXaN32ediXdBHljEApmznSfbuudTU8TmvOmHKl0gmQM).
 *
 * The version of the OpenAPI document: v1
 *
 *
 */

use crate::activity::ExeScriptCommandState;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExeScriptCommand {
    Deploy {},
    Start {
        #[serde(default)]
        args: Vec<String>,
    },
    Run {
        entry_point: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        capture: Option<Capture>,
    },
    Transfer {
        from: String,
        to: String,
        #[serde(flatten)]
        args: TransferArgs,
    },
    Terminate {},
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Capture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<CaptureMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<CaptureMode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CaptureMode {
    AtEnd {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        part: Option<CapturePart>,
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<CaptureFormat>,
    },
    Stream {
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<usize>,
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<CaptureFormat>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaptureFormat {
    #[serde(alias = "string")]
    Str,
    #[serde(alias = "binary")]
    Bin,
}

impl Default for CaptureFormat {
    fn default() -> Self {
        CaptureFormat::Str
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CapturePart {
    Head(usize),
    Tail(usize),
    HeadTail(usize),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferArgs {
    pub format: Option<String>,
    pub depth: Option<usize>,
    pub fileset: Option<FileSet>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileSet {
    Pattern(SetEntry<String>),
    Object(SetEntry<SetObject>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SetObject {
    pub desc: Option<String>,
    pub includes: Option<SetEntry<String>>,
    pub excludes: Option<SetEntry<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetEntry<T> {
    Single(T),
    Multiple(Vec<T>),
}

impl From<ExeScriptCommand> for ExeScriptCommandState {
    fn from(cmd: ExeScriptCommand) -> Self {
        match cmd {
            ExeScriptCommand::Deploy { .. } => ExeScriptCommandState {
                command: "Deploy".to_string(),
                progress: None,
                params: None,
            },
            ExeScriptCommand::Start { args } => ExeScriptCommandState {
                command: "Start".to_string(),
                progress: None,
                params: Some(args),
            },
            ExeScriptCommand::Run {
                entry_point,
                mut args,
                capture: _,
            } => ExeScriptCommandState {
                command: "Run".to_string(),
                progress: None,
                params: Some({
                    args.insert(0, entry_point);
                    args
                }),
            },
            ExeScriptCommand::Transfer { from, to, .. } => ExeScriptCommandState {
                command: "Transfer".to_string(),
                progress: None,
                params: Some(vec![from, to]),
            },
            ExeScriptCommand::Terminate {} => ExeScriptCommandState {
                command: "Terminate".to_string(),
                progress: None,
                params: None,
            },
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_transfers_parsing() {
        let command = r#"
        [ {"transfer": {
            "from": "http://some-site/data.zip",
            "to": "container:/app//in/data.zip"
          } },
          {"transfer": {
            "from": "http://some-site/data.zip",
            "to": "container:/app//in/",
            "format": "zip"
           } },
          {"transfer": {
            "from": "http://some-site/data.zip",
            "to": "container:/app//in/",
            "depth": 0,
            "format": "zip.0",
            "fileset": "*.o"
           } },
           {"transfer": {
            "from": "http://some-site/data.zip",
            "to": "container:/app//in/",
            "format": "zip.0",
            "fileset": [
                {"includes": "out/*",
                 "excludes": ["*.tmp", ".gitignore"]
                },
                {"includes": "gen-spec/*"
                }
            ]
           } }


        ]"#;
        let _: Vec<super::ExeScriptCommand> = serde_json::from_str(command).unwrap();
    }
}
