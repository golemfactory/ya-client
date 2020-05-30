//! Provider part of the Activity API
use ya_client_model::activity::{ActivityState, ActivityUsage, ProviderEvent, ACTIVITY_API_PATH};

use crate::{web::default_on_timeout, web::WebClient, web::WebInterface, Result};

#[derive(Clone)]
pub struct ActivityProviderApi {
    client: WebClient,
}

impl WebInterface for ActivityProviderApi {
    const API_URL_ENV_VAR: &'static str = crate::activity::ACTIVITY_URL_ENV_VAR;
    const API_SUFFIX: &'static str = ACTIVITY_API_PATH;

    fn from_client(client: WebClient) -> Self {
        ActivityProviderApi { client }
    }
}

/// Bindings for Provider part of the Activity API.
impl ActivityProviderApi {
    /// Fetch activity state (which may include error details)
    pub async fn get_activity_state(&self, activity_id: &str) -> Result<ActivityState> {
        let uri = url_format!("activity/{activity_id}/state", activity_id);
        self.client.get(&uri).send().json().await
    }

    /// Set state of specified Activity.
    pub async fn set_activity_state(&self, activity_id: &str, state: &ActivityState) -> Result<()> {
        let uri = url_format!("activity/{activity_id}/state", activity_id);
        self.client.put(&uri).send_json(&state).json().await
    }

    /// Fetch current activity usage (which may include error details)
    pub async fn get_activity_usage(&self, activity_id: &str) -> Result<ActivityUsage> {
        let uri = url_format!("activity/{activity_id}/usage", activity_id);
        self.client.get(&uri).send().json().await
    }

    /// Fetch Requestor command events.
    #[rustfmt::skip]
    pub async fn get_activity_events(
        &self,
        #[allow(non_snake_case)]
        timeout: Option<f32>,
        #[allow(non_snake_case)]
        maxEvents: Option<i32>,
    ) -> Result<Vec<ProviderEvent>> {
        let url = url_format!(
            "events",
            #[query] timeout,
            #[query] maxEvents
        );

        self.client.get(&url).send().json().await.or_else(default_on_timeout)
    }
}
