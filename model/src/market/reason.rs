use derive_more::Display;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Generic Event reason information structure.
#[derive(Clone, Display, Debug, PartialEq, Serialize, Deserialize)]
#[display(fmt = "'{}'", message)]
pub struct Reason {
    pub message: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl Reason {
    /// Generic Event reason information structure.
    pub fn new(message: impl ToString) -> Reason {
        Reason {
            message: message.to_string(),
            extra: serde_json::json!({}),
        }
    }
}

#[derive(thiserror::Error, Clone, Debug, PartialEq)]
#[error("Error converting `{0}` to Reason: {1}")]
pub struct ReasonConversionError(String, String);

impl Reason {
    pub fn from_value<T: Serialize + Debug>(value: &T) -> Result<Self, ReasonConversionError> {
        serde_json::to_value(value)
            .and_then(serde_json::from_value)
            .map_err(|e| ReasonConversionError(format!("{:?}", value), e.to_string()))
    }

    pub fn to_value<T: DeserializeOwned>(&self) -> Result<T, ReasonConversionError> {
        serde_json::to_value(self)
            .and_then(serde_json::from_value)
            .map_err(|e| ReasonConversionError(format!("{:?}", self), e.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_convert_self() {
        let reason = Reason::new("coś");
        assert_eq!(reason, Reason::from_value(&reason).unwrap());
    }

    #[test]
    // check if message field in extra will not overwrite top-level message
    fn test_try_convert_self_with_extra_message() {
        let reason = Reason {
            message: "coś".to_string(),
            extra: serde_json::json!({"ala":"ma kota","message": "coś innego"}),
        };
        assert_eq!(
            Reason {
                message: "coś innego".to_string(),
                extra: serde_json::json!({"ala":"ma kota"}),
            },
            Reason::from_value(&reason).unwrap()
        );

        assert_ne!(
            reason,
            Reason::from_value(&reason).unwrap().to_value().unwrap()
        )
    }

    #[test]
    fn test_try_convert_custom_reason_wo_message_field() {
        #[derive(Serialize, Deserialize, Debug)]
        struct CustomReason {};
        let custom = CustomReason {};

        assert_eq!(
            "Error converting `CustomReason` to Reason: missing field `message`",
            &Reason::from_value(&custom).unwrap_err().to_string()
        )
    }

    #[test]
    fn test_try_convert_custom_reason_with_message_field() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct CustomReason {
            message: String,
            other: bool,
        };
        let custom = CustomReason {
            message: "coś".to_string(),
            other: false,
        };

        assert_eq!(
            Reason {
                message: "coś".to_string(),
                extra: serde_json::json!({ "other": false }),
            },
            Reason::from_value(&custom).unwrap()
        );

        assert_eq!(
            custom,
            Reason::from_value(&custom).unwrap().to_value().unwrap()
        )
    }

    #[test]
    fn test_try_convert_custom_reason_wrong_message_type() {
        #[derive(Serialize, Deserialize, Debug)]
        struct CustomReason {
            message: u8,
        };
        let custom = CustomReason { message: 37 };

        assert_eq!(
            "Error converting `CustomReason { message: 37 }` to Reason: invalid \
            type: integer `37`, expected a string",
            &Reason::from_value(&custom).unwrap_err().to_string()
        )
    }

    #[test]
    fn test_try_convert_custom_reason_wrong_fancy_message_type() {
        #[derive(Serialize, Deserialize, Debug)]
        struct CustomReason {
            i: u8,
            b: bool,
            s: String,
            #[serde(flatten)]
            extra: serde_json::Value,
        }

        let custom = CustomReason {
            i: 0,
            b: false,
            s: "".to_string(),
            extra: serde_json::json!({"message": true}),
        };

        assert_eq!(
            format!(
                "Error converting `{:?}` to Reason: invalid type: \
                boolean `true`, expected a string",
                custom
            ),
            Reason::from_value(&custom).unwrap_err().to_string()
        )
    }

    #[test]
    fn test_try_convert_custom_reason_with_fancy_message() {
        #[derive(Serialize, Deserialize, Debug)]
        struct CustomReason {
            i: u8,
            b: bool,
            s: String,
            #[serde(flatten)]
            extra: serde_json::Value,
        }

        let custom = CustomReason {
            i: 0,
            b: false,
            s: "".to_string(),
            extra: serde_json::json!({"message": "coś"}),
        };

        assert_eq!(
            Reason {
                message: "coś".to_string(),
                extra: serde_json::json!({ "i": 0, "b": false, "s": "" }),
            },
            Reason::from_value(&custom).unwrap()
        )
    }

    #[test]
    fn test_try_convert_json_reason_with_message() {
        let json_reason = serde_json::json!({"message": "coś"});

        assert_eq!(
            Reason::new("coś"),
            Reason::from_value(&json_reason).unwrap()
        )
    }
}
