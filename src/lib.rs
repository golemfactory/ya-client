//! Yagna REST API client async binding

#[macro_use]
pub mod web;

pub mod activity;
pub mod market;
pub mod payment;

pub mod error;
pub use error::Error;

#[cfg(feature = "cli")]
pub mod cli;

pub type Result<T> = std::result::Result<T, Error>;

pub use ya_client_model as model;
