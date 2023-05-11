use crate::NodeId;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

pub const NET_API_PATH: &str = "/net-api/v2";
pub const NET_API_V2_NET_PATH: &str = "/net-api/v2/net";
pub const NET_API_V2_VPN_PATH: &str = "/net-api/v2/vpn";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub node_id: NodeId,
    pub listen_ip: Option<IpAddr>,
    pub public_ip: Option<IpAddr>,
    pub sessions: usize,
}
