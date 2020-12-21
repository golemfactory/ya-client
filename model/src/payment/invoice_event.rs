use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceEvent {
    pub invoice_id: String,
    pub event_date: DateTime<Utc>,
    pub event_type: InvoiceEventType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InvoiceEventType {
    InvoiceReceivedEvent,
    InvoiceAcceptedEvent,
    InvoiceRejectedEvent {
        rejection: crate::payment::Rejection,
    },
    InvoiceCancelledEvent,
    InvoiceSettledEvent,
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_serialize() {
        let ie = InvoiceEvent {
            invoice_id: "".to_string(),
            event_date: Utc
                .datetime_from_str("2020-12-21T15:51:21.126645Z", "%+")
                .unwrap(),
            event_type: InvoiceEventType::InvoiceReceivedEvent,
        };

        assert_eq!(
            "{\"invoiceId\":\"\",\
              \"eventDate\":\"2020-12-21T15:51:21.126645Z\",\
              \"eventType\":\"InvoiceReceivedEvent\"\
             }",
            serde_json::to_string(&ie).unwrap()
        );
    }
}
