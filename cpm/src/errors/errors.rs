use std::process;
use spdlog::prelude::*;

pub enum RuntimeErrors {
    // OS related errors 1-1
    NotSupportedOS(Option<String>),
    // JSON file related errors 2-2
    JSONFileNotFound(Option<String>),
}

impl RuntimeErrors {
    pub fn error_code(&self) -> i32 {
        match *self {
            // OS related errors 1-1
            RuntimeErrors::NotSupportedOS(_) => 1,
            // JSON file related errors 2-2
            RuntimeErrors::JSONFileNotFound(_) => 2,
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            // OS related errors 1-1
            RuntimeErrors::NotSupportedOS(Some(message)) => {
                format!("The OS is not supported: {}", message)
            }
            RuntimeErrors::NotSupportedOS(None) => "The OS is not supported".to_string(),
            // JSON file related errors 2-2
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
