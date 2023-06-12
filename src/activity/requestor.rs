//! Requestor part of the Activity API
use crate::web::{WebClient, WebInterface};
use crate::Result;
use ya_client_model::activity::ACTIVITY_API_PATH;

pub mod control;
pub mod state;

#[derive(Clone)]
pub struct ActivityRequestorApi {
    control: control::ActivityRequestorControlApi,
    state: state::ActivityRequestorStateApi,
    client: WebClient,
}

impl WebInterface for ActivityRequestorApi {
    const API_URL_ENV_VAR: &'static str = crate::activity::ACTIVITY_URL_ENV_VAR;
    const API_SUFFIX: &'static str = ACTIVITY_API_PATH;

    fn from_client(client: WebClient) -> Self {
        Self {
            control: WebInterface::from_client(client.clone()),
            state: WebInterface::from_client(client.clone()),
            client,
        }
    }
}

impl ActivityRequestorApi {
    pub fn control(&self) -> &control::ActivityRequestorControlApi {
        &self.control
    }

    pub fn state(&self) -> &state::ActivityRequestorStateApi {
        &self.state
    }

    /// Get agreement corresponding to the activity
    pub async fn get_agreement(&self, activity_id: &str) -> Result<String> {
        let uri = url_format!("activity/{activity_id}/agreement");
        self.client.get(&uri).send().json().await
    }
}

pub use control::ActivityRequestorControlApi;
pub use state::ActivityRequestorStateApi;
