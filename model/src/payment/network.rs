use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Network {
    pub default_token: String,
    pub tokens: HashMap<String, String>, // token -> platform
}
