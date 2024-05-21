use std::collections::HashMap;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::time::Duration;

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

#[serde_as]
#[skip_serializing_none]
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
    pub timeout: Option<DateTime<Utc>>,
    pub deposit: Option<Deposit>,
    #[serde(default)]
    pub make_deposit: bool,
    #[serde_as(as = "Option<DurationSeconds<u64>>")]
    pub extend_timeout: Option<Duration>,
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
    pub extend_timeout: Option<Duration>,
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
    use super::*;
    use serde::de::DeserializeOwned;
    use std::fmt::Debug;

    fn can_parse_to<T: DeserializeOwned + Debug>(json: &str) {
        let v: T = serde_json::from_str(json).unwrap();
        eprintln!("{:?}", v);
    }

    #[test]
    fn test_new_allocation() {
        can_parse_to::<NewAllocation>(
            r#"{
            "paymentPlatform": "erc20-polygon-glm",
            "totalAmount": 1.0,
            "timeout": "2023-08-28T15:16:31.858Z",
            "makeDeposit": false,
            "extendTimeout": 3600
        }"#,
        );
        can_parse_to::<NewAllocation>(
            r#"{
            "totalAmount": 5
        }"#,
        );
        can_parse_to::<NewAllocation>(
            r#"{
            "paymentPlatform": { "token": "GLM" },
            "totalAmount": "512.2345"
        }"#,
        );
    }

    #[test]
    fn test_allocation() {
        let j = serde_json::to_string(&Allocation {
            allocation_id: "".to_string(),
            address: "".to_string(),
            payment_platform: "".to_string(),
            total_amount: Default::default(),
            spent_amount: Default::default(),
            remaining_amount: Default::default(),
            timestamp: Default::default(),
            timeout: None,
            deposit: None,
            make_deposit: false,
            extend_timeout: None,
        })
        .unwrap();
        assert_eq!(
            r#"{"allocationId":"","address":"","paymentPlatform":"","totalAmount":"0","spentAmount":"0","remainingAmount":"0","timestamp":"1970-01-01T00:00:00Z","makeDeposit":false}"#,
            j
        );
    }
}
