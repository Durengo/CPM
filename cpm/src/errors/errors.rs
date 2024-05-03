use std::process;
use spdlog::prelude::*;

pub enum RuntimeErrors {
    // OS related errors 1-9
    NotSupportedOS(Option<String>),
    WorkingDirSameAsExePath(String, String),
    // JSON file related errors 10-10
    JSONFileNotFound(Option<String>),
    // Logic related errors 21-30
    NoInitFlagSet,
    NotInitialized,
    NoCommandsProvided,
    // Setup Command related errors 31-40
    GenerateProjectInvalidSystemType(Option<String>),
    GenerateProjectNtMsvcNoToolchain,
    ToolchainNotFound(String),
    // Not implemented 1000-1005
    NotImplemented,
}

impl RuntimeErrors {
    pub fn error_code(&self) -> i32 {
        match *self {
            // OS related errors 1-9
            RuntimeErrors::NotSupportedOS(_) => 1,
            RuntimeErrors::WorkingDirSameAsExePath(_, _) => 2,
            // JSON file related errors 10-20
            RuntimeErrors::JSONFileNotFound(_) => 2,
            // Logic related errors 21-30
            RuntimeErrors::NoInitFlagSet => 21,
            RuntimeErrors::NotInitialized => 22,
            RuntimeErrors::NoCommandsProvided => 23,
            // Setup Command related errors 31-40
            RuntimeErrors::GenerateProjectInvalidSystemType(_) => 31,
            RuntimeErrors::GenerateProjectNtMsvcNoToolchain => 32,
            RuntimeErrors::ToolchainNotFound(_) => 33,
            // Not implemented 1000-1005
            RuntimeErrors::NotImplemented => 1000,
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            // OS related errors 1-9
            RuntimeErrors::NotSupportedOS(Some(message)) => {
                format!("The OS is not supported: {}", message)
            }
            RuntimeErrors::NotSupportedOS(None) => "The OS is not supported".to_string(),
            RuntimeErrors::WorkingDirSameAsExePath(working_dir, exe_path) => {
                format!(
                    "The working directory is the same as the executable directory: {} == {}",
                    working_dir,
                    exe_path
                )
            }
            // JSON file related errors 10-10
            RuntimeErrors::JSONFileNotFound(Some(message)) => {
                format!("The JSON file was not found: {}", message)
            }
            RuntimeErrors::JSONFileNotFound(None) => "The JSON file was not found".to_string(),
            // Logic related errors 21-30
            RuntimeErrors::NoInitFlagSet =>
                "The no-init flag was set. Do not run 'init' from entrypoint".to_string(),
            RuntimeErrors::NotInitialized => "The settings are not initialized".to_string(),
            RuntimeErrors::NoCommandsProvided => "No commands were provided".to_string(),
            // Setup Command related errors 31-40
            RuntimeErrors::GenerateProjectInvalidSystemType(Some(system_type)) => {
                format!("The system type '{}' is invalid", system_type)
            }
            RuntimeErrors::GenerateProjectInvalidSystemType(None) => {
                "The system type is invalid".to_string()
            }
            RuntimeErrors::GenerateProjectNtMsvcNoToolchain => {
                "The system type 'nt/msvc' requires a toolchain path".to_string()
            }
            RuntimeErrors::ToolchainNotFound(toolchain) => {
                format!("Toolchain '{}' not found", toolchain)
            }
            // Not implemented 1000-1005
            RuntimeErrors::NotImplemented => "This feature is not implemented".to_string(),
        }
    }

    pub fn exit(&self) {
        error!("{}", self.error_message());
        process::exit(self.error_code());
    }
}
