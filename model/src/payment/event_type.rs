use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventType {
    Received,
    Accepted,
    Rejected,
    Cancelled,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid {} EventType option: \"{0}\"")]
pub struct InvalidOption(String);

impl TryFrom<String> for EventType {
    type Error = InvalidOption;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(v) = match value.as_str() {
            "RECEIVED" => Some(EventType::Received),
            "ACCEPTED" => Some(EventType::Accepted),
            "REJECTED" => Some(EventType::Rejected),
            "CANCELLED" => Some(EventType::Cancelled),
            _ => None,
        } {
            return Ok(v);
        }

        Err(InvalidOption(value))
    }
}

impl From<EventType> for String {
    fn from(event_type: EventType) -> Self {
        serde_json::to_string(&event_type)
            .unwrap()
            .trim_matches('"')
            .to_owned()
    }
}
