use serde::{Deserialize, Serialize};

/// Specify constructor data for new market scanning iterator.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewScan {
    timeout: Option<u64>,
    #[serde(rename="type")]
    scan_type: ScanType,
    constraints : Option<String>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanType {
    Offer,
    Demand,
}
