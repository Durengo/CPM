use std::process;
use spdlog::prelude::*;

pub enum RuntimeErrors {
    // OS related errors 1-1
    NotSupportedOS(Option<String>),
}

impl RuntimeErrors {
    pub fn error_code(&self) -> i32 {
        match *self {
            // OS related errors 1-1
            RuntimeErrors::NotSupportedOS(_) => 1,
        }
    }

    pub fn error_message(&self) -> String {
        match self {
            RuntimeErrors::NotSupportedOS(Some(message)) => {
                format!("The OS is not supported: {}", message)
            }
            RuntimeErrors::NotSupportedOS(None) => "The OS is not supported".to_string(),
        }
    }

    pub fn exit(&self) {
        error!("{}", self.error_message());
        process::exit(self.error_code());
    }
}
