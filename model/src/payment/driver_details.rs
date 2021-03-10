use crate::payment::network::Network;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DriverDetails {
    pub default_network: String,
    pub networks: HashMap<String, Network>,
    pub recv_init_required: bool, // Is account initialization required for receiving payments
}
