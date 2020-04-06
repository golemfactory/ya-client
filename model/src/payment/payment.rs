use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub payment_id: String,
    pub payer_id: String,
    pub payee_id: String,
    pub amount: BigDecimal,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub allocation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub debit_note_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub invoice_ids: Option<Vec<String>>,
    pub details: String,
}

impl Payment {
    pub fn new(
        payment_id: String,
        payer_id: String,
        payee_id: String,
        amount: BigDecimal,
        timestamp: DateTime<Utc>,
        details: String,
    ) -> Payment {
        Payment {
            payment_id,
            payer_id,
            payee_id,
            amount,
            timestamp,
            allocation_id: None,
            debit_note_ids: None,
            invoice_ids: None,
            details,
        }
    }
}
