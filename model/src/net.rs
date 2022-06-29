use serde::{Deserialize, Serialize};

use crate::NodeId;

pub const NET_API_PATH: &str = "/net-api/v2";
pub const NET_API_V1_VPN_PATH: &str = "/net-api/v1";
pub const NET_API_V2_NET_PATH: &str = "/net-api/v2/net";
pub const NET_API_V2_VPN_PATH: &str = "/net-api/v2/vpn";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub node_id: NodeId,
    pub listen_ip: Option<String>,
    pub public_ip: Option<String>,
    pub sessions: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub id: String,
    pub ip: String,
    pub mask: String,
    pub gateway: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNetwork {
    pub ip: String,
    pub mask: Option<String>,
    pub gateway: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub ip: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub ip: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub protocol: u16,
    pub local_ip: String,
    pub local_port: u16,
    pub remote_ip: String,
    pub remote_port: u16,
}
