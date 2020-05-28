use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DocumentStatus {
    Issued,
    Received,
    Accepted,
    Rejected,
    Failed,
    Settled,
    Cancelled,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid {} EventType option: \"{0}\"")]
pub struct InvalidOption(String);

impl TryFrom<String> for DocumentStatus {
    type Error = InvalidOption;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "ISSUED" => Ok(DocumentStatus::Issued),
            "RECEIVED" => Ok(DocumentStatus::Received),
            "ACCEPTED" => Ok(DocumentStatus::Accepted),
            "REJECTED" => Ok(DocumentStatus::Rejected),
            "FAILED" => Ok(DocumentStatus::Failed),
            "SETTLED" => Ok(DocumentStatus::Settled),
            "CANCELLED" => Ok(DocumentStatus::Cancelled),
            _ => Err(InvalidOption(value)),
        }
    }
}

impl From<DocumentStatus> for String {
    fn from(invoice_status: DocumentStatus) -> Self {
        invoice_status.to_string()
    }
}

impl Display for DocumentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let str = serde_json::to_string(self)
            .unwrap()
            .trim_matches('"')
            .to_owned();
        write!(f, "{}", str)
    }
}
