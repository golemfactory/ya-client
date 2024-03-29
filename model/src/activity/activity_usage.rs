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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityUsage {
    /// Current usage vector
    #[serde(rename = "currentUsage", skip_serializing_if = "Option::is_none")]
    pub current_usage: Option<Vec<f64>>,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}
