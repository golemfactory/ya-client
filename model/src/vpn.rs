use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNetwork {
    pub id: String,
    pub ip: String,
    pub mask: String,
}
