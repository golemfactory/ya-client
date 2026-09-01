use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::market::Reason;

/// Body of `POST /agreements/{agreementId}/terminationNotice`: the Provider
/// announces its intention to terminate an Approved Agreement.
///
/// The Agreement stays `Approved` and existing Activities may continue; the
/// Requestor is expected to finish or migrate its work by
/// `termination_deadline`, after which the Provider may terminate the
/// Agreement. Only one notice may be recorded per Agreement and its payload
/// is immutable - a repeated request with the same payload is acknowledged
/// idempotently, a different payload is rejected.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementTerminationNotice {
    /// Absolute time by which the Provider expects the Requestor to finish
    /// or migrate its work. Must be later than the time the notice is posted.
    pub termination_deadline: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
}
