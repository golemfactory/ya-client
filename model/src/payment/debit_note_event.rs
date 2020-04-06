use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitNoteEvent {
    pub debit_note_id: String,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub details: Option<serde_json::Value>,
    pub event_type: crate::payment::EventType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDebitNoteEvent {
    pub debit_note_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub details: Option<serde_json::Value>,
    pub event_type: crate::payment::EventType,
}

impl DebitNoteEvent {
    pub fn new(
        debit_note_id: String,
        timestamp: DateTime<Utc>,
        event_type: crate::payment::EventType,
    ) -> DebitNoteEvent {
        DebitNoteEvent {
            debit_note_id,
            timestamp,
            details: None,
            event_type,
        }
    }
}
