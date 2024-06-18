use std::collections::HashMap;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateDepositCall {
    #[serde(flatten)]
    pub arguments: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deposit {
    pub id: String,
    pub contract: String,
    pub validate: Option<ValidateDepositCall>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DepositUpdate {
    pub validate: Option<ValidateDepositCall>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub allocation_id: String,
    pub address: String,
    pub payment_platform: String,
    pub total_amount: BigDecimal,
    pub spent_amount: BigDecimal,
    pub remaining_amount: BigDecimal,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub deposit: Option<Deposit>,
    pub make_deposit: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPlatform {
    pub driver: Option<String>,
    pub network: Option<String>,
    pub token: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum PaymentPlatformEnum {
    PaymentPlatformName(String),
    PaymentPlatform(PaymentPlatform),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAllocation {
    pub address: Option<String>,
    pub payment_platform: Option<PaymentPlatformEnum>,
    pub total_amount: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub deposit: Option<Deposit>,
    pub make_deposit: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationUpdate {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub total_amount: Option<BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub deposit: Option<DepositUpdate>,
}
