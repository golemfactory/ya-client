use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitNoteEvent {
    pub debit_note_id: String,
    pub event_date: DateTime<Utc>,
    #[serde(flatten)]
    pub event_type: DebitNoteEventType,
}

#[derive(Clone, Debug, Display, Serialize, Deserialize, EnumString)]
#[serde(tag = "eventType")]
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::payment::{Rejection, RejectionReason};
    use bigdecimal::{BigDecimal, FromPrimitive};

    #[test]
    fn test_serialize_rejected_event_has_flat_rejection() {
        let ie = DebitNoteEvent {
            debit_note_id: "ajdik".to_string(),
            event_date: DateTime::parse_from_str("2020-12-21T15:51:21.126645Z", "%+")
                .unwrap()
                .into(),
            event_type: DebitNoteEventType::DebitNoteRejectedEvent {
                rejection: Rejection {
                    rejection_reason: RejectionReason::UnsolicitedService,
                    total_amount_accepted: BigDecimal::from_f32(13.14).unwrap(),
                    message: None,
                },
            },
        };

        assert_eq!(
            "{\"debitNoteId\":\"ajdik\",\
                \"eventDate\":\"2020-12-21T15:51:21.126645Z\",\
                \"eventType\":\"DebitNoteRejectedEvent\",\
                \"rejection\":{\
                    \"rejectionReason\":\"UNSOLICITED_SERVICE\",\
                    \"totalAmountAccepted\":\"13.14000\"\
                }\
             }",
            serde_json::to_string(&ie).unwrap()
        );
    }
}
