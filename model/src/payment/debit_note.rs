use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitNote {
    pub debit_note_id: String,
    pub issuer_id: String,
    pub recipient_id: String,
    pub payee_addr: String,
    pub payer_addr: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub previous_debit_note_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub agreement_id: String,
    pub activity_id: String,
    pub total_amount_due: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub usage_counter_vector: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub payment_due_date: Option<DateTime<Utc>>,
    pub status: crate::payment::InvoiceStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDebitNote {
    pub activity_id: String,
    pub total_amount_due: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub usage_counter_vector: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub payment_due_date: Option<DateTime<Utc>>,
}
