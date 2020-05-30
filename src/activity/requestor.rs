//! Requestor part of the Activity API
use crate::web::{WebClient, WebInterface};
use ya_client_model::activity::ACTIVITY_API_PATH;

pub mod control;
pub mod state;

#[derive(Clone)]
pub struct ActivityRequestorApi {
    control: control::ActivityRequestorControlApi,
    state: state::ActivityRequestorStateApi,
}

impl WebInterface for ActivityRequestorApi {
    const API_URL_ENV_VAR: &'static str = crate::activity::ACTIVITY_URL_ENV_VAR;
    const API_SUFFIX: &'static str = ACTIVITY_API_PATH;

    fn from_client(client: WebClient) -> Self {
        Self {
            control: WebInterface::from_client(client.clone()),
            state: WebInterface::from_client(client),
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
}
