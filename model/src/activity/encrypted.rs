use crate::activity::{ExeScriptCommand, ExeScriptCommandResult};
use rand::Rng as _;
use secp256k1::ecdh::SharedSecret;
use secp256k1::{PublicKey, SecretKey};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub activity_id: String,
    pub batch_id: String,
    pub timeout: Option<f32>,
    pub command: RequestCommand,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestCommand {
    Exec { exe_script: Vec<ExeScriptCommand> },
    GetExecBatchResults { command_index: Option<usize> },
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "command")]
pub enum Response {
    Exec(Result<String, RpcMessageError>),
    GetExecBatchResults(Result<Vec<ExeScriptCommandResult>, RpcMessageError>),
    Error(RpcMessageError),
}

/// Error message for activity service bus API.
#[derive(thiserror::Error, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcMessageError {
    #[error("Service error: {0}")]
    Service(String),
    #[error("Market API error: {0}")]
    Activity(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Usage limit exceeded: {0}")]
    UsageLimitExceeded(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Timeout")]
    Timeout,
}

pub struct EncryptionCtx {
    aeskey: [u8; 32],
}

#[derive(thiserror::Error, Debug)]
pub enum EncryptionError {
    #[error("Encrypt, {0}")]
    Encrypt(String),
    #[error("Decrypt, {0}")]
    Decrypt(String),
    #[error("invalid encyrypted message format")]
    InvalidFormat,
    #[error("sede: {0}")]
    Serde(#[from] serde_json::Error),
}

impl EncryptionError {
    pub fn encrypt_err_msg(e: impl ToString) -> Self {
        EncryptionError::Encrypt(e.to_string())
    }
    pub fn decrypt_err_msg(e: impl ToString) -> Self {
        EncryptionError::Decrypt(e.to_string())
    }
}

impl EncryptionCtx {
    pub fn new(point: &PublicKey, scalar: &SecretKey) -> Self {
        let mut aeskey = [0u8; 32];
        aeskey.copy_from_slice(SharedSecret::new(point, scalar).as_ref());
        EncryptionCtx { aeskey }
    }

    pub fn encrypt_bytes(&self, buf: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        let mut bytes = Vec::with_capacity(12 + 16 + buf.len() + 5);
        let iv: [u8; 12] = rand::thread_rng().gen();
        let mut tag = [0u8; 16];
        let out = openssl::symm::encrypt_aead(
            openssl::symm::Cipher::aes_256_gcm(),
            self.aeskey.as_ref(),
            Some(iv.as_ref()),
            &[],
            buf.as_ref(),
            tag.as_mut(),
        )
        .map_err(EncryptionError::encrypt_err_msg)?;
        bytes.push(iv.len() as u8);
        bytes.extend_from_slice(iv.as_ref());
        bytes.push(tag.len() as u8);
        bytes.extend_from_slice(tag.as_ref());
        bytes.extend_from_slice(&out);
        Ok(bytes)
    }

    pub fn encrypt<T: Serialize>(&self, data: &T) -> Result<Vec<u8>, EncryptionError> {
        let bytes = serde_json::to_vec(data)?;
        self.encrypt_bytes(&bytes)
    }

    pub fn decrypt_bytes(&self, buf: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        let cipher = openssl::symm::Cipher::aes_256_gcm();
        let iv_size = cipher.iv_len().unwrap();
        let iv_size_pos = 0;
        let tag_size = 16;
        let tag_size_pos = iv_size + 1;
        let data_pos = tag_size_pos + tag_size + 1;

        if buf.len() < data_pos {
            return Err(EncryptionError::InvalidFormat);
        }

        if buf[iv_size_pos] as usize != iv_size {
            return Err(EncryptionError::InvalidFormat);
        }
        let iv = &buf[1..tag_size_pos];
        if buf[tag_size_pos] as usize != tag_size {
            return Err(EncryptionError::InvalidFormat);
        }
        let tag = &buf[(tag_size_pos + 1)..data_pos];
        let data = &buf[data_pos..];
        let output =
            openssl::symm::decrypt_aead(cipher, self.aeskey.as_ref(), Some(iv), &[], data, tag)
                .map_err(EncryptionError::decrypt_err_msg)?;

        Ok(output)
    }

    pub fn decrypt<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, EncryptionError> {
        let bytes = self.decrypt_bytes(bytes)?;
        Ok(serde_json::from_slice(bytes.as_slice())?)
    }
}
