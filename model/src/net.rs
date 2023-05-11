//! API for creating and managing exe-units networking.
//!
//!  - VPN - Virtual Private Networks
//!  - Internet Gateway
//!
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};

use crate::NodeId;

#[doc(hidden)]
pub const NET_API_PATH: &str = "/net-api/v1";

/// Represents a new network configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNetwork {
    /// The IP address for the network.
    pub ip: Ipv4Addr,

    /// The subnet mask for the network.
    pub mask: Ipv4Addr,

    /// The gateway IP address for the network. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub gateway: Option<Ipv4Addr>,
}

/// Represents a network configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    /// The ID of the network.
    pub id: String,

    /// The IP address for the network.
    pub ip: Ipv4Addr,

    /// The subnet mask for the network.
    pub mask: Ipv4Addr,

    /// The gateway IP address for the network. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub gateway: Option<Ipv4Addr>,
}

/// Represents single VPN node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Provider node id assigned to given ip.
    pub id: NodeId,
    /// Container IP in given network.
    pub ip: IpAddr,
    /// Activity id for case when mode than one container runs on given provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
}

/// Requestor's address in a virtual private network.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// The IP address of the requestor in the virtual private network.
    pub ip: Ipv4Addr,
}

/// Represents a network connection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    /// The protocol of the connection.
    pub protocol: u16,

    /// The local IP address of the connection.
    pub local_ip: String,

    /// The local port number of the connection.
    pub local_port: u16,

    /// The remote IP address of the connection.
    pub remote_ip: String,

    /// The remote port number of the connection.
    pub remote_port: u16,
}

/// Represents proxy binding.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proxy {
    /// Local address to listen-on.
    pub bind_address: SocketAddrV4,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    use std::net::Ipv4Addr;

    #[test]
    fn test_serialize() {
        const MASK: Ipv4Addr = Ipv4Addr::new(255, 255, 255, 0);

        let json = serde_json::to_string(&Network {
            id: "id".to_string(),
            ip: "192.168.0.1".parse().unwrap(),
            mask: MASK,
            gateway: None,
        })
        .unwrap();

        eprintln!("json={}", json);
    }

    #[test]
    fn test_serialize_proxy() {
        let json = serde_json::to_string(&Proxy {
            bind_address: SocketAddrV4::new("127.0.0.1".parse().unwrap(), 9000),
        })
        .unwrap();

        let _: Proxy = serde_json::from_str(r#"{"bindAddress": "127.0.0.1:9000"}"#).unwrap();
    }
}
