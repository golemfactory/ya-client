pub mod activity_state;
pub mod activity_usage;
mod create_activity;
#[cfg(feature = "sgx")]
pub mod encrypted;
pub mod exe_script_command;
pub mod exe_script_command_result;
pub mod exe_script_command_state;
pub mod exe_script_request;
pub mod provider_event;
#[cfg(feature = "sgx")]
mod sgx_credentials;

pub use self::activity_state::{ActivityState, State, StatePair};
pub use self::activity_usage::ActivityUsage;
pub use self::create_activity::{CreateActivityRequest, CreateActivityResult, Credentials};
pub use self::exe_script_command::ExeScriptCommand;
pub use self::exe_script_command_result::{ExeScriptCommandResult, Result as CommandResult};
pub use self::exe_script_command_state::ExeScriptCommandState;
pub use self::exe_script_request::ExeScriptRequest;
pub use self::provider_event::ProviderEvent;
#[cfg(feature = "sgx")]
pub use self::sgx_credentials::SgxCredentials;

pub const ACTIVITY_API_PATH: &str = "/activity-api/v1/";
