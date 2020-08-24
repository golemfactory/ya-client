use secp256k1::PublicKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SgxCredentials {
    #[serde(rename = "enclavePubKey")]
    pub enclave_pub_key: PublicKey,
    #[serde(rename = "requestorPubKey")]
    pub requestor_pub_key: PublicKey,
    #[serde(rename = "payloadHash")]
    pub payload_hash: String,
    #[serde(rename = "enclaveHash")]
    pub enclave_hash: String,
    #[serde(rename = "iasReport")]
    pub ias_report: String,
    #[serde(rename = "iasSig", with = "binenc")]
    pub ias_sig: Vec<u8>,
    #[serde(rename = "sessionKey", with = "binenc")]
    pub session_key: Vec<u8>,
}

impl SgxCredentials {
    pub fn new(
        enclave_pub_key: PublicKey,
        requestor_pub_key: PublicKey,
        payload_hash: String,
        enclave_hash: String,
        ias_report: String,
        ias_sig: Vec<u8>,
        session_key: Vec<u8>,
    ) -> SgxCredentials {
        SgxCredentials {
            enclave_pub_key,
            requestor_pub_key,
            payload_hash,
            enclave_hash,
            ias_report,
            ias_sig,
            session_key,
        }
    }

    pub fn try_with(
        enclave_pub_key: Vec<u8>,
        requestor_pub_key: Vec<u8>,
        payload_hash: String,
        enclave_hash: String,
        ias_report: String,
        ias_sig: Vec<u8>,
        session_key: Vec<u8>,
    ) -> Result<Self, secp256k1::Error> {
        Ok(Self::new(
            PublicKey::from_slice(enclave_pub_key.as_slice())?,
            PublicKey::from_slice(requestor_pub_key.as_slice())?,
            payload_hash,
            enclave_hash,
            ias_report,
            ias_sig,
            session_key,
        ))
    }
}

mod binenc {
    use super::*;
    use serde::de::Error;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = hex::encode(&bytes);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(D::Error::custom)?;
        Ok(bytes)
    }
}
