//! Requestor control part of Activity API
use ya_client_model::activity::{
    CreateActivityRequest, CreateActivityResult, ExeScriptCommandResult, ExeScriptRequest,
    ACTIVITY_API_PATH,
};

use crate::{web::default_on_timeout, web::WebClient, web::WebInterface, Result};

/// Bindings for Requestor Control part of the Activity API.
#[derive(Clone)]
pub struct ActivityRequestorControlApi {
    client: WebClient,
}

impl WebInterface for ActivityRequestorControlApi {
    const API_URL_ENV_VAR: &'static str = crate::activity::ACTIVITY_URL_ENV_VAR;
    const API_SUFFIX: &'static str = ACTIVITY_API_PATH;

    fn from_client(client: WebClient) -> Self {
        ActivityRequestorControlApi { client }
    }
}

impl ActivityRequestorControlApi {
    /// Creates new Activity based on given Agreement.
    pub async fn create_activity(&self, agreement_id: &str) -> Result<String> {
        let r = CreateActivityRequest::new(agreement_id.to_owned());
        let result: CreateActivityResult =
            self.client.post("activity").send_json(&r).json().await?;
        Ok(result.activity_id)
    }

    #[cfg(feature = "sgx")]
    pub async fn create_secure_activity_raw(
        &self,
        agreement_id: &str,
        pub_key: secp256k1::PublicKey,
    ) -> Result<CreateActivityResult> {
        let mut r = CreateActivityRequest::new(agreement_id.to_owned());
        r.requestor_pub_key = Some(pub_key.to_string());
        self.client.post("activity").send_json(&r).json().await
    }

    #[cfg(feature = "sgx")]
    pub async fn create_secure_activity(
        &self,
        agreement_id: &str,
    ) -> Result<sgx::SecureActivityRequestorApi> {
        let s = secp256k1::Secp256k1::new();
        let (secret, pub_key) = s.generate_keypair(&mut rand::thread_rng());
        let result = self
            .create_secure_activity_raw(agreement_id, pub_key.clone())
            .await?;
        let api = sgx::SecureActivityRequestorApi::from_response(
            self.client.clone(),
            result.activity_id.clone(),
            result,
            secret,
        )
        .map_err(|e| crate::Error::InternalError(e.to_string()))?;
        Ok(api)
    }

    /// Destroys given Activity.
    pub async fn destroy_activity(&self, activity_id: &str) -> Result<()> {
        let uri = url_format!("activity/{activity_id}", activity_id);
        self.client.delete(&uri).send().json().await?;
        Ok(())
    }

    /// Executes an ExeScript batch within a given Activity.
    pub async fn exec(&self, script: ExeScriptRequest, activity_id: &str) -> Result<String> {
        let uri = url_format!("activity/{activity_id}/exec", activity_id);
        self.client.post(&uri).send_json(&script).json().await
    }

    /// Queries for ExeScript batch results.
    #[rustfmt::skip]
    pub async fn get_exec_batch_results(
        &self,
        activity_id: &str,
        batch_id: &str,
        #[allow(non_snake_case)]
        timeout: Option<f32>,
        command_index: Option<usize>,
    ) -> Result<Vec<ExeScriptCommandResult>> {
        let uri = url_format!(
            "activity/{activity_id}/exec/{batch_id}",
            activity_id,
            batch_id,
            #[query] timeout,
            #[query] command_index,
        );
        self.client.get(&uri).send().json().await.or_else(default_on_timeout)
    }
}

#[cfg(feature = "sgx")]
pub mod sgx {
    use super::*;
    use crate::model::activity::encrypted as enc;
    use crate::model::activity::{Credentials, ExeScriptCommand, SgxCredentials};
    use crate::Error as AppError;
    use secp256k1::{PublicKey, SecretKey};
    use std::sync::Arc;
    use ya_client_model::activity::encrypted::EncryptionCtx;
    use ya_client_model::activity::ExeScriptCommandState;

    #[derive(thiserror::Error, Debug)]
    pub enum SgxError {
        #[error("activity without keys")]
        MissingKeys,
        #[error("activity with unknown keys")]
        InvalidKeys,
    }

    struct Session {
        activity_id: String,
        #[allow(unused)]
        enclave_key: PublicKey,
        ctx: EncryptionCtx,
    }

    #[derive(Clone)]
    pub struct SecureActivityRequestorApi {
        client: WebClient,
        session: Arc<Session>,
    }

    fn gen_id() -> String {
        use rand::Rng;
        let v: u128 = rand::thread_rng().gen();
        format!("{:032x}", v)
    }

    impl SecureActivityRequestorApi {
        pub fn from_response(
            client: WebClient,
            activity_id: String,
            response: CreateActivityResult,
            requestor_key: SecretKey,
        ) -> std::result::Result<Self, SgxError> {
            let sgx: SgxCredentials = match response.credentials {
                Some(Credentials::Sgx(sgx)) => sgx,
                None => return Err(SgxError::MissingKeys),
                Some(_) => return Err(SgxError::InvalidKeys),
            };
            let enclave_key = sgx.enclave_pub_key;
            let ctx = EncryptionCtx::new(&enclave_key, &requestor_key);
            let session = Arc::new(Session {
                activity_id,
                enclave_key,
                ctx,
            });

            // TODO: Add attestation here!

            Ok(SecureActivityRequestorApi { client, session })
        }

        pub async fn exec(&self, exe_script: Vec<ExeScriptCommand>) -> Result<String> {
            let request = enc::Request {
                activity_id: self.session.activity_id.clone(),
                batch_id: gen_id(),
                timeout: None,
                command: enc::RequestCommand::Exec { exe_script },
            };
            let resp = match self.send(request).await? {
                enc::Response::Exec(r) => r,
                enc::Response::Error(e) => Err(e),
                _ => return Err(AppError::InternalError("invalid response".to_string())),
            };
            Ok(resp.map_err(|e| AppError::InternalError(e.to_string()))?)
        }

        pub async fn get_exec_batch_results(
            &self,
            batch_id: &str,
            timeout: Option<f32>,
            command_index: Option<usize>,
        ) -> Result<Vec<ExeScriptCommandResult>> {
            let request = enc::Request {
                activity_id: self.session.activity_id.clone(),
                batch_id: batch_id.to_string(),
                timeout,
                command: enc::RequestCommand::GetExecBatchResults { command_index },
            };
            let resp = match self.send(request).await? {
                enc::Response::GetExecBatchResults(r) => r,
                enc::Response::Error(e) => Err(e),
                _ => return Err(AppError::InternalError("invalid response".to_string())),
            };
            Ok(resp.map_err(|e| AppError::InternalError(e.to_string()))?)
        }

        pub async fn get_running_command(
            &self,
            timeout: Option<f32>,
        ) -> Result<ExeScriptCommandState> {
            let request = enc::Request {
                activity_id: self.session.activity_id.clone(),
                batch_id: String::new(),
                timeout,
                command: enc::RequestCommand::GetRunningCommand,
            };
            let resp = match self.send(request).await? {
                enc::Response::GetRunningCommand(r) => r,
                enc::Response::Error(e) => Err(e),
                _ => return Err(AppError::InternalError("invalid response".to_string())),
            };
            Ok(resp.map_err(|e| AppError::InternalError(e.to_string()))?)
        }

        async fn send(&self, request: enc::Request) -> Result<enc::Response> {
            let bytes = self
                .session
                .ctx
                .encrypt(&request)
                .map_err(|e| AppError::InternalError(e.to_string()))?;
            let uri = format!(
                "activity/{activity_id}/encrypted",
                activity_id = self.session.activity_id
            );
            let response = self
                .session
                .ctx
                .decrypt(&self.client.post(&uri).send_bytes(bytes).bytes().await?)
                .map_err(|e| AppError::InternalError(e.to_string()))?;
            Ok(response)
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    #[cfg(feature = "sgx")]
    fn test_encdec() {
        use crate::model::activity::encrypted::EncryptionCtx;
        use rand::Rng;

        let mut rng = rand::thread_rng();
        let s = secp256k1::Secp256k1::new();
        let (s1, p1) = s.generate_keypair(&mut rng);
        let (s2, p2) = s.generate_keypair(&mut rng);

        let ctx1 = EncryptionCtx::new(&p2, &s1);
        let ctx2 = EncryptionCtx::new(&p1, &s2);
        let data: [u8; 20] = rng.gen();
        let data2 = ctx2
            .decrypt_bytes(&ctx1.encrypt_bytes(&data).unwrap())
            .unwrap();
        assert_eq!(data2.as_slice(), data.as_ref())
    }
}
