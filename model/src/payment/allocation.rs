use std::time::Duration;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deposit {
    pub id: String,
    pub contract: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde_as]
#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct Allocation {
    pub allocation_id: String,
    pub address: String,
    pub payment_platform: String,
    pub total_amount: BigDecimal,
    pub spent_amount: BigDecimal,
    pub remaining_amount: BigDecimal,
    pub timestamp: DateTime<Utc>,
    pub timeout: Option<DateTime<Utc>>,
    pub deposit: Option<Deposit>,
    #[serde(default)]
    pub make_deposit: bool,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    pub extend_timeout : Option<Duration>
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


#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewAllocation {
    pub address: Option<String>,
    pub payment_platform: Option<PaymentPlatformEnum>,
    pub total_amount: BigDecimal,
    pub timeout: Option<DateTime<Utc>>,
    pub deposit: Option<Deposit>,
    #[serde(default)]
    pub make_deposit: bool,
    #[serde_as(as = "Option<serde_with::DurationSeconds<u64>>")]
    pub extend_timeout : Option<Duration>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationUpdate {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub total_amount: Option<BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub timeout: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use serde::de::DeserializeOwned;
    use super::*;

    fn can_parse_to<T : DeserializeOwned + Debug>(json : &str) {
        let v : T = serde_json::from_str(json).unwrap();
        eprintln!("{:?}", v);
    }

    #[test]
    fn test_new_allocation() {
        can_parse_to::<NewAllocation>(r#"{
            "paymentPlatform": "erc20-polygon-glm",
            "totalAmount": 1.0,
            "timeout": "2023-08-28T15:16:31.858Z",
            "makeDeposit": false,
            "extendTimeout": 3600
        }"#);
        can_parse_to::<NewAllocation>(r#"{
            "totalAmount": 5
        }"#);
        can_parse_to::<NewAllocation>(r#"{
            "paymentPlatform": { "token": "GLM" },
            "totalAmount": "512.2345"
        }"#);
    }

}