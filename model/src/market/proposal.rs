/*
 * Yagna Market API
 *
 * The version of the OpenAPI document: 1.6.1
 *
 * Generated by: https://openapi-generator.tech
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{ErrorMessage, NodeId};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Proposal {
    /// The object which includes all the Proposal properties.
    /// This is a JSON object in \"flat convention\" - where keys are full
    /// property names and their values indicate properties.
    ///
    /// The value's Javascript type shall conform with the type of the
    /// property (as indicated in Golem Standards).
    ///
    /// ### Example property object:
    /// ```json
    /// {
    ///     "golem.com.pricing.model": "linear",
    ///     "golem.com.pricing.model.linear.coeffs": [0.001, 0.002, 0.0],
    ///     "golem.com.scheme": "payu",
    ///     "golem.com.scheme.payu.interval_sec": 6.0,
    ///     "golem.com.usage.vector": ["golem.usage.duration_sec", "golem.usage.cpu_sec"],
    ///     "golem.inf.cpu.architecture": "x86_64",
    ///     "golem.inf.cpu.cores": 4,
    ///     "golem.inf.cpu.threads": 7,
    ///     "golem.inf.mem.gib": 10.612468048930168,
    ///     "golem.inf.storage.gib": 81.7227783203125,
    ///     "golem.node.debug.subnet": "market-devnet",
    ///     "golem.node.id.name": "tworec@mf-market-devnet",
    ///     "golem.runtime.name": "vm",
    ///     "golem.runtime.version@v": "0.1.0"
    /// }
    /// ```        #[serde(rename = "properties")]
    pub properties: serde_json::Value,
    #[serde(rename = "constraints")]
    pub constraints: String,
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
    #[serde(rename = "issuerId")]
    pub issuer_id: NodeId,
    /// * `Initial` - proposal arrived from the market as response to subscription
    /// * `Draft` - bespoke counter-proposal issued by one party directly to other party (negotiation phase)
    /// * `Rejected` by other party
    /// * `Accepted` - promoted into the Agreement draft
    /// * `Expired` - not accepted nor rejected before validity period
    #[serde(rename = "state")]
    pub state: State,
    /// Object creation timestamp
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    /// id of the proposal from other side which this proposal responds to
    #[serde(rename = "prevProposalId", skip_serializing_if = "Option::is_none")]
    pub prev_proposal_id: Option<String>,
}

impl Proposal {
    pub fn new(
        properties: serde_json::Value,
        constraints: String,
        proposal_id: String,
        issuer_id: NodeId,
        state: State,
        timestamp: DateTime<Utc>,
    ) -> Proposal {
        Proposal {
            properties,
            constraints,
            proposal_id,
            issuer_id,
            state,
            timestamp,
            prev_proposal_id: None,
        }
    }

    pub fn prev_proposal_id(&self) -> Result<&String, ErrorMessage> {
        self.prev_proposal_id
            .as_ref()
            .ok_or("no previous proposal id".into())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Initial")]
    /// Proposal arrived from the market as response to subscription
    Initial,
    #[serde(rename = "Draft")]
    /// Bespoke counter-proposal issued by one party directly to other party (negotiation phase)
    Draft,
    #[serde(rename = "Rejected")]
    /// Rejected by other party
    Rejected,
    #[serde(rename = "Accepted")]
    /// Promoted to the Agreement draft
    Accepted,
    #[serde(rename = "Expired")]
    /// Not accepted nor rejected before validity period
    Expired,
}
