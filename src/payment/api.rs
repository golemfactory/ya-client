//!  part of the Payment API
use chrono::{DateTime, TimeZone, Utc};
use std::fmt::Display;
use std::sync::Arc;

use crate::{
    web::{default_on_timeout, url_format_obj, WebClient, WebInterface},
    Result,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ya_client_model::payment::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiConfig {
    // All timeouts are given in seconds.
    // None is interpreted by server as default timeout (60 seconds).

    // Requestor
    pub accept_debit_note_timeout: Option<f64>,
    pub reject_debit_note_timeout: Option<f64>,
    pub accept_invoice_timeout: Option<f64>,
    pub reject_invoice_timeout: Option<f64>,

    // Provider
    pub send_debit_note_timeout: Option<f64>,
    pub cancel_debit_note_timeout: Option<f64>,
    pub send_invoice_timeout: Option<f64>,
    pub cancel_invoice_timeout: Option<f64>,
}

impl ApiConfig {
    pub fn from_env() -> envy::Result<Self> {
        envy::from_env()
    }
}

#[derive(Clone)]
pub struct PaymentApi {
    client: WebClient,
    config: Arc<ApiConfig>,
}

impl WebInterface for PaymentApi {
    const API_URL_ENV_VAR: &'static str = crate::payment::PAYMENT_URL_ENV_VAR;
    const API_SUFFIX: &'static str = ya_client_model::payment::PAYMENT_API_PATH;

    fn from_client(client: WebClient) -> Self {
        let config = ApiConfig::default();
        let config = Arc::new(config);
        Self { client, config }
    }
}

impl PaymentApi {
    pub fn new(client: &WebClient, config: ApiConfig) -> Self {
        let config = config.into();
        let client = client.clone();
        Self { client, config }
    }

    // accounts

    pub async fn get_requestor_accounts(&self) -> Result<Vec<Account>> {
        self.client.get("requestorAccounts").send().json().await
    }

    pub async fn get_provider_accounts(&self) -> Result<Vec<Account>> {
        self.client.get("providerAccounts").send().json().await
    }

    // allocations

    pub async fn create_allocation(&self, allocation: &NewAllocation) -> Result<Allocation> {
        self.client
            .post("allocations")
            .send_json(allocation)
            .json()
            .await
    }

    pub async fn get_allocations<Tz>(
        &self,
        after_timestamp: Option<DateTime<Tz>>,
        max_items: Option<u32>,
    ) -> Result<Vec<Allocation>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        let input = params::FilterParams {
            after_timestamp: after_timestamp.map(|dt| dt.with_timezone(&Utc)),
            max_items,
        };
        let url = url_format_obj("allocations", &input);
        self.client.get(&url).send().json().await
    }

    pub async fn get_allocation(&self, allocation_id: &str) -> Result<Allocation> {
        let url = url_format!("allocations/{allocation_id}", allocation_id);
        self.client.get(&url).send().json().await
    }

    pub async fn amend_allocation(&self, allocation: &Allocation) -> Result<Allocation> {
        let allocation_id = &allocation.allocation_id;
        let url = url_format!("allocations/{allocation_id}", allocation_id);
        self.client.put(&url).send_json(allocation).json().await
    }

    pub async fn release_allocation(&self, allocation_id: &str) -> Result<()> {
        let url = url_format!("allocations/{allocation_id}", allocation_id);
        self.client.delete(&url).send().json().await
    }

    #[rustfmt::skip]
    pub async fn get_demand_decorations(
        &self,
        allocation_ids: Vec<String>,
    ) -> Result<MarketDecoration> {
        // *Not* using url_format_obj because serde_qs doesn't support comma-separated list serialization
        let allocation_ids = Some(allocation_ids.join(","));
        let url = url_format!(
            "demandDecorations",
            #[query] allocation_ids
        );
        self.client.get(&url).send().json().await
    }

    // debit_notes
    // Shared

    pub async fn get_debit_notes<Tz>(
        &self,
        after_timestamp: Option<DateTime<Tz>>,
        max_items: Option<u32>,
    ) -> Result<Vec<DebitNote>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        let input = params::FilterParams {
            after_timestamp: after_timestamp.map(|dt| dt.with_timezone(&Utc)),
            max_items,
        };
        let url = url_format_obj("debitNotes", &input);
        self.client.get(&url).send().json().await
    }

    pub async fn get_debit_note(&self, debit_note_id: &str) -> Result<DebitNote> {
        let url = url_format!("debitNotes/{debit_note_id}", debit_note_id);
        self.client.get(&url).send().json().await
    }

    pub async fn get_payments_for_debit_note<Tz>(
        &self,
        debit_note_id: &str,
        after_timestamp: Option<DateTime<Tz>>,
        max_items: Option<u32>,
    ) -> Result<Vec<Payment>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        // NOT IMPLEMENTED ON SERVER
        let input = params::FilterParams {
            after_timestamp: after_timestamp.map(|dt| dt.with_timezone(&Utc)),
            max_items,
        };
        let base_url = format!("debitNotes/{}/payments", debit_note_id);
        let url = url_format_obj(&base_url, &input);
        self.client.get(&url).send().json().await
    }

    pub async fn get_debit_note_events<Tz>(
        &self,
        after_timestamp: Option<&DateTime<Tz>>,
        timeout: Option<Duration>,
        max_events: Option<u32>,
        app_session_id: Option<String>,
    ) -> Result<Vec<DebitNoteEvent>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        let input = params::EventParams {
            after_timestamp: after_timestamp.map(|dt| dt.with_timezone(&Utc)),
            timeout: timeout.map(|d| d.as_secs_f64()),
            max_events,
            app_session_id,
        };
        let url = url_format_obj("debitNoteEvents", &input);
        self.client
            .get(&url)
            .send()
            .json()
            .await
            .or_else(default_on_timeout)
    }

    // debit_notes
    // Provider

    pub async fn issue_debit_note(&self, debit_note: &NewDebitNote) -> Result<DebitNote> {
        self.client
            .post("debitNotes")
            .send_json(debit_note)
            .json()
            .await
    }

    pub async fn send_debit_note(&self, debit_note_id: &str) -> Result<()> {
        let input = params::Timeout {
            timeout: self.config.send_debit_note_timeout,
        };
        let base_url = format!("debitNotes/{}/send", debit_note_id);
        let url = url_format_obj(&base_url, &input);
        self.client.post(&url).send().json().await
    }

    pub async fn cancel_debit_note(&self, debit_note_id: &str) -> Result<()> {
        let input = params::Timeout {
            timeout: self.config.cancel_debit_note_timeout,
        };
        let base_url = format!("debitNotes/{}/cancel", debit_note_id);
        let url = url_format_obj(&base_url, &input);
        self.client.post(&url).send().json().await
    }

    // debit_notes
    // Requestor

    pub async fn accept_debit_note(
        &self,
        debit_note_id: &str,
        acceptance: &Acceptance,
    ) -> Result<()> {
        let input = params::Timeout {
            timeout: self.config.accept_debit_note_timeout,
        };
        let base_url = format!("debitNotes/{}/accept", debit_note_id);
        let url = url_format_obj(&base_url, &input);
        self.client.post(&url).send_json(acceptance).json().await
    }

    pub async fn reject_debit_note(
        &self,
        debit_note_id: &str,
        rejection: &Rejection,
    ) -> Result<()> {
        let input = params::Timeout {
            timeout: self.config.reject_debit_note_timeout,
        };
        let base_url = format!("debitNotes/{}/reject", debit_note_id);
        let url = url_format_obj(&base_url, &input);
        self.client.post(&url).send_json(rejection).json().await
    }

    // invoices
    // Shared

    pub async fn get_invoices<Tz>(
        &self,
        after_timestamp: Option<DateTime<Tz>>,
        max_items: Option<u32>,
    ) -> Result<Vec<Invoice>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        let input = params::FilterParams {
            after_timestamp: after_timestamp.map(|dt| dt.with_timezone(&Utc)),
            max_items,
        };
        let url = url_format_obj("invoices", &input);
        self.client.get(&url).send().json().await
    }

    pub async fn get_invoice(&self, invoice_id: &str) -> Result<Invoice> {
        let url = url_format!("invoices/{invoice_id}", invoice_id);
        self.client.get(&url).send().json().await
    }

    pub async fn get_payments_for_invoice<Tz>(
        &self,
        invoice_id: &str,
        after_timestamp: Option<DateTime<Tz>>,
        max_items: Option<u32>,
    ) -> Result<Vec<Payment>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        // NOT IMPLEMENTED ON SERVER
        let input = params::FilterParams {
            after_timestamp: after_timestamp.map(|dt| dt.with_timezone(&Utc)),
            max_items,
        };
        let base_url = format!("invoices/{}/payments", invoice_id);
        let url = url_format_obj(&base_url, &input);
        self.client.get(&url).send().json().await
    }

    pub async fn get_invoice_events<Tz>(
        &self,
        after_timestamp: Option<&DateTime<Tz>>,
        timeout: Option<Duration>,
        max_events: Option<u32>,
        app_session_id: Option<String>,
    ) -> Result<Vec<InvoiceEvent>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        let input = params::EventParams {
            after_timestamp: after_timestamp.map(|dt| dt.with_timezone(&Utc)),
            timeout: timeout.map(|d| d.as_secs_f64()),
            max_events,
            app_session_id,
        };

        let url = url_format_obj("invoiceEvents", &input);
        self.client
            .get(&url)
            .send()
            .json()
            .await
            .or_else(default_on_timeout)
    }

    // invoices
    // Provider

    pub async fn issue_invoice(&self, invoice: &NewInvoice) -> Result<Invoice> {
        self.client.post("invoices").send_json(invoice).json().await
    }

    pub async fn send_invoice(&self, invoice_id: &str) -> Result<()> {
        let input = params::Timeout {
            timeout: self.config.send_invoice_timeout,
        };
        let base_url = format!("invoices/{}/send", invoice_id);
        let url = url_format_obj(&base_url, &input);
        self.client.post(&url).send().json().await
    }

    pub async fn cancel_invoice(&self, invoice_id: &str) -> Result<()> {
        let input = params::Timeout {
            timeout: self.config.cancel_invoice_timeout,
        };
        let base_url = format!("invoices/{}/cancel", invoice_id);
        let url = url_format_obj(&base_url, &input);
        self.client.post(&url).send().json().await
    }

    // invoices
    // Requestor

    pub async fn accept_invoice(&self, invoice_id: &str, acceptance: &Acceptance) -> Result<()> {
        let input = params::Timeout {
            timeout: self.config.accept_invoice_timeout,
        };
        let base_url = format!("invoices/{}/accept", invoice_id);
        let url = url_format_obj(&base_url, &input);
        self.client.post(&url).send_json(acceptance).json().await
    }

    pub async fn reject_invoice(&self, invoice_id: &str, rejection: &Rejection) -> Result<()> {
        let input = params::Timeout {
            timeout: self.config.reject_invoice_timeout,
        };
        let base_url = format!("invoices/{}/reject", invoice_id);
        let url = url_format_obj(&base_url, &input);
        self.client.post(&url).send_json(rejection).json().await
    }

    // payments

    pub async fn get_payments<Tz>(
        &self,
        after_timestamp: Option<&DateTime<Tz>>,
        timeout: Option<Duration>,
        max_events: Option<u32>,
        app_session_id: Option<String>,
    ) -> Result<Vec<Payment>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        let input = params::EventParams {
            after_timestamp: after_timestamp.map(|dt| dt.with_timezone(&Utc)),
            timeout: timeout.map(|d| d.as_secs_f64()),
            max_events,
            app_session_id,
        };
        let url = url_format_obj("payments", &input);
        self.client
            .get(&url)
            .send()
            .json()
            .await
            .or_else(default_on_timeout)
    }

    pub async fn get_payment(&self, payment_id: &str) -> Result<Payment> {
        let url = url_format!("payments/{payment_id}", payment_id);
        self.client.get(&url).send().json().await
    }
}
