//! Requestor part of the Market API
use ya_client_model::market::{Agreement, AgreementProposal, Demand, Proposal, RequestorEvent};

use crate::{web::default_on_timeout, web::WebClient, web::WebInterface, Result};

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
    pub async fn subscribe(&self, demand: &Demand) -> Result<String> {
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
    #[rustfmt::skip]
    pub async fn collect(
        &self,
        subscription_id: &str,
        timeout: Option<f32>,
        #[allow(non_snake_case)]
        maxEvents: Option<i32>,
    ) -> Result<Vec<RequestorEvent>> {
        let url = url_format!(
            "demands/{subscription_id}/events",
            subscription_id,
            #[query] timeout,
            #[query] maxEvents
        );
        self.client.get(&url).send().json().await.or_else(default_on_timeout)
    }

    /// Responds with a bespoke Demand to received Offer.
    pub async fn counter_proposal(
        &self,
        demand_proposal: &Proposal,
        subscription_id: &str,
    ) -> Result<String> {
        let proposal_id = demand_proposal.prev_proposal_id()?;
        let url = url_format!(
            "demands/{subscription_id}/proposals/{proposal_id}",
            subscription_id,
            proposal_id
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
            proposal_id
        );
        self.client.get(&url).send().json().await
    }

    /// Rejects a bespoke Demand.
    pub async fn reject_proposal(
        &self,
        subscription_id: &str,
        proposal_id: &str,
    ) -> Result<String> {
        let url = url_format!(
            "demands/{subscription_id}/proposals/{proposal_id}",
            subscription_id,
            proposal_id
        );
        self.client.delete(&url).send().json().await
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
    pub async fn confirm_agreement(&self, agreement_id: &str) -> Result<String> {
        let url = url_format!("agreements/{agreement_id}/confirm", agreement_id);
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
    /// - The Provider is now ready to accept a request to start an Activity
    /// as described in the negotiated agreement.
    /// - The Requestor’s corresponding `wait_for_approval` call returns Ok after
    /// this on the Provider side.
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
            #[query] timeout
        );
        self.client.post(&url).send().json().await
    }

    /// Cancels agreement.
    /// Causes the awaiting `wait_for_approval` call to return with `Cancelled` response.
    /// Also the Provider's corresponding `approve_agreement` returns `Cancelled`.
    pub async fn cancel_agreement(&self, agreement_id: &str) -> Result<()> {
        let url = url_format!("agreements/{agreement_id}", agreement_id);
        self.client.delete(&url).send().json().await
    }

    /// Terminates approved Agreement.
    pub async fn terminate_agreement(&self, agreement_id: &str) -> Result<String> {
        let url = url_format!("agreements/{agreement_id}/terminate", agreement_id);
        self.client.post(&url).send().json().await
    }
}
