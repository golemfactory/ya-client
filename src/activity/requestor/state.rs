//! Requestor state part of Activity API
use ya_client_model::activity::{ActivityState, ExeScriptCommandState, ACTIVITY_API_PATH};

use crate::{web::WebClient, web::WebInterface, Result};

/// Bindings for Requestor State part of the Activity API.
#[derive(Clone)]
pub struct ActivityRequestorStateApi {
    client: WebClient,
}

impl WebInterface for ActivityRequestorStateApi {
    const API_URL_ENV_VAR: &'static str = crate::activity::ACTIVITY_URL_ENV_VAR;
    const API_SUFFIX: &'static str = ACTIVITY_API_PATH;

    fn from_client(client: WebClient) -> Self {
        ActivityRequestorStateApi { client }
    }
}

impl ActivityRequestorStateApi {
    /// Get running command for a specified Activity.
    pub async fn get_running_command(&self, activity_id: &str) -> Result<ExeScriptCommandState> {
        let uri = url_format!("activity/{activity_id}/command", activity_id);
        self.client.get(&uri).send().json().await
    }

    /// Get state of specified Activity.
    pub async fn get_state(&self, activity_id: &str) -> Result<ActivityState> {
        let uri = url_format!("activity/{activity_id}/state", activity_id);
        self.client.get(&uri).send().json().await
    }

    /// Get usage of specified Activity.
    pub async fn get_usage(&self, activity_id: &str) -> Result<Vec<f64>> {
        let uri = url_format!("activity/{activity_id}/usage", activity_id);
        self.client.get(&uri).send().json().await
    }
}
