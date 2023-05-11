//! Yagna REST API client async binding

#[macro_use]
pub mod web;

pub mod activity;
pub mod market;
pub mod net;
pub mod p2p;
pub mod payment;

pub mod error;
pub use error::Error;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "sgx")]
mod sgx;
#[cfg(feature = "sgx")]
pub use sgx::SGX_CONFIG;

pub type Result<T> = std::result::Result<T, Error>;

pub use ya_client_model as model;
