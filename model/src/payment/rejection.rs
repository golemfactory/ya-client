use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rejection {
    pub rejection_reason: crate::payment::RejectionReason,
    pub total_amount_accepted: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub message: Option<String>,
}
