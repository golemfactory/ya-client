use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Generic Event reason information structure.
#[derive(Clone, Display, Debug, PartialEq, Serialize, Deserialize)]
#[display(fmt = "'{}'", message)]
pub struct Reason {
    pub message: String,
    #[serde(default)]
    pub extra: serde_json::Value,
}

impl Reason {
    /// Generic Event reason information structure.
    pub fn new(message: impl ToString) -> Reason {
        Reason {
            message: message.to_string(),
            extra: serde_json::Value::Null,
        }
    }
}

#[derive(thiserror::Error, Clone, Debug, PartialEq)]
#[error("Error converting `{0}` to Reason: {1}")]
pub struct ReasonConversionError(String, String);

impl Reason {
    pub fn try_convert<T: Serialize + Debug>(value: &T) -> Result<Self, ReasonConversionError> {
        serde_json::to_value(value)
            .and_then(serde_json::from_value)
            .map_err(|e| ReasonConversionError(format!("{:?}", value), e.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_try_convert_self() {
        let reason = Reason::new("coś");
        assert_eq!(reason, Reason::try_convert(&reason).unwrap());
    }

    #[test]
    // check if message field in extra will not overwrite top-level message
    fn test_try_convert_self_with_extra_message() {
        let reason = Reason {
            message: "coś".to_string(),
            extra: serde_json::json!({"ala":"ma kota","message": "coś innego"}),
        };
        assert_eq!(reason, Reason::try_convert(&reason).unwrap());
    }

    #[test]
    fn test_try_convert_custom_reason_wo_message_field() {
        #[derive(Serialize, Deserialize, Debug)]
        struct CustomReason {};
        let custom = CustomReason {};

        assert_eq!(
            "Error converting `CustomReason` to Reason: missing field `message`",
            &Reason::try_convert(&custom).unwrap_err().to_string()
        )
    }

    #[test]
    fn test_try_convert_custom_reason_with_message_field() {
        #[derive(Serialize, Deserialize, Debug)]
        struct CustomReason {
            message: String,
            other: bool,
        };
        let custom = CustomReason {
            message: "coś".to_string(),
            other: false,
        };

        assert_eq!(Reason::new("coś"), Reason::try_convert(&custom).unwrap())
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
            &Reason::try_convert(&custom).unwrap_err().to_string()
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
            "Error converting `CustomReason { i: 0, b: false, s: \"\", extra: \
            Object({\"message\": Bool(true)}) }` to Reason: invalid type: \
            boolean `true`, expected a string",
            &Reason::try_convert(&custom).unwrap_err().to_string()
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

        assert_eq!(Reason::new("coś"), Reason::try_convert(&custom).unwrap())
    }

    #[test]
    fn test_try_convert_json_reason_with_message() {
        let json_reason = serde_json::json!({"message": "coś"});

        assert_eq!(
            Reason::new("coś"),
            Reason::try_convert(&json_reason).unwrap()
        )
    }
}
