use crate::payment::network::Network;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DriverDetails {
    pub default_network: String,
    pub networks: HashMap<String, Network>,
    pub recv_init_required: bool, // Is account initialization required for receiving payments
}
