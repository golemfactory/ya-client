//! Activity part of the Yagna API
mod provider;
mod requestor;

pub use provider::ActivityProviderApi;
pub use requestor::ActivityRequestorApi;
pub use requestor::control::ActivityRequestorControlApi;
pub use requestor::state::ActivityRequestorStateApi;

#[cfg(feature = "sgx")]
pub use requestor::control::sgx::SecureActivityRequestorApi;

pub(crate) const ACTIVITY_URL_ENV_VAR: &str = "YAGNA_ACTIVITY_URL";
