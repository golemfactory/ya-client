//! Provider part of the Activity API
use ya_client_model::activity::{ActivityState, ActivityUsage, ProviderEvent, ACTIVITY_API_PATH};

use crate::{web::default_on_timeout, web::WebClient, web::WebInterface, Result};
use chrono::{DateTime, Utc};
use std::time::Duration;

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
    /// Fetch list of activity_ids
    pub async fn get_activity_ids(&self) -> Result<Vec<String>> {
        self.client.get(&"activity").send().json().await
    }

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

    /// Get agreement corresponding to the activity
    pub async fn get_agreement_id(&self, activity_id: &str) -> Result<String> {
        let uri = url_format!("activity/{activity_id}/agreement");
        self.client.get(&uri).send().json().await
    }

    /// Fetch Requestor command events.
    #[rustfmt::skip]
    pub async fn get_activity_events(
        &self,
        after_timestamp: Option<DateTime<Utc>>,
        app_session_id: Option<String>,
        timeout: Option<Duration>,
        max_events: Option<u32>,
    ) -> Result<Vec<ProviderEvent>> {
        let after_timestamp = after_timestamp.map(|ts| ts.to_rfc3339());
        let timeout = timeout.map(|d| d.as_secs_f32());
        let url = url_format!(
            "events",
            #[query] after_timestamp,
            #[query] app_session_id,
            #[query] timeout,
            #[query] max_events,
        );

        self.client.get(&url).send().json().await.or_else(default_on_timeout)
    }
}
