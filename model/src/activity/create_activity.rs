use serde::{Deserialize, Serialize};

#[cfg(feature = "sgx")]
use crate::activity::sgx_credentials::SgxCredentials;

/// Create new activity request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateActivityRequest {
    /// Agreement identifier for which we want to create activity.
    #[serde(rename = "agreementId")]
    pub agreement_id: String,
    /// For secure computing (sgx) scenario, it is requestor public key needed for e2e encyprtion.
    #[serde(rename = "requestorPubKey", skip_serializing_if = "Option::is_none")]
    pub requestor_pub_key: Option<String>,
}

impl CreateActivityRequest {
    /// New request for given agreement id.
    pub fn new(agreement_id: String) -> CreateActivityRequest {
        CreateActivityRequest {
            agreement_id,
            requestor_pub_key: None,
        }
    }
}

/// Create new activity response.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateActivityResult {
    /// Id of new activity.
    #[serde(rename = "activityId")]
    pub activity_id: String,
    /// For secure computing (sgx) scenario, it is compute unit credentials.
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
}

impl CreateActivityResult {
    /// Creates result for simple scenario
    pub fn new(activity_id: String) -> CreateActivityResult {
        CreateActivityResult {
            activity_id,
            credentials: None,
        }
    }
}

#[doc(hidden)]
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
