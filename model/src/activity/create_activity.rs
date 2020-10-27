use serde::{Deserialize, Serialize};

#[cfg(feature = "sgx")]
use crate::activity::sgx_credentials::SgxCredentials;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateActivityRequest {
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    #[serde(rename = "requestorPubKey", skip_serializing_if = "Option::is_none")]
    pub requestor_pub_key: Option<String>,
}

impl CreateActivityRequest {
    pub fn new(agreement_id: String) -> CreateActivityRequest {
        CreateActivityRequest {
            agreement_id,
            requestor_pub_key: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateActivityResult {
    #[serde(rename = "activityId")]
    pub activity_id: String,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}

impl CreateActivityResult {
    pub fn new(activity_id: String) -> CreateActivityResult {
        CreateActivityResult {
            activity_id,
            credentials: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Credentials {
    #[cfg(feature = "sgx")]
    #[serde(rename = "sgx")]
    Sgx(SgxCredentials),
    #[cfg(not(feature = "sgx"))]
    #[serde(rename = "sgx")]
    Sgx(serde_json::Value),
}
