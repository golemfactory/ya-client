use serde::{Deserialize, Serialize};

/// Generic for api call error.
#[derive(thiserror::Error, Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[error("Yagna API error: {message:?}")]
pub struct ErrorMessage {
    /// The error message.
    pub message: Option<String>,
}

impl ErrorMessage {
    /// Creates a new `ErrorMessage` with the specified message.
    pub fn new(message: impl ToString) -> ErrorMessage {
        ErrorMessage {
            message: Some(message.to_string()),
        }
    }
}

impl<T: Into<String>> From<T> for ErrorMessage {
    fn from(s: T) -> Self {
        Self::new(s.into())
    }
}
