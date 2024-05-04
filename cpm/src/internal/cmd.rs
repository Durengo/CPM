use std::process::Command;
use spdlog::prelude::*;

use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;

fn init(cmd_array: Vec<String>) -> String {
    let mut settings = Settings::init(false).unwrap();

    settings.last_command = cmd_array;
    let _ = settings.save_default();

    check_supported_os(&settings)
}

pub fn execute_and_display_output(cmd_array: Vec<String>) {
    if cmd_array.is_empty() {
        RuntimeErrors::NoCommandsProvided.exit();
    }

    trace!("Executing command: {}", cmd_array.join(" "));

    match init(cmd_array.clone()).as_str() {
        "windows" => {
            let (command, args) = cmd_array.split_first().unwrap();

            let output = Command::new("cmd")
                .args(&["/C", command])
                .args(args)
                .output()
                .expect("Failed to execute command");

            // Remove all trailing newline characters
            if !output.stdout.is_empty() {
                let out = String::from_utf8_lossy(&output.stdout)
                    .trim_end_matches(|c| (c == '\r' || c == '\n'))
                    .to_string();
                trace!("STDOUT:\n{}", out);
            }
            if !output.stderr.is_empty() {
                let err = String::from_utf8_lossy(&output.stderr)
                    .trim_end_matches(|c| (c == '\r' || c == '\n'))
                    .to_string();
                error!("STDERR:\n{}", err);
                RuntimeErrors::CmdCaughtStdErr(Some(err));
            }
        }
        _ => {
            RuntimeErrors::NotSupportedOS(None).exit();
        }
    }
}

pub fn execute(cmd_array: Vec<String>) {
    if cmd_array.is_empty() {
        RuntimeErrors::NoCommandsProvided.exit();
    }

    trace!("Executing command: {}", cmd_array.join(" "));

    match init(cmd_array.clone()).as_str() {
        "windows" => {
            let (command, args) = cmd_array.split_first().unwrap();

            let output = Command::new("cmd")
                .args(&["/C", command])
                .args(args)
                .output()
                .expect("Failed to execute command");

            // Remove all trailing newline characters
            if !output.stdout.is_empty() {
                let out = String::from_utf8_lossy(&output.stdout)
                    .trim_end_matches(|c| (c == '\r' || c == '\n'))
                    .to_string();
                debug!("STDOUT:\n{}", out);
            }
            if !output.stderr.is_empty() {
                let err = String::from_utf8_lossy(&output.stderr)
                    .trim_end_matches(|c| (c == '\r' || c == '\n'))
                    .to_string();
                error!("STDERR:\n{}", err);
                RuntimeErrors::CmdCaughtStdErr(Some(err));
            }
        }
        _ => {
            RuntimeErrors::NotSupportedOS(None).exit();
        }
    }
}

pub fn execute_and_return_output(cmd_array: Vec<String>) -> String {
    if cmd_array.is_empty() {
        RuntimeErrors::NoCommandsProvided.exit();
    }

    trace!("Executing command: {}", cmd_array.join(" "));

    match init(cmd_array.clone()).as_str() {
        "windows" => {
            let (command, args) = cmd_array.split_first().unwrap();

            let output = Command::new("cmd")
                .args(&["/C", command])
                .args(args)
                .output()
                .expect("Failed to execute command");

            // Remove all trailing newline characters
            if !output.stdout.is_empty() {
                let out = String::from_utf8_lossy(&output.stdout)
                    .trim_end_matches(|c| (c == '\r' || c == '\n'))
                    .to_string();
                debug!("STDOUT:\n{}", out);
            }
            if !output.stderr.is_empty() {
                let err = String::from_utf8_lossy(&output.stderr)
                    .trim_end_matches(|c| (c == '\r' || c == '\n'))
                    .to_string();
                error!("STDERR:\n{}", err);
                RuntimeErrors::CmdCaughtStdErr(Some(err));
            }

            // Before we return the string we must make sure to remove the trailing newline character (\r\n)
            String::from_utf8_lossy(&output.stdout)
                .trim_end_matches(|c| (c == '\r' || c == '\n'))
                .to_string()
        }
        _ => {
            RuntimeErrors::NotSupportedOS(None).exit();
            String::new()
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
