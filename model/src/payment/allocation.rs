use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub allocation_id: String,
    pub address: String,
    pub payment_platform: String,
    pub total_amount: BigDecimal,
    pub spent_amount: BigDecimal,
    pub remaining_amount: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<DateTime<Utc>>,
    pub make_deposit: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAllocation {
    pub address: Option<String>,
    pub payment_platform: Option<String>,
    pub total_amount: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<DateTime<Utc>>,
    pub make_deposit: bool,
}
