//! Requestor part of the Market API
use ya_client_model::market::{
    Agreement, AgreementOperationEvent, AgreementProposal, Demand, NewDemand, NewProposal,
    Proposal, Reason, RequestorEvent,
};

use crate::{web::default_on_timeout, web::WebClient, web::WebInterface, Result};
use chrono::{DateTime, TimeZone};
use std::fmt::Display;

/// Bindings for Requestor part of the Market API.
#[derive(Clone)]
pub struct MarketRequestorApi {
    client: WebClient,
}

impl WebInterface for MarketRequestorApi {
    const API_URL_ENV_VAR: &'static str = crate::market::MARKET_URL_ENV_VAR;
    const API_SUFFIX: &'static str = ya_client_model::market::MARKET_API_PATH;

    fn from_client(client: WebClient) -> Self {
        MarketRequestorApi { client }
    }
}

impl MarketRequestorApi {
    /// Publishes Requestor capabilities via Demand.
    ///
    /// Demand object can be considered an "open" or public Demand, as it is not directed
    /// at a specific Provider, but rather is sent to the market so that the matching
    /// mechanism implementation can associate relevant Offers.
    ///
    /// **Note**: it is an "atomic" operation, ie. as soon as Subscription is placed,
    /// the Demand is published on the market.
    pub async fn subscribe(&self, demand: &NewDemand) -> Result<String> {
        self.client.post("demands").send_json(&demand).json().await
    }

    /// Fetches all active Demands which have been published by the Requestor.
    pub async fn get_demands(&self) -> Result<Vec<Demand>> {
        self.client.get("demands").send().json().await
    }

    /// Stop subscription by invalidating a previously published Demand.
    pub async fn unsubscribe(&self, subscription_id: &str) -> Result<String> {
        let url = url_format!("demands/{subscription_id}", subscription_id);
        self.client.delete(&url).send().json().await
    }

    /// Get events which have arrived from the market in response to the Demand
    /// published by the Requestor via  [`subscribe`](#method.subscribe).
    /// Returns collection of at most `max_events` `RequestorEvents` or times out.
    ///
    /// This is a blocking operation. It will not return until there is at
    /// least one new event.
    ///
    /// Returns Proposal related events:
    ///
    /// * `ProposalEvent` - Indicates that there is new Offer Proposal for
    /// this Demand.
    ///
    /// * `ProposalRejectedEvent` - Indicates that the Provider has rejected
    /// our previous Proposal related to this Demand. This effectively ends a
    /// Negotiation chain - it explicitly indicates that the sender will not
    /// create another counter-Proposal.
    ///
    /// * `PropertyQueryEvent` - not supported yet.
    ///
    /// **Note**: When `collectOffers` is waiting, simultaneous call to
    /// `unsubscribeDemand` on the same `subscriptionId` should result in
    /// "Subscription does not exist" error returned from `collectOffers`.
    ///
    /// **Note**: Specification requires this endpoint to support list of
    /// specific Proposal Ids to listen for messages related only to specific
    /// Proposals. This is not covered yet.
    #[rustfmt::skip]
    pub async fn collect(
        &self,
        subscription_id: &str,
        timeout: Option<f32>,
        max_events: Option<i32>,
    ) -> Result<Vec<RequestorEvent>> {
        let url = url_format!(
            "demands/{subscription_id}/events",
            subscription_id,
            #[query] timeout,
            #[query] max_events,
        );
        self.client.get(&url).send().json().await.or_else(default_on_timeout)
    }

    /// Responds with a bespoke Demand to received Offer.
    pub async fn counter_proposal(
        &self,
        demand_proposal: &NewProposal,
        subscription_id: &str,
        proposal_id: &str,
    ) -> Result<String> {
        let url = url_format!(
            "demands/{subscription_id}/proposals/{proposal_id}",
            subscription_id,
            proposal_id,
        );
        self.client
            .post(&url)
            .send_json(&demand_proposal)
            .json()
            .await
    }

    /// Fetches Proposal (Offer) with given id.
    pub async fn get_proposal(&self, subscription_id: &str, proposal_id: &str) -> Result<Proposal> {
        let url = url_format!(
            "demands/{subscription_id}/proposals/{proposal_id}",
            subscription_id,
            proposal_id,
        );
        self.client.get(&url).send().json().await
    }

    /// Rejects Proposal (Offer)
    ///
    /// Effectively ends a Negotiation chain - it explicitly indicates that
    /// the sender will not create another counter-Proposal.
    #[deprecated(
        since = "0.4.0",
        note = "Please use the reject_proposal_with_reason function instead"
    )]
    pub async fn reject_proposal(
        &self,
        subscription_id: &str,
        proposal_id: &str,
    ) -> Result<String> {
        let url = url_format!(
            "demands/{subscription_id}/proposals/{proposal_id}",
            subscription_id,
            proposal_id,
        );
        self.client.delete(&url).send().json().await
    }

    /// Rejects Proposal (Offer)
    ///
    /// Effectively ends a Negotiation chain - it explicitly indicates that
    /// the sender will not create another counter-Proposal.
    pub async fn reject_proposal_with_reason(
        &self,
        subscription_id: &str,
        proposal_id: &str,
        reason: &Option<Reason>,
    ) -> Result<String> {
        let url = url_format!(
            "demands/{subscription_id}/proposals/{proposal_id}",
            subscription_id,
            proposal_id,
        );
        self.client.post(&url).send_json(&reason).json().await
    }

    /// Creates Agreement from selected Proposal.
    ///
    /// Initiates the Agreement handshake phase.
    ///
    /// Formulates an Agreement artifact from the Proposal indicated by the
    /// received Proposal Id.
    ///
    /// The Approval Expiry Date is added to Agreement artifact and implies
    /// the effective timeout on the whole Agreement Confirmation sequence.
    ///
    /// A successful call to `create_agreement` shall immediately be followed
    /// by a `confirm_agreement` and `wait_for_approval` call in order to listen
    /// for responses from the Provider.
    ///
    /// **Note**: Moves given Proposal to `Approved` state.
    pub async fn create_agreement(&self, agreement: &AgreementProposal) -> Result<String> {
        self.client
            .post("agreements")
            .send_json(&agreement)
            .json()
            .await
    }

    /// Fetches agreement with given agreement id.
    pub async fn get_agreement(&self, agreement_id: &str) -> Result<Agreement> {
        let url = url_format!("agreements/{agreement_id}", agreement_id);
        self.client.get(&url).send().json().await
    }

    /// Sends Agreement draft to the Provider.
    /// Signs Agreement self-created via `create_agreement` and sends it to the Provider.
    #[rustfmt::skip]
    pub async fn confirm_agreement(
        &self,
        agreement_id: &str,
        app_session_id: Option<String>,
    ) -> Result<String> {
        let url = url_format!(
            "agreements/{agreement_id}/confirm",
            agreement_id,
            #[query] app_session_id,
        );
        self.client.post(&url).send().json().await
    }

    /// Waits for Agreement approval by the Provider.
    ///
    /// This is a blocking operation. The call may be aborted by Requestor caller
    /// code. After the call is aborted or timed out, another `wait_for_approval`
    /// call can be raised on the same `agreement_id`.
    ///
    /// It returns one of the following options:
    ///
    /// * `Approved` - Indicates that the Agreement has been approved by the Provider.
    ///   - The Provider is now ready to accept a request to start an Activity
    ///     as described in the negotiated agreement.
    ///   - The Requestor’s corresponding `wait_for_approval` call returns Ok after
    ///     this on the Provider side.
    ///
    /// * `Rejected` - Indicates that the Provider has called `reject_agreement`,
    /// which effectively stops the Agreement handshake. The Requestor may attempt
    /// to return to the Negotiation phase by sending a new Proposal.
    ///
    /// * `Cancelled` - Indicates that the Requestor himself has called
    /// `cancel_agreement`, which effectively stops the Agreement handshake.
    #[rustfmt::skip]
    pub async fn wait_for_approval(
        &self,
        agreement_id: &str,
        timeout: Option<f32>,
    ) -> Result<String> {
        let url = url_format!(
            "agreements/{agreement_id}/wait",
            agreement_id,
            #[query] timeout,
        );
        self.client.post(&url).send().json().await
    }

    /// Cancels Agreement.
    ///
    /// It is only possible before Requestor confirmed or Provider approved
    /// or rejected the Agreement, and before Expiration.
    ///
    /// Causes the awaiting `wait_for_approval` call to return with `Cancelled` response.
    /// Also the Provider's corresponding `approve_agreement` returns `Cancelled`.
    #[deprecated(
        since = "0.4.0",
        note = "Please use the cancel_agreement_with_reason function instead"
    )]
    pub async fn cancel_agreement(&self, agreement_id: &str) -> Result<()> {
        let url = url_format!("agreements/{agreement_id}", agreement_id);
        self.client.delete(&url).send().json().await
    }

    /// Cancels Agreement.
    ///
    /// It is only possible before Requestor confirmed or Provider approved
    /// or rejected the Agreement, and before Expiration.
    ///
    /// Causes the awaiting `wait_for_approval` call to return with `Cancelled` response.
    /// Also the Provider's corresponding `approve_agreement` returns `Cancelled`.
    pub async fn cancel_agreement_with_reason(
        &self,
        agreement_id: &str,
        reason: &Option<Reason>,
    ) -> Result<()> {
        let url = url_format!("agreements/{agreement_id}", agreement_id);
        self.client.post(&url).send_json(&reason).json().await
    }

    /// Terminates approved Agreement.
    pub async fn terminate_agreement(
        &self,
        agreement_id: &str,
        reason: &Option<Reason>,
    ) -> Result<String> {
        let url = url_format!("agreements/{agreement_id}/terminate", agreement_id);
        self.client.post(&url).send_json(&reason).json().await
    }

    /// Collects events related to an Agreement.
    ///
    /// This is a blocking operation. It will not return until there is
    /// at least one new event. All events are appearing on both sides equally.
    ///
    /// Returns Agreement related events:
    ///
    /// * `AgreementApprovedEvent` - Indicates that the Agreement has been
    ///   approved by the Provider.
    ///     - The Provider is now ready to accept a request to start an
    ///       Activity as described in the negotiated agreement.
    ///     - The Providers’s corresponding `approveAgreement` call
    ///       returns `Approved` after this event is emitted.
    ///
    /// * `AgreementRejectedEvent` - Indicates that the Provider has called
    ///   `rejectAgreement`, which effectively stops the Agreement handshake.
    ///   The Requestor may attempt to return to the Negotiation phase by
    ///   sending a new Proposal.
    ///
    /// * `AgreementCancelledEvent` - Indicates that the Requestor has called
    ///   `cancelAgreement`, which effectively stops the Agreement handshake.
    ///
    /// * `AgreementTerminatedEvent` - Indicates that the Agreement has been
    ///   terminated by specified party (contains signature).
    #[rustfmt::skip]
    pub async fn collect_agreement_events<Tz>(
        &self,
        timeout: Option<f32>,
        after_timestamp: Option<&DateTime<Tz>>,
        max_events: Option<i32>,
        app_session_id: Option<String>,
    ) -> Result<Vec<AgreementOperationEvent>>
        where
            Tz: TimeZone,
            Tz::Offset: Display,
    {
        let after_timestamp = after_timestamp.map(|dt| dt.to_rfc3339());
        let url = url_format!(
            "agreementEvents",
            #[query] timeout,
            #[query] after_timestamp,
            #[query] max_events,
            #[query] app_session_id,
        );
        self.client.get(&url).send().json().await.or_else(default_on_timeout)
    }
}
