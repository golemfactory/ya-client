/*
 * Yagna Market API
 *
 * The version of the OpenAPI document: 1.6.1
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyQuery {
    #[serde(rename = "issuerProperties", skip_serializing_if = "Option::is_none")]
    pub issuer_properties: Option<serde_json::Value>,
    #[serde(rename = "queryId", skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(rename = "queriedProperties")]
    pub queried_properties: Vec<String>,
}

impl PropertyQuery {
    pub fn new(queried_properties: Vec<String>) -> PropertyQuery {
        PropertyQuery {
            issuer_properties: None,
            query_id: None,
            queried_properties,
        }
    }
}
