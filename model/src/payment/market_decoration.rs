use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketDecoration {
    pub properties: Vec<MarketProperty>,
    pub constraints: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketProperty {
    pub key: String,
    pub value: String,
}
