use serde::{Deserialize, Serialize};

pub const NET_API_PATH: &str = "/net-api/v1/";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNetwork {
    pub network: Network,
    pub requestor_address: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub id: String,
    pub address: String,
    pub mask: Option<String>,
    pub gateway: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub address: String,
}
