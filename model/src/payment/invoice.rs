use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_id: String,
    pub issuer_id: String,
    pub recipient_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub last_debit_note_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub agreement_id: String,
    pub activity_ids: Vec<String>,
    pub amount: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub usage_counter_vector: Option<serde_json::Value>,
    pub credit_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub payment_platform: Option<String>,
    pub payment_due_date: DateTime<Utc>,
    pub status: crate::payment::InvoiceStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewInvoice {
    pub agreement_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub activity_ids: Option<Vec<String>>,
    pub amount: BigDecimal,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub usage_counter_vector: Option<serde_json::Value>,
    pub credit_account_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub payment_platform: Option<String>,
    pub payment_due_date: DateTime<Utc>,
}

impl Invoice {
    pub fn new(
        invoice_id: String,
        issuer_id: String,
        recipient_id: String,
        timestamp: DateTime<Utc>,
        agreement_id: String,
        activity_ids: Vec<String>,
        amount: BigDecimal,
        credit_account_id: String,
        payment_due_date: DateTime<Utc>,
        status: crate::payment::InvoiceStatus,
    ) -> Invoice {
        Invoice {
            invoice_id,
            issuer_id,
            recipient_id,
            last_debit_note_id: None,
            timestamp,
            agreement_id,
            activity_ids,
            amount,
            usage_counter_vector: None,
            credit_account_id,
            payment_platform: None,
            payment_due_date,
            status,
        }
    }
}
