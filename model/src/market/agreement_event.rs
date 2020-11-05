/*
 * Yagna Market API
 *
 * The version of the OpenAPI document: 1.6.1
 *
 * Generated by: https://openapi-generator.tech
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::market::Reason;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "eventtype")]
pub enum AgreementOperationEvent {
    #[serde(rename = "AgreementApprovedEvent")]
    AgreementApprovedEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "agreementId")]
        agreement_id: String,
    },
    #[serde(rename = "AgreementRejectedEvent")]
    AgreementRejectedEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "agreementId")]
        agreement_id: String,
        #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
        reason: Option<Reason>,
    },
    #[serde(rename = "AgreementCancelledEvent")]
    AgreementCancelledEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "agreementId")]
        agreement_id: String,
        #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
        reason: Option<Reason>,
    },
    #[serde(rename = "AgreementTerminatedEvent")]
    AgreementTerminatedEvent {
        #[serde(rename = "eventDate")]
        event_date: DateTime<Utc>,
        #[serde(rename = "agreementId")]
        agreement_id: String,
        #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
        reason: Option<Reason>,
    },
}
