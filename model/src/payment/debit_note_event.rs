use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, ToString};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitNoteEvent {
    pub debit_note_id: String,
    pub event_date: DateTime<Utc>,
    pub event_type: DebitNoteEventType,
}

#[derive(Clone, Debug, Serialize, Deserialize, EnumString, ToString)]
pub enum DebitNoteEventType {
    #[strum(to_string = "RECEIVED")]
    DebitNoteReceivedEvent,
    #[strum(to_string = "ACCEPTED")]
    DebitNoteAcceptedEvent,
    #[strum(to_string = "REJECTED")]
    DebitNoteRejectedEvent {
        rejection: crate::payment::Rejection,
    },
    #[strum(to_string = "CANCELLED")]
    DebitNoteCancelledEvent,
    #[strum(to_string = "SETTLED")]
    DebitNoteSettledEvent,
}
