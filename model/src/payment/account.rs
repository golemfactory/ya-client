use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub platform: String,
    pub address: String,
    pub driver: String,
    pub network: String,
    pub token: String,
    pub send: bool,
    pub receive: bool,
}
