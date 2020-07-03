use crate::payment::{ActivityPayment, AgreementPayment};
use crate::NodeId;
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub payment_id: String,
    pub payer_id: NodeId,
    pub payee_id: NodeId,
    pub payer_addr: String,
    pub payee_addr: String,
    pub payment_platform: String,
    pub amount: BigDecimal,
    pub timestamp: DateTime<Utc>,
    pub agreement_payments: Vec<AgreementPayment>,
    pub activity_payments: Vec<ActivityPayment>,
    pub details: String,
}
