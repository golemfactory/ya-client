//! Activity part of the Yagna API
mod provider;
mod requestor;

pub use provider::ActivityProviderApi;
pub use requestor::control::ActivityRequestorControlApi;
pub use requestor::state::ActivityRequestorStateApi;
pub use requestor::ActivityRequestorApi;

#[cfg(feature = "sgx")]
pub use requestor::control::sgx::SecureActivityRequestorApi;

pub(crate) const ACTIVITY_URL_ENV_VAR: &str = "YAGNA_ACTIVITY_URL";
