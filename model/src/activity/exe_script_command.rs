use crate::activity::ExeScriptCommandState;
use crate::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

/// Exe-unit control commands.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExeScriptCommand {
    #[doc(hidden)]
    Sign {},
    /// Configures the container.
    /// Downloads the image and verifies the state.
    Deploy {
        /// For exe-units with network abstraction defines network
        /// list of network interfaces.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        net: Vec<Network>,
        /// Static hostname mapping.
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        hosts: HashMap<String, IpAddr>, // hostname -> IP
    },
    /// Starts container.
    Start {
        /// Entry point arguments.
        ///
        /// - `{"start": {"args": [] }}` - runs entry point with empty arguments.
        /// - `{"start": {}}` - runs entry point with default arguments.
        ///
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        args: Option<Vec<String>>,
    },
    /// Executes process
    Run {
        /// Binary path inside container of executable to run.
        entry_point: String,
        /// Arguments to pass to the executable.
        #[serde(default)]
        args: Vec<String>,
        /// Specifies whether and how to capture the output of the executed process.
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        capture: Option<Capture>,
    },
    /// Transfers files or directories between the host and the container or
    /// between containers.
    ///
    /// ## Example
    ///
    /// ```json
    /// {"transfer": {
    ///   "from": "http://some-site/data.zip",
    ///   "to": "container:/app//in/data.zip"
    ///   }
    ///  }
    ///
    Transfer {
        /// Source path.
        from: String,
        /// Destination path.
        to: String,
        /// Transfer args.
        #[serde(flatten)]
        args: TransferArgs,
    },
    /// Transfers files or directories between the host and the container.
    Terminate {},
}

/// Reference to container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum NetworkActivityRef {
    /// For single container on single node
    Node(NodeId),
    /// More specific addressing by pair node id and activity id.
    Activity(NodeId, String),
}

/// Network specification for deploy command.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    /// Network id, created by network api.
    pub id: String,
    /// Network ip addr for example: 192.168.0.0
    pub ip: Ipv4Addr,
    /// Network mask. For example 255.255.0.0
    pub mask: Option<Ipv4Addr>,
    /// Default gateway.
    pub gateway: Option<Ipv4Addr>,
    /// Ip for given node. For example 192.168.0.1
    pub node_ip: Ipv4Addr,
    /// Static routing rules.
    #[serde(default)]
    pub nodes: HashMap<Ipv4Addr, NetworkActivityRef>, // IP -> NodeId
}

/// Represents the capture configuration for stdout and stderr.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Capture {
    /// Specifies the capture mode for stdout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<CaptureMode>,
    /// Specifies the capture mode for stderr.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<CaptureMode>,
}

/// Represents the capture mode for stdout or stderr.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CaptureMode {
    /// Captures the output at the end of the process.
    AtEnd {
        /// Specifies the capture part (head, tail, or head and tail).
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(flatten)]
        part: Option<CapturePart>,
        /// Specifies if captured output is binary or text.
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<CaptureFormat>,
    },
    /// Captures the output as a continuous stream.
    Stream {
        /// If not None it limits capture buffer. overflowed bytes will be discarded.
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<usize>,
        /// Specifies if captured output is binary or text.
        #[serde(skip_serializing_if = "Option::is_none")]
        format: Option<CaptureFormat>,
    },
}

/// Represents the format for captured output.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaptureFormat {
    /// UTF8 string output
    #[default]
    #[serde(alias = "string")]
    Str,
    /// Capture as bytes.
    #[serde(alias = "binary")]
    Bin,
}

/// Represents the capture part for the output.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CapturePart {
    /// Captures the first n bytes of the output.
    Head(usize),
    /// Captures the last n bytes of the output.
    Tail(usize),
    /// Captures both the n/2 first bytes and n/2 last bytes of the output.
    HeadTail(usize),
}

/// Represents the transfer arguments for the file transfer command.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TransferArgs {
    /// Specifies the format for the file transfer.
    ///
    /// - `tar` packs fileset into tar format.
    /// - `tar.bz2` packs fileset into tar format compressed with bzip2 algorithm.
    /// - `tar.gz` packs fileset into tar format compressed with gzip algorithm.
    /// - `tar.xz` packs fileset into tar format compressed with xz algorithm.
    /// - `zip` packs fleset into zip format compressed with defalate algorithm.
    /// - `zip.0` packs fleset into zip format without compression (store only).
    ///
    pub format: Option<String>,
    /// Specifies the scanning depth of the file transfer.
    pub depth: Option<usize>,
    /// Specifies the file selection rules for the transfer.
    pub fileset: Option<FileSet>,
}

/// Represents a file set for the file transfer command.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileSet {
    /// Simple includes pattern.
    Pattern(SetEntry<String>),
    /// Complex pattern with includes/excludes
    Object(SetEntry<SetObject>),
}

/// Represents a file set selection rule.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SetObject {
    /// Provides a description for the file set selection rule.
    pub desc: Option<String>,
    /// Specifies the includes rules.
    pub includes: Option<SetEntry<String>>,
    /// Specifies the excludes rules.
    pub excludes: Option<SetEntry<String>>,
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetEntry<T> {
    Single(T),
    Multiple(Vec<T>),
}

impl From<ExeScriptCommand> for ExeScriptCommandState {
    fn from(cmd: ExeScriptCommand) -> Self {
        match cmd {
            ExeScriptCommand::Sign { .. } => ExeScriptCommandState {
                command: "Sign".to_string(),
                progress: None,
                params: None,
            },
            ExeScriptCommand::Deploy { .. } => ExeScriptCommandState {
                command: "Deploy".to_string(),
                progress: None,
                params: None,
            },
            ExeScriptCommand::Start { args } => ExeScriptCommandState {
                command: "Start".to_string(),
                progress: None,
                params: args,
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
