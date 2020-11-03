use chrono::Duration;
use graphene_sgx::sgx::{self, SgxMeasurement};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

const CONFIG: &str = include_str!("sgx_config.json");

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SgxConfigJson {
    pub enable_attestation: bool,
    pub exeunit_hashes: Vec<String>,
    pub allow_debug: bool,
    pub allow_outdated_tcb: bool,
    pub max_evidence_age: i64, // in seconds
}

#[derive(Clone, Debug)]
pub struct SgxConfig {
    pub enable_attestation: bool,
    pub exeunit_hashes: Vec<SgxMeasurement>,
    pub allow_debug: bool,
    pub allow_outdated_tcb: bool,
    pub max_evidence_age: Duration,
}

lazy_static! {
    pub static ref SGX_CONFIG: SgxConfig = {
        let json_cfg = match std::env::var("YAGNA_SGX_CONFIG") {
            Ok(cfg) => String::from_utf8(std::fs::read(cfg).unwrap()).unwrap(),
            Err(_) => CONFIG.to_owned(),
        };
        let cfg: SgxConfigJson = serde_json::from_str(&json_cfg).unwrap();
        log::debug!("SGX config: {:?}", &cfg);
        SgxConfig {
            enable_attestation: cfg.enable_attestation,
            exeunit_hashes: cfg
                .exeunit_hashes
                .into_iter()
                .map(|hex| sgx::parse_measurement(&hex).unwrap())
                .collect(),
            allow_debug: cfg.allow_debug,
            allow_outdated_tcb: cfg.allow_outdated_tcb,
            max_evidence_age: Duration::seconds(cfg.max_evidence_age),
        }
    };
}
