use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Network {
    pub default_token: String,
    pub tokens: HashMap<String, String>, // token -> platform
}
