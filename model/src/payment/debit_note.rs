use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitNote {
    pub debit_note_id: String,
    pub issuer_id: String,
    pub recipient_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub previous_debit_note_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub agreement_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub activity_id: Option<String>,
    pub total_amount_due: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub usage_counter_vector: Option<serde_json::Value>,
    pub credit_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub payment_platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub payment_due_date: Option<DateTime<Utc>>,
    pub status: crate::payment::InvoiceStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDebitNote {
    pub agreement_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub activity_id: Option<String>,
    pub total_amount_due: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub usage_counter_vector: Option<serde_json::Value>,
    pub credit_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub payment_platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub payment_due_date: Option<DateTime<Utc>>,
}

impl DebitNote {
    pub fn new(
        debit_note_id: String,
        issuer_id: String,
        recipient_id: String,
        timestamp: DateTime<Utc>,
        agreement_id: String,
        total_amount_due: BigDecimal,
        credit_account_id: String,
        status: crate::payment::InvoiceStatus,
    ) -> DebitNote {
        DebitNote {
            debit_note_id,
            issuer_id,
            recipient_id,
            previous_debit_note_id: None,
            timestamp,
            agreement_id,
            activity_id: None,
            total_amount_due,
            usage_counter_vector: None,
            credit_account_id,
            payment_platform: None,
            payment_due_date: None,
            status,
        }
    }
}
