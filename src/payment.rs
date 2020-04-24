//! Payment part of the Yagna API
pub mod provider;
pub mod requestor;

pub use provider::PaymentProviderApi;
pub use requestor::PaymentRequestorApi;

pub(crate) const PAYMENT_URL_ENV_VAR: &str = "YAGNA_PAYMENT_URL";
