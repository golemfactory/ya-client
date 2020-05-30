//! Provider part of the Market API
use ya_client_model::market::{Agreement, Offer, Proposal, ProviderEvent, MARKET_API_PATH};

use crate::{web::default_on_timeout, web::WebClient, web::WebInterface, Result};

/// Bindings for Provider part of the Market API.
#[derive(Clone)]
pub struct MarketProviderApi {
    client: WebClient,
}

impl WebInterface for MarketProviderApi {
    const API_URL_ENV_VAR: &'static str = crate::market::MARKET_URL_ENV_VAR;
    const API_SUFFIX: &'static str = MARKET_API_PATH;

    fn from_client(client: WebClient) -> Self {
        MarketProviderApi { client }
    }
}

impl MarketProviderApi {
    /// Publish Provider’s service capabilities (`Offer`) on the market to declare an
    /// interest in Demands meeting specified criteria.
    pub async fn subscribe(&self, offer: &Offer) -> Result<String> {
        self.client.post("offers").send_json(&offer).json().await
    }

    /// Fetches all active Offers which have been published by the Provider.
    pub async fn get_offers(&self) -> Result<Vec<Offer>> {
        self.client.get("offers").send().json().await
    }

    /// Stop subscription by invalidating a previously published Offer.
    ///
    /// Stop receiving Proposals.
    /// **Note**: this will terminate all pending `collect_demands` calls on this subscription.
    /// This implies, that client code should not `unsubscribe_offer` before it has received
    /// all expected/useful inputs from `collect_demands`.
    pub async fn unsubscribe(&self, subscription_id: &str) -> Result<String> {
        let url = url_format!("offers/{subscription_id}", subscription_id);
        self.client.delete(&url).send().json().await
    }

    /// Get events which have arrived from the market in response to the Offer
    /// published by the Provider via  [`subscribe`](#method.subscribe).
    /// Returns collection of at most `max_events` `ProviderEvents` or times out.
    #[rustfmt::skip]
    pub async fn collect(
        &self,
        subscription_id: &str,
        timeout: Option<f32>,
        #[allow(non_snake_case)]
        maxEvents: Option<i32>,
    ) -> Result<Vec<ProviderEvent>> {
        let url = url_format!(
            "offers/{subscription_id}/events",
            subscription_id,
            #[query] timeout,
            #[query] maxEvents
        );

        self.client.get(&url).send().json().await.or_else(default_on_timeout)
    }

    /// Fetches Proposal (Demand) with given id.
    pub async fn get_proposal(&self, subscription_id: &str, proposal_id: &str) -> Result<Proposal> {
        let url = url_format!(
            "offers/{subscription_id}/proposals/{proposal_id}",
            subscription_id,
            proposal_id
        );
        self.client.get(&url).send().json().await
    }

    /// Rejects Proposal (Demand).
    /// Effectively ends a Negotiation chain - it explicitly indicates that the sender
    /// will not create another counter-Proposal.
    pub async fn reject_proposal(
        &self,
        subscription_id: &str,
        proposal_id: &str,
    ) -> Result<String> {
        let url = url_format!(
            "offers/{subscription_id}/proposals/{proposal_id}",
            subscription_id,
            proposal_id
        );
        self.client.delete(&url).send().json().await
    }

    /// Responds with a bespoke Offer to received Demand.
    /// Creates and sends a modified version of original Offer (a
    /// counter-proposal) adjusted to previously received Proposal (ie. Demand).
    /// Changes Proposal state to `Draft`. Returns created Proposal id.
    pub async fn counter_proposal(
        &self,
        offer_proposal: &Proposal,
        subscription_id: &str,
    ) -> Result<String> {
        let proposal_id = offer_proposal.prev_proposal_id()?;
        let url = url_format!(
            "offers/{subscription_id}/proposals/{proposal_id}",
            subscription_id,
            proposal_id
        );
        self.client
            .post(&url)
            .send_json(&offer_proposal)
            .json()
            .await
    }

    /// Approves Agreement proposed by the Reqestor.
    ///
    /// This is a blocking operation. The call may be aborted by Provider caller
    /// code. After the call is aborted or timed out, another `approve_agreement`
    /// call can be raised on the same `agreement_id`.
    ///
    /// It returns one of the following options:
    ///
    /// * `Ok` - Indicates that the approved Agreement has been successfully
    /// delivered to the Requestor and acknowledged.
    /// - The Requestor side has been notified about the Provider’s commitment
    /// to the Agreement.
    /// - The Provider is now ready to accept a request to start an Activity
    /// as described in the negotiated agreement.
    /// - The Requestor’s corresponding `wait_for_approval` call returns Ok after
    /// the one on the Provider side.
    ///
    /// * `Cancelled` - Indicates that before delivering the approved Agreement,
    /// the Requestor has called `cancel_agreement`, thus invalidating the
    /// Agreement. The Provider may attempt to return to the Negotiation phase
    /// by sending a new Proposal.
    ///
    /// **Note**: It is expected from the Provider node implementation to “ring-fence”
    /// the resources required to fulfill the Agreement before the `approve_agreement`
    /// is sent. However, the resources should not be fully committed until `Ok`
    /// response is received from the `approve_agreement` call.
    ///
    ///
    /// **Note**: Mutually exclusive with `reject_agreement`.
    #[rustfmt::skip]
    pub async fn approve_agreement(
        &self,
        agreement_id: &str,
        timeout: Option<f32>,
    ) -> Result<String> {
        let url = url_format!(
            "agreements/{agreement_id}/approve",
            agreement_id,
            #[query] timeout
        );
        self.client.post(&url).send().json().await
    }

    /// Rejects Agreement proposed by the Requestor.
    ///
    /// The Requestor side is notified about the Provider’s decision to reject
    /// a negotiated agreement. This effectively stops the Agreement handshake.
    ///
    /// **Note**: Mutually exclusive with `approve_agreement`.
    pub async fn reject_agreement(&self, agreement_id: &str) -> Result<String> {
        let url = url_format!("agreements/{agreement_id}/reject", agreement_id);
        self.client.post(&url).send().json().await
    }

    /// Terminates approved Agreement.
    pub async fn terminate_agreement(&self, agreement_id: &str) -> Result<String> {
        let url = url_format!("agreements/{agreement_id}/terminate", agreement_id);
        self.client.post(&url).send().json().await
    }

    /// Fetches agreement with given agreement id.
    pub async fn get_agreement(&self, agreement_id: &str) -> Result<Agreement> {
        let url = url_format!("agreements/{agreement_id}", agreement_id);
        self.client.get(&url).send().json().await
    }
}
