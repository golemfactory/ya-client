//! Payment part of the Yagna API
pub mod provider;
pub mod requestor;

pub use provider::PaymentProviderApi;
pub use requestor::PaymentRequestorApi;
use std::str::FromStr;

pub(crate) const PAYMENT_URL_ENV_VAR: &str = "YAGNA_PAYMENT_URL";

pub(crate) fn parse_env_var<T: FromStr>(key: &str) -> Result<Option<T>, T::Err> {
    match std::env::var(key) {
        Ok(x) => Ok(Some(x.parse()?)),
        Err(_) => Ok(None),
    }
}
