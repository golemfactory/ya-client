use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPayment {
    pub activity_id: String,
    pub amount: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub allocation_id: Option<String>,
}
