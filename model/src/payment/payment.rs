use crate::payment::{ActivityPayment, AgreementPayment};
use crate::NodeId;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub payment_id: String,
    pub payer_id: NodeId,
    pub payee_id: NodeId,
    pub payer_addr: String,
    pub payee_addr: String,
    pub payment_platform: String,
    pub amount: BigDecimal,
    pub timestamp: DateTime<Utc>,
    pub agreement_payments: Vec<AgreementPayment>,
    pub activity_payments: Vec<ActivityPayment>,
    pub details: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DriverStatusProperty {
    InsufficientGas {
        driver: String,
        network: String,
        #[serde(rename = "neededGasEst")]
        needed_gas_est: String,
    },
    InsufficientToken {
        driver: String,
        network: String,
        #[serde(rename = "neededTokenEst")]
        needed_token_est: String,
    },
    InvalidChainId {
        driver: String,
        #[serde(rename = "chainId")]
        chain_id: i64,
    },
    CantSign {
        driver: String,
        network: String,
        address: String,
    },
    TxStuck {
        driver: String,
        network: String,
    },
    RpcError {
        driver: String,
        network: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, json, value::to_value};

    #[test]
    fn status_prop_serialization() {
        assert_eq!(
            json!({
                "driver": "erc20",
                "kind": "InsufficientGas",
                "network": "foo",
                "neededGasEst": "bar",
            }),
            to_value(&DriverStatusProperty::InsufficientGas {
                driver: "erc20".into(),
                network: "foo".into(),
                needed_gas_est: "bar".into()
            })
            .unwrap()
        );
    }

    #[test]
    fn status_prop_deserialization() {
        assert_eq!(
            DriverStatusProperty::TxStuck {
                driver: "erc20".into(),
                network: "baz".into(),
            },
            from_str(
                r#"{
                    "driver": "erc20",
                    "kind": "TxStuck",
                    "network": "baz"
            }"#
            )
            .unwrap()
        );
    }
}
