//! The Activity API.

mod activity_state;
mod activity_usage;
mod create_activity;

#[cfg(feature = "sgx")]
#[doc(hidden)]
pub mod encrypted;
mod exe_script_command;
mod exe_script_command_result;
mod exe_script_command_state;
mod exe_script_request;
mod provider_event;
mod runtime_event;
#[cfg(feature = "sgx")]
#[doc(hidden)]
mod sgx_credentials;

pub use self::activity_state::{ActivityState, State, StatePair};
pub use self::activity_usage::ActivityUsage;
pub use self::create_activity::{CreateActivityRequest, CreateActivityResult, Credentials};
pub use self::exe_script_command::{Capture, CaptureFormat, CaptureMode, CapturePart, Network};
pub use self::exe_script_command::{ExeScriptCommand, FileSet, SetEntry, SetObject, TransferArgs};
pub use self::exe_script_command_result::{CommandOutput, CommandResult, ExeScriptCommandResult};
pub use self::exe_script_command_state::ExeScriptCommandState;
pub use self::exe_script_request::ExeScriptRequest;
pub use self::provider_event::{ProviderEvent, ProviderEventType};
pub use self::runtime_event::{RuntimeEvent, RuntimeEventKind};
#[cfg(feature = "sgx")]
pub use self::sgx_credentials::SgxCredentials;

#[doc(hidden)]
pub const ACTIVITY_API_PATH: &str = "/activity-api/v1";
