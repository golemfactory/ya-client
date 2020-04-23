use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvoiceStatus {
    Issued,
    Received,
    Accepted,
    Rejected,
    Failed,
    Settled,
    Cancelled,
}

impl From<String> for InvoiceStatus {
    fn from(value: String) -> Self {
        serde_json::from_str(&format!("\"{}\"", value)).unwrap()
    }
}

impl From<InvoiceStatus> for String {
    fn from(invoice_status: InvoiceStatus) -> Self {
        invoice_status.to_string()
    }
}

impl ToString for InvoiceStatus {
    fn to_string(&self) -> String {
        serde_json::to_string(self)
            .unwrap()
            .trim_matches('"')
            .to_owned()
    }
}
