use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

pub const DEFAULT_ACK_TIMEOUT: f64 = 15.0; // seconds
pub const DEFAULT_EVENT_TIMEOUT: f64 = 5.0; // seconds

#[derive(Deserialize, Serialize)]
pub struct DebitNoteId {
    pub debit_note_id: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitNotePaymentsParams {
    pub debit_note_id: String,
    #[serde(default)]
    pub max_items: Option<u32>,
    #[serde(default)]
    pub after_timestamp: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct InvoiceId {
    pub invoice_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct AllocationId {
    pub allocation_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct PaymentId {
    pub payment_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Timeout {
    #[serde(default)]
    pub timeout: Option<f64>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventParams {
    #[serde(default)]
    pub timeout: Option<f64>,
    #[serde(default)]
    pub after_timestamp: Option<DateTime<Utc>>,
    #[serde(default)]
    pub max_events: Option<u32>,
    #[serde(default)]
    pub app_session_id: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterParams {
    #[serde(default)]
    pub max_items: Option<u32>,
    #[serde(default)]
    pub after_timestamp: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverNetworkParams {
    #[serde(flatten)]
    pub event_params: EventParams,
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub driver: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DriverStatusParams {
    #[serde(default)]
    pub network: Option<String>,
    #[serde(default)]
    pub driver: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct AllocationIds {
    #[serde(
        rename = "allocationIds",
        deserialize_with = "deserialize_comma_separated"
    )]
    pub allocation_ids: Vec<String>,
}

fn deserialize_comma_separated<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.split(',').map(str::to_string).collect())
}
