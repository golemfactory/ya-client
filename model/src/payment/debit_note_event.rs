use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use super::DriverStatusProperty;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitNoteEvent {
    pub debit_note_id: String,
    pub event_date: DateTime<Utc>,
    #[serde(flatten)]
    pub event_type: DebitNoteEventType,
}

#[derive(Clone, Debug, Display, Serialize, Deserialize)]
#[serde(tag = "eventType")]
pub enum DebitNoteEventType {
    DebitNoteReceivedEvent,
    DebitNoteAcceptedEvent,
    DebitNoteRejectedEvent {
        rejection: crate::payment::Rejection,
    },
    DebitNoteCancelledEvent,
    DebitNoteSettledEvent,
    DebitNotePaymentStatusEvent {
        property: DriverStatusProperty,
    },
    DebitNotePaymentOkEvent,
}

impl DebitNoteEventType {
    pub fn discriminant(&self) -> &'static str {
        use DebitNoteEventType::*;
        match self {
            DebitNoteReceivedEvent => "RECEIVED",
            DebitNoteAcceptedEvent => "ACCEPTED",
            DebitNoteRejectedEvent { .. } => "REJECTED",
            DebitNoteCancelledEvent => "CANCELLED",
            DebitNoteSettledEvent => "SETTLED",
            DebitNotePaymentStatusEvent { .. } => "PAYMENT_EVENT",
            DebitNotePaymentOkEvent => "PAYMENT_OK",
        }
    }

    pub fn details(&self) -> Option<serde_json::Value> {
        use serde_json::to_value;
        use DebitNoteEventType::*;

        match self {
            DebitNoteRejectedEvent { rejection } => to_value(rejection).ok(),
            DebitNotePaymentStatusEvent { property } => to_value(property).ok(),
            _ => None,
        }
    }

    pub fn from_discriminant_and_details(
        discriminant: &str,
        details: Option<serde_json::Value>,
    ) -> Option<Self> {
        use serde_json::from_value;
        use DebitNoteEventType::*;

        Some(match (discriminant, details) {
            ("RECEIVED", _) => DebitNoteReceivedEvent,
            ("ACCEPTED", _) => DebitNoteAcceptedEvent,
            ("REJECTED", Some(details)) => DebitNoteRejectedEvent {
                rejection: from_value(details).ok()?,
            },
            ("CANCELLED", _) => DebitNoteCancelledEvent,
            ("SETTLED", _) => DebitNoteSettledEvent,
            ("PAYMENT_EVENT", Some(details)) => DebitNotePaymentStatusEvent {
                property: from_value(details).ok()?,
            },
            ("PAYMENT_OK", _) => DebitNotePaymentOkEvent,
            _ => None?,
        })
    }
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
