use crate::NodeId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, ToString};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitNoteEvent {
    pub debit_note_id: String,
    pub event_date: DateTime<Utc>,
    #[serde(flatten)]
    pub event_type: DebitNoteEventType,
}

#[derive(Clone, Debug, Serialize, Deserialize, EnumString, ToString)]
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
    use chrono::TimeZone;

    #[test]
    fn test_serialize_rejected_event_has_flat_rejection() {
        let ie = DebitNoteEvent {
            node_id: NodeId::default(),
            agreement_id: "538115101743e2e79e9d34b554079f070d286a98f6542c50e7ad61c19538ee16"
                .to_string(),
            activity_id: "12345".to_string(),
            debit_note_id: "ajdik".to_string(),
            event_date: Utc
                .datetime_from_str("2020-12-21T15:51:21.126645Z", "%+")
                .unwrap(),
            event_type: DebitNoteEventType::DebitNoteRejectedEvent {
                rejection: Rejection {
                    rejection_reason: RejectionReason::UnsolicitedService,
                    total_amount_accepted: BigDecimal::from_f32(3.14).unwrap(),
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
                    \"totalAmountAccepted\":\"3.140000\"\
                }\
             }",
            serde_json::to_string(&ie).unwrap()
        );
    }
}
