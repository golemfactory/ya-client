use serde::{Deserialize, Serialize};

/// Specify constructor data for new market scanning iterator.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewScan {
    timeout: Option<u64>,
    stype: ScanType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanType {
    Offer,
    Demand,
}
