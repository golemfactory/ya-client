use serde::{Deserialize, Serialize};

#[derive(
    Clone, Default, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RejectionReason {
    #[default]
    UnsolicitedService,
    BadService,
    IncorrectAmount,
}
