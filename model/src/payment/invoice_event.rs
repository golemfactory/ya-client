use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, ToString};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceEvent {
    pub invoice_id: String,
    pub event_date: DateTime<Utc>,
    #[serde(flatten)]
    pub event_type: InvoiceEventType,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, EnumString, ToString)]
#[serde(tag = "eventType")]
pub enum InvoiceEventType {
    #[strum(to_string = "RECEIVED")]
    InvoiceReceivedEvent,
    #[strum(to_string = "ACCEPTED")]
    InvoiceAcceptedEvent,
    #[strum(to_string = "REJECTED")]
    InvoiceRejectedEvent {
        rejection: crate::payment::Rejection,
    },
    #[strum(to_string = "CANCELLED")]
    InvoiceCancelledEvent,
    #[strum(to_string = "SETTLED")]
    InvoiceSettledEvent,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::payment::{Rejection, RejectionReason};
    use bigdecimal::{BigDecimal, FromPrimitive};
    use chrono::TimeZone;

    #[test]
    fn test_serialize_rejected_event_has_flat_rejection() {
        let ie = InvoiceEvent {
            invoice_id: "ajdik".to_string(),
            event_date: Utc
                .datetime_from_str("2020-12-21T15:51:21.126645Z", "%+")
                .unwrap(),
            event_type: InvoiceEventType::InvoiceRejectedEvent {
                rejection: Rejection {
                    rejection_reason: RejectionReason::UnsolicitedService,
                    total_amount_accepted: BigDecimal::from_f32(3.14).unwrap(),
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
                    \"totalAmountAccepted\":\"3.140000\"\
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
                event_date: Utc
                    .datetime_from_str("2020-12-21T15:51:21.126645Z", "%+")
                    .unwrap(),
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
        let iet: InvoiceEventType = "REJECTED".parse().unwrap();
        assert_eq!(
            InvoiceEventType::InvoiceRejectedEvent {
                rejection: Default::default()
            },
            iet
        );
    }

    #[test]
    fn test_deserialize_event_type_to_string() {
        assert_eq!(InvoiceEventType::InvoiceSettledEvent.to_string(), "SETTLED");
    }
}
