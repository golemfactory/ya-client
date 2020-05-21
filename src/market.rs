//! Market part of the Yagna API
mod provider;
mod requestor;

pub use provider::MarketProviderApi;
pub use requestor::MarketRequestorApi;

pub(crate) const MARKET_URL_ENV_VAR: &str = "YAGNA_MARKET_URL";
