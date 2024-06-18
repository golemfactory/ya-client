use serde::{Deserialize, Serialize};

use crate::NodeId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identity {
    pub identity: NodeId,
    pub name: String,
    pub role: String,
}
