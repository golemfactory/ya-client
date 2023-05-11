use crate::error::Result;
use crate::web::{WebClient, WebInterface};
use ya_client_model::p2p::Status;

/// Bindings for Requestor part of the Net API.
#[derive(Clone)]
pub struct NetApi {
    client: WebClient,
}

impl WebInterface for NetApi {
    const API_URL_ENV_VAR: &'static str = "YAGNA_NET_URL";
    const API_SUFFIX: &'static str = ya_client_model::p2p::NET_API_V2_NET_PATH;

    fn from_client(client: WebClient) -> Self {
        NetApi { client }
    }
}

impl NetApi {
    /// Retrieves connection status.
    pub async fn get_status(&self) -> Result<Status> {
        self.client.get("status").send().json().await
    }
}
