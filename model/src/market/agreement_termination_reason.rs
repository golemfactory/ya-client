use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::market::AgreementTerminator;
use crate::market::Reason;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgreementTerminationReason {
    #[serde(rename = "eventDate")]
    pub event_date: DateTime<Utc>,
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    #[serde(rename = "terminator")]
    pub terminator: AgreementTerminator,
    #[serde(rename = "signature")]
    pub signature: String,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
}
