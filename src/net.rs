use crate::error::Error;
use crate::model::net::*;
use crate::web::{WebClient, WebInterface};
use actix_codec::Framed;
use awc::ws::Codec;
use awc::{BoxedSocket, ClientResponse};

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
    /// Registers a new virtual private network overlay on the network.
    pub async fn create_network(&self, create: &CreateNetwork) -> Result<()> {
        self.client.post("net").send_json(&create).json().await
    }

    /// Unregisters an existing virtual private network overlay on the network.
    pub async fn remove_network(&self, network_id: &str) -> Result<()> {
        let url = url_format!("net/{network_id}", network_id);
        self.client.delete(&url).send().json().await
    }

    /// Registers a new node in a virtual private network.
    pub async fn add_node(&self, network_id: &str, node: &Node) -> Result<()> {
        let url = url_format!("net/{network_id}/nodes", network_id);
        self.client.post(&url).send_json(&node).json().await
    }

    /// Unregisters an existing node in a virtual private network.
    pub async fn remove_node(&self, network_id: &str, node_id: &str) -> Result<()> {
        let url = url_format!("net/{network_id}/nodes/{node_id}", network_id, node_id);
        self.client.post(&url).send().json().await
    }

    /// Creates a new TCP connection
    pub async fn connect_tcp(
        &self,
        network_id: &str,
        ip: &str,
        port: u16,
    ) -> Result<(ClientResponse, Framed<BoxedSocket, Codec>)> {
        let url = url_format!("net/{network_id}/tcp/{ip}/{port}", network_id, ip, port);
        self.client.ws(&url).await
    }
}
