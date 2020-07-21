//! Requestor control part of Activity API
use ya_client_model::activity::ACTIVITY_API_PATH;
use ya_client_model::activity::{ExeScriptCommandResult, ExeScriptRequest, RuntimeEvent};

use crate::web::{default_on_timeout, Event, WebClient, WebInterface};
use crate::{Error, Result};
use futures::{Stream, StreamExt};
use std::convert::TryFrom;

/// Bindings for Requestor Control part of the Activity API.
#[derive(Clone)]
pub struct ActivityRequestorControlApi {
    client: WebClient,
}

impl WebInterface for ActivityRequestorControlApi {
    const API_URL_ENV_VAR: &'static str = crate::activity::ACTIVITY_URL_ENV_VAR;
    const API_SUFFIX: &'static str = ACTIVITY_API_PATH;

    fn from_client(client: WebClient) -> Self {
        ActivityRequestorControlApi { client }
    }
}

impl ActivityRequestorControlApi {
    /// Creates new Activity based on given Agreement.
    pub async fn create_activity(&self, agreement_id: &str) -> Result<String> {
        self.client
            .post("activity")
            .send_json(&agreement_id)
            .json()
            .await
    }

    /// Destroys given Activity.
    pub async fn destroy_activity(&self, activity_id: &str) -> Result<()> {
        let uri = url_format!("activity/{activity_id}", activity_id);
        self.client.delete(&uri).send().json().await?;
        Ok(())
    }

    /// Executes an ExeScript batch within a given Activity.
    pub async fn exec(&self, script: ExeScriptRequest, activity_id: &str) -> Result<String> {
        let uri = url_format!("activity/{activity_id}/exec", activity_id);
        self.client.post(&uri).send_json(&script).json().await
    }

    /// Queries for ExeScript batch results.
    #[rustfmt::skip]
    pub async fn get_exec_batch_results(
        &self,
        activity_id: &str,
        batch_id: &str,
        #[allow(non_snake_case)]
        timeout: Option<f32>,
        command_index: Option<usize>,
    ) -> Result<Vec<ExeScriptCommandResult>> {
        let uri = url_format!(
            "activity/{activity_id}/exec/{batch_id}",
            activity_id,
            batch_id,
            #[query] timeout,
            #[query] command_index,
        );
        self.client.get(&uri).send().json().await.or_else(default_on_timeout)
    }

    /// Streams ExeScript batch results
    pub async fn stream_exec_batch_results(
        &self,
        activity_id: &str,
        batch_id: &str,
    ) -> Result<impl Stream<Item = RuntimeEvent>> {
        let uri = url_format!(
            "activity/{activity_id}/exec/{batch_id}",
            activity_id,
            batch_id,
        );
        let stream = self
            .client
            .event_stream(&uri)
            .await?
            .filter_map(|result| async {
                match result {
                    Ok(evt) => RuntimeEvent::try_from(evt).ok(),
                    _ => None,
                }
            });
        Ok(stream)
    }
}

impl TryFrom<Event> for RuntimeEvent {
    type Error = Error;

    fn try_from(evt: Event) -> Result<Self> {
        serde_json::from_str(evt.data.as_str()).map_err(Error::from)
    }
}
