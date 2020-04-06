use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Acceptance {
    pub total_amount_accepted: BigDecimal,
    pub allocation_id: String,
}

impl Acceptance {
    pub fn new(total_amount_accepted: BigDecimal, allocation_id: String) -> Acceptance {
        Acceptance {
            total_amount_accepted,
            allocation_id,
        }
    }
}
