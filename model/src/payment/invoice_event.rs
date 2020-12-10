use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceEvent {
    pub invoice_id: String,
    pub event_date: DateTime<Utc>,
    #[serde(flatten)]
    pub event_type: crate::payment::EventType,
}
