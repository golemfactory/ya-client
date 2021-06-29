use crate::error::Error;
use crate::model::net::*;
use crate::web::{WebClient, WebInterface};
use actix_codec::Framed;
use awc::http::Method;
use awc::ws::Codec;
use awc::BoxedSocket;
use std::ops::Not;

pub type Result<T> = std::result::Result<T, Error>;

/// Bindings for Requestor part of the Net API.
#[derive(Clone)]
pub struct NetRequestorApi {
    client: WebClient,
}

impl WebInterface for NetRequestorApi {
    const API_URL_ENV_VAR: &'static str = "YAGNA_NET_URL";
    const API_SUFFIX: &'static str = ya_client_model::net::NET_API_PATH;

    fn from_client(client: WebClient) -> Self {
        NetRequestorApi { client }
    }
}

impl NetRequestorApi {
    /// Retrieves requestor's virtual private networks.
    pub async fn get_networks(&self) -> Result<Vec<Network>> {
        self.client.get("net").send().json().await
    }

    /// Registers a new virtual private network overlay on the network.
    pub async fn create_network(&self, network: &Network) -> Result<()> {
        self.client.post("net").send_json(&network).json().await
    }

    /// Retrieves a requestor's virtual private network.
    pub async fn get_network(&self, network_id: &str) -> Result<Network> {
        let url = url_format!("net/{network_id}", network_id);
        self.client.get(&url).send().json().await
    }

    /// Unregisters an existing virtual private network overlay on the network.
    pub async fn remove_network(&self, network_id: &str) -> Result<()> {
        let url = url_format!("net/{network_id}", network_id);
        self.client.delete(&url).send().json().await
    }

    /// Retrieves requestor's addresses in a virtual private network.
    pub async fn get_addresses(&self, network_id: &str) -> Result<Vec<Address>> {
        let url = url_format!("net/{network_id}/addresses", network_id);
        self.client.get(&url).send().json().await
    }

    /// Assigns a new address of the requestor in an existing private network.
    pub async fn add_address(&self, network_id: &str, address: &Address) -> Result<()> {
        let url = url_format!("net/{network_id}/addresses", network_id);
        self.client.post(&url).send_json(&address).json().await
    }

    /// Retrieves nodes within a virtual private network.
    pub async fn get_nodes(&self, network_id: &str) -> Result<Vec<Node>> {
        let url = url_format!("net/{network_id}/nodes", network_id);
        self.client.get(&url).send().json().await
    }

    /// Registers a node in a virtual private network.
    pub async fn add_node(&self, network_id: &str, node: &Node) -> Result<()> {
        let url = url_format!("net/{network_id}/nodes", network_id);
        self.client.post(&url).send_json(&node).json().await
    }

    /// Unregisters an existing node in a virtual private network.
    pub async fn remove_node(&self, network_id: &str, node_id: &str) -> Result<()> {
        let url = url_format!("net/{network_id}/nodes/{node_id}", network_id, node_id);
        self.client.post(&url).send().json().await
    }

    /// Lists TCP connections
    pub async fn list_tcp(&self, network_id: &str) -> Result<Vec<Connection>> {
        let url = url_format!("net/{network_id}/tcp", network_id);
        self.client.get(&url).send().json().await
    }

    /// Creates a new TCP connection
    pub async fn connect_tcp(
        &self,
        network_id: &str,
        ip: &str,
        port: u16,
    ) -> Result<Framed<BoxedSocket, Codec>> {
        let url = url_format!("net/{network_id}/tcp/{ip}/{port}", network_id, ip, port);
        let (mut res, conn) = self.client.ws(&url).await?;

        let status = res.status();
        if status.is_success().not() && status.is_informational().not() {
            let body = res.body().limit(16384 as usize).await?;
            return Err(Error::HttpError {
                code: status,
                msg: String::from_utf8(body.to_vec())?,
                method: Method::GET,
                url,
            });
        }

        Ok(conn)
    }
}
