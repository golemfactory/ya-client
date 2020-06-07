//! Provider part of the Payment API
use chrono::{DateTime, TimeZone};
use std::fmt::Display;
use std::sync::Arc;

use crate::{web::WebClient, web::WebInterface, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ya_client_model::payment::*;

#[derive(Default, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProviderApiConfig {
    // All timeouts are given in seconds.
    // None is interpreted by server as default timeout (60 seconds).
    pub send_debit_note_timeout: Option<f64>,
    pub cancel_debit_note_timeout: Option<f64>,
    pub send_invoice_timeout: Option<f64>,
    pub cancel_invoice_timeout: Option<f64>,
}

impl ProviderApiConfig {
    pub fn from_env() -> envy::Result<Self> {
        envy::from_env()
    }
}

#[derive(Clone)]
pub struct PaymentProviderApi {
    client: Arc<WebClient>,
    config: ProviderApiConfig,
}

impl WebInterface for PaymentProviderApi {
    const API_URL_ENV_VAR: &'static str = crate::payment::PAYMENT_URL_ENV_VAR;
    const API_SUFFIX: &'static str = PAYMENT_API_PATH;

    fn from_client(client: WebClient) -> Self {
        let config = ProviderApiConfig::default();
        PaymentProviderApi::new(&Arc::new(client), config)
    }
}

impl PaymentProviderApi {
    pub fn new(client: &Arc<WebClient>, config: ProviderApiConfig) -> Self {
        Self {
            client: client.clone(),
            config,
        }
    }

    pub async fn issue_debit_note(&self, debit_note: &NewDebitNote) -> Result<DebitNote> {
        self.client
            .post("provider/debitNotes")
            .send_json(debit_note)
            .json()
            .await
    }

    pub async fn get_debit_notes(&self) -> Result<Vec<DebitNote>> {
        self.client.get("provider/debitNotes").send().json().await
    }

    pub async fn get_debit_note(&self, debit_note_id: &str) -> Result<DebitNote> {
        let url = url_format!("provider/debitNotes/{debit_note_id}", debit_note_id);
        self.client.get(&url).send().json().await
    }

    pub async fn get_payments_for_debit_note(&self, debit_note_id: &str) -> Result<Vec<Payment>> {
        let url = url_format!(
            "provider/debitNotes/{debit_note_id}/payments",
            debit_note_id
        );
        self.client.get(&url).send().json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn send_debit_note(&self, debit_note_id: &str) -> Result<()> {
        let timeout = self.config.send_debit_note_timeout;
        let url = url_format!(
            "provider/debitNotes/{debit_note_id}/send",
            debit_note_id,
            #[query] timeout
        );
        self.client.post(&url).send().json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn cancel_debit_note(&self, debit_note_id: &str) -> Result<()> {
        let timeout = self.config.cancel_debit_note_timeout;
        let url = url_format!(
            "provider/debitNotes/{debit_note_id}/cancel",
            debit_note_id,
            #[query] timeout
        );
        self.client.post(&url).send().json().await
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
            "provider/debitNoteEvents",
            #[query] laterThan,
            #[query] timeout
        );
        self.client.get(&url).send().json().await
    }

    pub async fn issue_invoice(&self, invoice: &NewInvoice) -> Result<Invoice> {
        self.client
            .post("provider/invoices")
            .send_json(invoice)
            .json()
            .await
    }

    pub async fn get_invoices(&self) -> Result<Vec<Invoice>> {
        self.client.get("provider/invoices").send().json().await
    }

    pub async fn get_invoice(&self, invoice_id: &str) -> Result<Invoice> {
        let url = url_format!("provider/invoices/{invoice_id}", invoice_id);
        self.client.get(&url).send().json().await
    }

    pub async fn get_payments_for_invoice(&self, invoice_id: &str) -> Result<Vec<Payment>> {
        let url = url_format!("provider/invoices/{invoice_id}/payments", invoice_id);
        self.client.get(&url).send().json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn send_invoice(&self, invoice_id: &str) -> Result<()> {
        let timeout = self.config.send_invoice_timeout;
        let url = url_format!(
            "provider/invoices/{invoice_id}/send",
            invoice_id,
            #[query] timeout
        );
        self.client.post(&url).send().json().await
    }

    #[allow(non_snake_case)]
    #[rustfmt::skip]
    pub async fn cancel_invoice(&self, invoice_id: &str) -> Result<()> {
        let timeout = self.config.cancel_invoice_timeout;
        let url = url_format!(
            "provider/invoices/{invoice_id}/cancel",
            invoice_id,
            #[query] timeout
        );
        self.client.post(&url).send().json().await
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
            "provider/invoiceEvents",
            #[query] laterThan,
            #[query] timeout
        );
        self.client.get(&url).send().json().await
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
            "provider/payments",
            #[query] laterThan,
            #[query] timeout
        );
        self.client.get(&url).send().json().await
    }

    pub async fn get_payment(&self, payment_id: &str) -> Result<Payment> {
        let url = url_format!("provider/payments/{payment_id}", payment_id);
        self.client.get(&url).send().json().await
    }
}
