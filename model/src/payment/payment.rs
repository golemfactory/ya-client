use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::NodeId;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub payment_id: String,
    pub payer_id: NodeId,
    pub payee_id: NodeId,
    pub payer_addr: String,
    pub payee_addr: String,
    pub amount: BigDecimal,
    pub timestamp: DateTime<Utc>,
    pub agreement_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub allocation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub debit_note_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub invoice_ids: Option<Vec<String>>,
    pub details: String,
}
