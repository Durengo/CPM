use std::process::Command;
use spdlog::prelude::*;
use shellwords::split;

use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;

fn init() -> String {
    let mut settings = Settings::init(false).unwrap();

    check_supported_os(&settings)
}

pub fn execute(cmd_array: Vec<String>) {
    if cmd_array.is_empty() {
        RuntimeErrors::NoCommandsProvided.exit();
    }

    info!("Executing command: {}", cmd_array.join(" "));

    match init().as_str() {
        "windows" => {
            let (command, args) = cmd_array.split_first().unwrap();

            let output = Command::new("cmd")
                .args(&["/C", command])
                .args(args)
                .output()
                .expect("Failed to execute command");

            if !output.stdout.is_empty() {
                info!("\n{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                error!("\n{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        _ => {
            RuntimeErrors::NotSupportedOS(None).exit();
        }
    }
}

fn check_supported_os(settings: &Settings) -> String {
    let env = &settings.os;

    match env.as_str() {
        "linux" => {
            RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit();
            String::new()
        }
        "macos" => {
            RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit();
            String::new()
        }
        "windows" => { env.to_string() }
        _ => {
            RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit();
            String::new()
        }
    }
}
