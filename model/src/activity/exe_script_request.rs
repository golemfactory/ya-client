/*
 * Yagna Activity API
 *
 * It conforms with capability level 1 of the [Activity API specification](https://golem-network.gitbook.io/golem-internal-documentation-test/golem-activity-protocol/golem-activity-api).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

/// Represents a request for executing a script.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExeScriptRequest {
    #[serde(rename = "text")]
    /// The script text to execute.
    pub text: String,
}

impl ExeScriptRequest {
    /// Creates a new `ExeScriptRequest` with the specified script text.
    pub fn new(text: String) -> ExeScriptRequest {
        ExeScriptRequest { text }
    }
}
