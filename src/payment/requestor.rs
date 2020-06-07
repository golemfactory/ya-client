//! Requestor part of the Payment API
use chrono::{DateTime, TimeZone};
use std::fmt::Display;
use std::sync::Arc;

use crate::{web::default_on_timeout, web::WebClient, web::WebInterface, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ya_client_model::payment::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RequestorApiConfig {
    // All timeouts are given in seconds.
    // None is interpreted by server as default timeout (60 seconds).
    pub accept_debit_note_timeout: Option<f64>,
    pub reject_debit_note_timeout: Option<f64>,
    pub accept_invoice_timeout: Option<f64>,
    pub reject_invoice_timeout: Option<f64>,
}

impl RequestorApiConfig {
    pub fn from_env() -> envy::Result<Self> {
        envy::from_env()
    }
}

#[derive(Clone)]
pub struct PaymentRequestorApi {
    client: WebClient,
    config: Arc<RequestorApiConfig>,
}

impl WebInterface for PaymentRequestorApi {
    const API_URL_ENV_VAR: &'static str = crate::payment::PAYMENT_URL_ENV_VAR;
    const API_SUFFIX: &'static str = ya_client_model::payment::PAYMENT_API_PATH;

    fn from_client(client: WebClient) -> Self {
        let config = RequestorApiConfig::default();
        let config = Arc::new(config);
        Self { client, config }
    }
}

impl PaymentRequestorApi {
    pub fn new(client: &WebClient, config: RequestorApiConfig) -> Self {
        let config = config.into();
        let client = client.clone();
        Self { client, config }
    }

    pub async fn get_debit_notes(&self) -> Result<Vec<DebitNote>> {
        self.client.get("requestor/debitNotes").send().json().await
    }

    pub async fn get_debit_note(&self, debit_note_id: &str) -> Result<DebitNote> {
        let url = url_format!("requestor/debitNotes/{debit_note_id}", debit_note_id);
        self.client.get(&url).send().json().await
    }

    pub async fn get_payments_for_debit_note(&self, debit_note_id: &str) -> Result<Vec<Payment>> {
        let url = url_format!(
            "requestor/debitNotes/{debit_note_id}/payments",
            debit_note_id
        );
        self.client.get(&url).send().json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn accept_debit_note(
        &self,
        debit_note_id: &str,
        acceptance: &Acceptance,
    ) -> Result<()> {
        let timeout = self.config.accept_debit_note_timeout;
        let url = url_format!(
            "requestor/debitNotes/{debit_note_id}/accept",
            debit_note_id,
            #[query] timeout
        );
        self.client.post(&url).send_json(acceptance).json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn reject_debit_note(
        &self,
        debit_note_id: &str,
        rejection: &Rejection,
    ) -> Result<()> {
        let timeout = self.config.reject_debit_note_timeout;
        let url = url_format!(
            "requestor/debitNotes/{debit_note_id}/reject",
            debit_note_id,
            #[query] timeout
        );
        self.client.post(&url).send_json(rejection).json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn get_debit_note_events<Tz>(
        &self,
        later_than: Option<&DateTime<Tz>>,
        timeout: Option<Duration>,
    ) -> Result<Vec<DebitNoteEvent>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        let laterThan = later_than.map(|dt| dt.to_rfc3339());
        let timeout = timeout.map(|d| d.as_secs_f64());
        let url = url_format!(
            "requestor/debitNoteEvents",
            #[query] laterThan,
            #[query] timeout
        );
        self.client.get(&url).send().json().await.or_else(default_on_timeout)
    }

    pub async fn get_invoices(&self) -> Result<Vec<Invoice>> {
        self.client.get("requestor/invoices").send().json().await
    }

    pub async fn get_invoice(&self, invoice_id: &str) -> Result<Invoice> {
        let url = url_format!("requestor/invoices/{invoice_id}", invoice_id);
        self.client.get(&url).send().json().await
    }

    pub async fn get_payments_for_invoice(&self, invoice_id: &str) -> Result<Vec<Payment>> {
        let url = url_format!("requestor/invoices/{invoice_id}/payments", invoice_id);
        self.client.get(&url).send().json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn accept_invoice(
        &self,
        invoice_id: &str,
        acceptance: &Acceptance,
    ) -> Result<()> {
        let timeout = self.config.accept_invoice_timeout;
        let url = url_format!(
            "requestor/invoices/{invoice_id}/accept",
            invoice_id,
            #[query] timeout
        );
        self.client.post(&url).send_json(acceptance).json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn reject_invoice(&self, invoice_id: &str, rejection: &Rejection) -> Result<()> {
        let timeout = self.config.reject_invoice_timeout;
        let url = url_format!(
            "requestor/invoices/{invoice_id}/reject",
            invoice_id,
            #[query] timeout
        );
        self.client.post(&url).send_json(rejection).json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn get_invoice_events<Tz>(
        &self,
        later_than: Option<&DateTime<Tz>>,
        timeout: Option<Duration>,
    ) -> Result<Vec<InvoiceEvent>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        let laterThan = later_than.map(|dt| dt.to_rfc3339());
        let timeout = timeout.map(|d| d.as_secs_f64());
        let url = url_format!(
            "requestor/invoiceEvents",
            #[query] laterThan,
            #[query] timeout
        );
        self.client.get(&url).send().json().await.or_else(default_on_timeout)
    }

    pub async fn create_allocation(&self, allocation: &NewAllocation) -> Result<Allocation> {
        self.client
            .post("requestor/allocations")
            .send_json(allocation)
            .json()
            .await
    }

    pub async fn get_allocations(&self) -> Result<Vec<Allocation>> {
        self.client.get("requestor/allocations").send().json().await
    }

    pub async fn get_allocation(&self, allocation_id: &str) -> Result<Allocation> {
        let url = url_format!("requestor/allocations/{allocation_id}", allocation_id);
        self.client.get(&url).send().json().await
    }

    pub async fn amend_allocation(&self, allocation: &Allocation) -> Result<Allocation> {
        let allocation_id = &allocation.allocation_id;
        let url = url_format!("requestor/allocations/{allocation_id}", allocation_id);
        self.client.put(&url).send_json(allocation).json().await
    }

    pub async fn release_allocation(&self, allocation_id: &str) -> Result<()> {
        let url = url_format!("requestor/allocations/{allocation_id}", allocation_id);
        self.client.delete(&url).send().json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn get_payments<Tz>(
        &self,
        later_than: Option<&DateTime<Tz>>,
        timeout: Option<Duration>,
    ) -> Result<Vec<Payment>>
    where
        Tz: TimeZone,
        Tz::Offset: Display,
    {
        let laterThan = later_than.map(|dt| dt.to_rfc3339());
        let timeout = timeout.map(|d| d.as_secs_f64());
        let url = url_format!(
            "requestor/payments",
            #[query] laterThan,
            #[query] timeout
        );
        self.client.get(&url).send().json().await.or_else(default_on_timeout)
    }

    pub async fn get_payment(&self, payment_id: &str) -> Result<Payment> {
        let url = url_format!("requestor/payments/{payment_id}", payment_id);
        self.client.get(&url).send().json().await
    }
}
