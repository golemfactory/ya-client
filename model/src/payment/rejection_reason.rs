use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RejectionReason {
    UnsolicitedService,
    BadService,
    IncorrectAmount,
}
