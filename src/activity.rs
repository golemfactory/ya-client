//! Activity part of the Yagna API

#[cfg(feature = "provider")]
mod provider;
#[cfg(feature = "provider")]
pub use provider::ActivityProviderApi;

#[cfg(feature = "requestor")]
mod requestor;
#[cfg(feature = "requestor")]
pub use requestor::{ActivityRequestorApi, ActivityRequestorControlApi, ActivityRequestorStateApi};

#[cfg(all(feature = "sgx", feature = "requestor"))]
pub use requestor::control::sgx::SecureActivityRequestorApi;

pub(crate) const ACTIVITY_URL_ENV_VAR: &str = "YAGNA_ACTIVITY_URL";
