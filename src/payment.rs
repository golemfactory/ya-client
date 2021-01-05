//! Payment part of the Yagna API
pub mod api;

pub use api::PaymentApi;

pub(crate) const PAYMENT_URL_ENV_VAR: &str = "YAGNA_PAYMENT_URL";
