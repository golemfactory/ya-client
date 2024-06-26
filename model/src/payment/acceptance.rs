use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Acceptance {
    pub total_amount_accepted: BigDecimal,
    pub allocation_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitNoteAcceptance {
    pub total_amount_accepted: BigDecimal,
    pub allocation_id: String,
    pub auto_accept_to: Option<BigDecimal>,
}
