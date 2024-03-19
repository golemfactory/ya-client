use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use super::DriverStatusProperty;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceEvent {
    pub invoice_id: String,
    pub event_date: DateTime<Utc>,
    #[serde(flatten)]
    pub event_type: InvoiceEventType,
}

#[derive(Clone, Debug, Display, Serialize, Deserialize, PartialEq)]
#[serde(tag = "eventType")]
pub enum InvoiceEventType {
    InvoiceReceivedEvent,
    InvoiceAcceptedEvent,
    InvoiceRejectedEvent {
        rejection: crate::payment::Rejection,
    },
    InvoiceCancelledEvent,
    InvoiceSettledEvent,
    InvoicePaymentStatusEvent {
        property: DriverStatusProperty,
    },
    InvoicePaymentOkEvent,
}

impl InvoiceEventType {
    pub fn discriminant(&self) -> &'static str {
        use InvoiceEventType::*;
        match self {
            InvoiceReceivedEvent => "RECEIVED",
            InvoiceAcceptedEvent => "ACCEPTED",
            InvoiceRejectedEvent { .. } => "REJECTED",
            InvoiceCancelledEvent => "CANCELLED",
            InvoiceSettledEvent => "SETTLED",
            InvoicePaymentStatusEvent { .. } => "PAYMENT_EVENT",
            InvoicePaymentOkEvent => "PAYMENT_OK",
        }
    }

    pub fn details(&self) -> Option<serde_json::Value> {
        use serde_json::to_value;
        use InvoiceEventType::*;

        match self {
            InvoiceRejectedEvent { rejection } => to_value(rejection).ok(),
            InvoicePaymentStatusEvent { property } => to_value(property).ok(),
            _ => None,
        }
    }

    pub fn from_discriminant_and_details(
        discriminant: &str,
        details: Option<serde_json::Value>,
    ) -> Option<Self> {
        use serde_json::from_value;
        use InvoiceEventType::*;

        Some(match (discriminant, details) {
            ("RECEIVED", _) => InvoiceReceivedEvent,
            ("ACCEPTED", _) => InvoiceAcceptedEvent,
            ("REJECTED", Some(details)) => InvoiceRejectedEvent {
                rejection: from_value(details).ok()?,
            },
            ("CANCELLED", _) => InvoiceCancelledEvent,
            ("SETTLED", _) => InvoiceSettledEvent,
            ("PAYMENT_EVENT", Some(details)) => InvoicePaymentStatusEvent {
                property: from_value(details).ok()?,
            },
            ("PAYMENT_OK", _) => InvoicePaymentOkEvent,
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
        let ie = InvoiceEvent {
            invoice_id: "ajdik".to_string(),
            event_date: DateTime::parse_from_str("2020-12-21T15:51:21.126645Z", "%+")
                .unwrap()
                .with_timezone(&Utc),
            event_type: InvoiceEventType::InvoiceRejectedEvent {
                rejection: Rejection {
                    rejection_reason: RejectionReason::UnsolicitedService,
                    total_amount_accepted: BigDecimal::from_f32(13.14).unwrap(),
                    message: None,
                },
            },
        };

        assert_eq!(
            "{\"invoiceId\":\"ajdik\",\
                \"eventDate\":\"2020-12-21T15:51:21.126645Z\",\
                \"eventType\":\"InvoiceRejectedEvent\",\
                \"rejection\":{\
                    \"rejectionReason\":\"UNSOLICITED_SERVICE\",\
                    \"totalAmountAccepted\":\"13.14000\"\
                }\
             }",
            serde_json::to_string(&ie).unwrap()
        );
    }

    #[test]
    fn test_deserialize_event() {
        let ie: InvoiceEvent = serde_json::from_str(
            "{\
                \"invoiceId\":\"ajdik\",\
                \"eventDate\":\"2020-12-21T15:51:21.126645Z\",\
                \"eventType\":\"InvoiceAcceptedEvent\"\
            }",
        )
        .unwrap();

        assert_eq!(
            InvoiceEvent {
                invoice_id: "ajdik".to_string(),
                event_date: DateTime::parse_from_str("2020-12-21T15:51:21.126645Z", "%+")
                    .unwrap()
                    .with_timezone(&Utc),
                event_type: InvoiceEventType::InvoiceAcceptedEvent,
            },
            ie
        );
    }

    #[test]
    fn test_serialize_event_type() {
        let iet = InvoiceEventType::InvoiceSettledEvent;
        assert_eq!(
            "{\"eventType\":\"InvoiceSettledEvent\"}",
            serde_json::to_string(&iet).unwrap()
        );
    }

    #[test]
    fn test_deserialize_event_type() {
        let iet: InvoiceEventType =
            serde_json::from_str("{\"eventType\":\"InvoiceReceivedEvent\"}").unwrap();
        assert_eq!(InvoiceEventType::InvoiceReceivedEvent, iet);
    }

    #[test]
    fn test_deserialize_event_type_from_str() {
        let iet = InvoiceEventType::from_discriminant_and_details(
            "REJECTED",
            InvoiceEventType::InvoiceRejectedEvent {
                rejection: Default::default(),
            }
            .details(),
        )
        .unwrap();
        assert_eq!(
            InvoiceEventType::InvoiceRejectedEvent {
                rejection: Default::default()
            },
            iet
        );
    }

    #[test]
    fn test_deserialize_event_type_to_string() {
        assert_eq!(
            InvoiceEventType::InvoiceSettledEvent.discriminant(),
            "SETTLED"
        );
    }
}
