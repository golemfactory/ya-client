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
pub mod runtime_event;
#[cfg(feature = "sgx")]
mod sgx_credentials;

pub use self::activity_state::{ActivityState, State, StatePair};
pub use self::activity_usage::ActivityUsage;
pub use self::create_activity::{CreateActivityRequest, CreateActivityResult, Credentials};
pub use self::exe_script_command::{Capture, CaptureFormat, CaptureMode, CapturePart};
pub use self::exe_script_command::{ExeScriptCommand, FileSet, SetEntry, SetObject, TransferArgs};
pub use self::exe_script_command_result::{CommandOutput, CommandResult, ExeScriptCommandResult};
pub use self::exe_script_command_state::ExeScriptCommandState;
pub use self::exe_script_request::ExeScriptRequest;
pub use self::provider_event::ProviderEvent;
pub use self::runtime_event::{RuntimeEvent, RuntimeEventKind};
#[cfg(feature = "sgx")]
pub use self::sgx_credentials::SgxCredentials;

pub const ACTIVITY_API_PATH: &str = "/activity-api/v1/";
