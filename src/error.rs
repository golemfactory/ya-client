//! Error definitions and mappings
use awc::error::{JsonPayloadError, PayloadError, SendRequestError};
use awc::http::{Method, StatusCode};

use ya_client_model::ErrorMessage;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("AWC error requesting {method} {url}: {msg}")]
    SendRequestError {
        msg: String,
        method: Method,
        url: String,
    },
    #[error("AWC timeout requesting {method} {url}: {msg}")]
    TimeoutError {
        msg: String,
        method: Method,
        url: String,
    },
    #[error("AWC payload error: {0}")]
    PayloadError(PayloadError),
    #[error("AWC JSON payload error: {0}")]
    JsonPayloadError(JsonPayloadError),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::error::Error),
    #[error("HTTP error requesting {method} {url}: {code}; msg: '{msg}'")]
    HttpError {
        code: StatusCode,
        msg: String,
        method: Method,
        url: String,
    },
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    #[error("Serde JSON error: {0}")]
    SerdeJsonError(serde_json::Error),
    #[error("Invalid address: {0}")]
    InvalidAddress(std::convert::Infallible),
    #[error("Invalid header: {0}")]
    InvalidHeaderName(#[from] awc::http::header::InvalidHeaderName),
    #[error("Invalid header: {0}")]
    InvalidHeaderValue(#[from] awc::http::header::InvalidHeaderValue),
    #[error("Invalid UTF8 string: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Invalid UTF8 string: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    ApiErrorMessage(#[from] ErrorMessage),
    #[error("Internal ya-client error: {0}")]
    InternalError(String),
    #[error("Event stream error: {0}")]
    EventStreamError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<PayloadError> for Error {
    fn from(e: PayloadError) -> Self {
        Error::PayloadError(e)
    }
}

impl From<JsonPayloadError> for Error {
    fn from(e: JsonPayloadError) -> Self {
        Error::JsonPayloadError(e)
    }
}

impl From<awc::error::WsClientError> for Error {
    fn from(e: awc::error::WsClientError) -> Self {
        Error::WebSocketError(e.to_string())
    }
}

impl Error {
    pub(crate) fn from_request(err: SendRequestError, method: Method, url: String) -> Self {
        let msg = err.to_string();
        match err {
            SendRequestError::Timeout => Error::TimeoutError { msg, method, url },
            _ => Error::SendRequestError { msg, method, url },
        }
    }

    pub(crate) fn from_response(
        code: StatusCode,
        msg: String,
        method: Method,
        url: String,
    ) -> Self {
        if code == StatusCode::REQUEST_TIMEOUT {
            Error::TimeoutError { msg, method, url }
        } else {
            Error::HttpError {
                method,
                url,
                code,
                msg,
            }
        }
    }
}
