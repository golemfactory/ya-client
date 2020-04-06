//! Async bindings for the Yagna API (REST)

#[macro_use]
pub mod web;

pub mod activity;
pub mod market;
pub mod payment;

pub mod error;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
