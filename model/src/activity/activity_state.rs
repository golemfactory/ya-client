use serde::{Deserialize, Serialize};

/// Reported activity state
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityState {
    /// Pair with current state and optional pending state.
    #[serde(rename = "state")]
    pub state: StatePair,
    /// Reason for Activity termination (specified when Activity in Terminated state).
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// If error caused state change - error message shall be provided.
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ActivityState {
    /// `true` if activity is terminated or during termination.
    pub fn alive(&self) -> bool {
        self.state.alive()
    }
}

impl From<&StatePair> for ActivityState {
    fn from(pending: &StatePair) -> Self {
        ActivityState {
            state: *pending,
            reason: None,
            error_message: None,
        }
    }
}

impl From<StatePair> for ActivityState {
    fn from(pending: StatePair) -> Self {
        ActivityState {
            state: pending,
            reason: None,
            error_message: None,
        }
    }
}

/// Pair with current state and optional pending state.
#[derive(
    Clone, Copy, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub struct StatePair(pub State, pub Option<State>);

impl StatePair {
    /// `true` if activity is terminated or during termination.
    pub fn alive(&self) -> bool {
        !matches!(
            (&self.0, &self.1),
            (State::Terminated, _) | (_, Some(State::Terminated))
        )
    }

    /// Creates transition state from current state to new state.
    pub fn to_pending(&self, state: State) -> Self {
        StatePair(self.0, Some(state))
    }
}

impl From<State> for StatePair {
    fn from(state: State) -> Self {
        StatePair(state, None)
    }
}

/// Represents activity state.
#[derive(
    Clone, Default, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum State {
    /// Activity is new and uninitialized.
    #[default]
    New,
    /// Activity is configured (agreement data is read).
    Initialized,
    /// Activity is ready to start, deploy command ends successfully.
    /// Activity can be started with start command.
    Deployed,
    /// Activity is running.
    Ready,
    /// Activity is terminated.
    Terminated,
    /// Running activity that for some reason
    /// is not reporting its state for some time.
    Unresponsive,
}
