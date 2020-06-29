use crate::NodeId;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_id: String,
    pub issuer_id: NodeId,
    pub recipient_id: NodeId,
    pub payee_addr: String,
    pub payer_addr: String,
    pub payment_platform: String,
    pub timestamp: DateTime<Utc>,
    pub agreement_id: String,
    pub activity_ids: Vec<String>,
    pub amount: BigDecimal,
    pub payment_due_date: DateTime<Utc>,
    pub status: crate::payment::DocumentStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewInvoice {
    pub agreement_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub activity_ids: Option<Vec<String>>,
    pub amount: BigDecimal,
    pub payment_due_date: DateTime<Utc>,
}
