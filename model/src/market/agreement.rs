/*
 * Yagna Market API
 *
 *  ## Yagna Market The Yagna Market is a core component of the Yagna Network, which enables computational Offers and Demands circulation. The Market is open for all entities willing to buy computations (Demands) or monetize computational resources (Offers). ## Yagna Market API The Yagna Market API is the entry to the Yagna Market through which Requestors and Providers can publish their Demands and Offers respectively, find matching counterparty, conduct negotiations and make an agreement.  This version of Market API conforms with capability level 1 of the <a href=\"https://docs.google.com/document/d/1Zny_vfgWV-hcsKS7P-Kdr3Fb0dwfl-6T_cYKVQ9mkNg\"> Market API specification</a>.  Market API contains two roles: Requestors and Providers which are symmetrical most of the time (excluding agreement phase).
 *
 * The version of the OpenAPI document: 1.4.2
 *
 * Generated by: https://openapi-generator.tech
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{ErrorMessage, NodeId};

/// Agreement expresses the terms of the deal between Provider and Requestor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Agreement {
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    #[serde(rename = "demand")]
    pub demand: crate::market::Demand,
    #[serde(rename = "offer")]
    pub offer: crate::market::Offer,
    /// End of validity period.
    ///
    /// Agreement needs to be accepted, rejected or cancelled before this date; otherwise will expire.
    #[serde(rename = "validTo")]
    pub valid_to: DateTime<Utc>,
    /// date of the Agreement approval.
    #[serde(rename = "approvedDate", skip_serializing_if = "Option::is_none")]
    pub approved_date: Option<DateTime<Utc>>,
    /// See [State](enum.State.html).
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "proposedSignature", skip_serializing_if = "Option::is_none")]
    pub proposed_signature: Option<String>,
    #[serde(rename = "approvedSignature", skip_serializing_if = "Option::is_none")]
    pub approved_signature: Option<String>,
    #[serde(rename = "committedSignature", skip_serializing_if = "Option::is_none")]
    pub committed_signature: Option<String>,
}

impl Agreement {
    pub fn new(
        agreement_id: String,
        demand: crate::market::Demand,
        offer: crate::market::Offer,
        valid_to: DateTime<Utc>,
        state: State,
    ) -> Agreement {
        Agreement {
            agreement_id,
            demand,
            offer,
            valid_to,
            approved_date: None,
            state,
            proposed_signature: None,
            approved_signature: None,
            committed_signature: None,
        }
    }

    pub fn provider_id(&self) -> Result<&NodeId, ErrorMessage> {
        self.offer.provider_id()
    }

    pub fn requestor_id(&self) -> Result<&NodeId, ErrorMessage> {
        self.demand.requestor_id()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    /// Newly created by a Requestor (based on Proposal)
    #[serde(rename = "Proposal")]
    Proposal,
    /// Confirmed by a Requestor and send to Provider for approval
    #[serde(rename = "Pending")]
    Pending,
    /// Cancelled by a Requestor
    #[serde(rename = "Cancelled")]
    Cancelled,
    /// Rejected by a Provider
    #[serde(rename = "Rejected")]
    Rejected,
    /// Approved by both sides
    #[serde(rename = "Approved")]
    Approved,
    /// Not accepted, rejected nor cancelled within validity period
    #[serde(rename = "Expired")]
    Expired,
    /// Finished after approval
    #[serde(rename = "Terminated")]
    Terminated,
}
