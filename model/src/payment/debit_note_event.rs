use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitNoteEvent {
    pub debit_note_id: String,
    pub event_date: DateTime<Utc>,
    pub event_type: DebitNoteEventType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DebitNoteEventType {
    DebitNoteReceivedEvent,
    DebitNoteAcceptedEvent,
    DebitNoteRejectedEvent {
        rejection: crate::payment::Rejection,
    },
    DebitNoteCancelledEvent,
    DebitNoteSettledEvent,
}
