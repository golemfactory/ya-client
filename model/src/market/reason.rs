/*
 * Yagna Market API
 *
 * The version of the OpenAPI document: 1.6.1
 *
 * Generated by: https://openapi-generator.tech
 */

use derive_more::Display;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

/// Generic Event reason information structure.
#[derive(Clone, Display, Debug, PartialEq, Serialize, Deserialize)]
#[display(fmt = "'{}'", message)]
pub struct Reason {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Reason {
    /// Generic Event reason information structure.
    pub fn new(message: String) -> Reason {
        Reason {
            message,
            extra: Default::default(),
        }
    }

    /// Calling functions with `impl ConvertReason` is hard, because it requires
    /// specifying full Option type, like this: `Option<Reason>::None` instead of jsut
    /// typing `None`. Use this function to simplify.
    pub fn none() -> Option<Reason> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonReason {
    #[serde(flatten)]
    pub json: Value,
}

pub trait ConvertReason: Sized {
    fn from_json_reason(value: JsonReason) -> Result<Self>;
    fn into_json_reason(self) -> Result<JsonReason>;
}

impl<R> ConvertReason for R
where
    R: Serialize + DeserializeOwned,
{
    fn from_json_reason(value: JsonReason) -> Result<Self> {
        serde_json::from_value(value.json)
    }

    fn into_json_reason(self) -> Result<JsonReason> {
        Ok(JsonReason {
            json: serde_json::to_value(self)?,
        })
    }
}

pub fn convert_reason(reason: Option<impl ConvertReason>) -> Result<Option<JsonReason>> {
    Ok(match reason {
        Some(r) => Some(r.into_json_reason()?),
        None => None,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_into() {
        let jr = JsonReason {
            json: serde_json::json!({"ala":"ma kota","message": "coś"}),
        };
        let reason = Reason::from_json_reason(jr.clone()).unwrap();
        assert_eq!(
            Reason {
                message: "coś".into(),
                extra: vec![("ala".to_string(), serde_json::json!("ma kota"))]
                    .into_iter()
                    .collect()
            },
            reason
        );

        let json_reason = reason.clone().into_json_reason().unwrap();
        assert_eq!(jr, json_reason);
    }
}
