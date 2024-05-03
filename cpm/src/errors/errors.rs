use std::process;
use spdlog::prelude::*;

pub enum RuntimeErrors {
    // OS related errors 1-9
    NotSupportedOS(Option<String>),
    WorkingDirSameAsExePath(String, String),
    // JSON file related errors 10-10
    JSONFileNotFound(Option<String>),
}

impl RuntimeErrors {
    pub fn error_code(&self) -> i32 {
        match *self {
            // OS related errors 1-9
            RuntimeErrors::NotSupportedOS(_) => 1,
            RuntimeErrors::WorkingDirSameAsExePath(_, _) => 2,
            // JSON file related errors 10-10
            RuntimeErrors::JSONFileNotFound(_) => 2,
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
        }
    }

    pub fn exit(&self) {
        error!("{}", self.error_message());
        process::exit(self.error_code());
    }
}
