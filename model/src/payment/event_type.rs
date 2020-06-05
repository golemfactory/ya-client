use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventType {
    Received,
    Accepted,
    Rejected,
    Cancelled,
    Settled,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid {} EventType option: \"{0}\"")]
pub struct InvalidOption(String);

impl TryFrom<String> for EventType {
    type Error = InvalidOption;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "RECEIVED" => Ok(EventType::Received),
            "ACCEPTED" => Ok(EventType::Accepted),
            "REJECTED" => Ok(EventType::Rejected),
            "CANCELLED" => Ok(EventType::Cancelled),
            "SETTLED" => Ok(EventType::Settled),
            _ => Err(InvalidOption(value)),
        }
    }
}

impl From<EventType> for String {
    fn from(event_type: EventType) -> Self {
        event_type.to_string()
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let str = serde_json::to_string(self)
            .unwrap()
            .trim_matches('"')
            .to_owned();
        write!(f, "{}", str)
    }
}
