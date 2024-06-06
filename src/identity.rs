use ya_client_model::NodeId;

use crate::web::{WebClient, WebInterface};
use crate::Result;

pub const IDENTITY_URL_ENV_VAR: &str = "YAGNA_IDENTITY_URL";

/// Bindings for Requestor part of the Identity API.
#[derive(Clone)]
pub struct IdentityApi {
    client: WebClient,
}

impl WebInterface for IdentityApi {
    const API_URL_ENV_VAR: &'static str = "YAGNA_IDENTITY_URL";
    const API_SUFFIX: &'static str = "";

    fn from_client(client: WebClient) -> Self {
        IdentityApi { client }
    }
}

impl IdentityApi {
    pub async fn get_default_identity(&self) -> Result<NodeId> {
        self.client.get("default-identity").send().json().await
    }
}
