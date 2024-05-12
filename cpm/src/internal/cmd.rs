use spdlog::prelude::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Output, Stdio};
use std::thread;

use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;

fn init(cmd_array: Vec<String>) -> String {
    let mut settings = Settings::init(false).unwrap();

    settings.last_command = cmd_array;
    let _ = settings.save_default();

    let os = check_supported_os(&settings);
    debug!("CMD CHECK: OS: {}", os);

    os
}

pub fn execute_and_display_output_live(cmd_array: Vec<String>) {
    if cmd_array.is_empty() {
        RuntimeErrors::NoCommandsProvided.exit();
    }

    trace!("Executing command: {}", cmd_array.join(" "));

    match init(cmd_array.clone()).as_str() {
        "windows" => {
            let (command, args) = cmd_array.split_first().unwrap();
            let mut child = Command::new(command)
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start command");

            let stdout = child.stdout.take().expect("Failed to take stdout of child");
            let stderr = child.stderr.take().expect("Failed to take stderr of child");

            let stdout_reader = BufReader::new(stdout);
            let stderr_reader = BufReader::new(stderr);

            let stdout_handle = thread::spawn(move || {
                for line in stdout_reader.lines() {
                    match line {
                        Ok(line) => println!("{}", line),
                        Err(e) => error!("Error reading stdout: {}", e),
                    }
                }
            });

            let stderr_handle = thread::spawn(move || {
                for line in stderr_reader.lines() {
                    match line {
                        Ok(line) => println!("{}", line),
                        Err(e) => error!("Error reading stderr: {}", e),
                    }
                }
            });

            stdout_handle
                .join()
                .expect("The stdout thread has panicked");
            stderr_handle
                .join()
                .expect("The stderr thread has panicked");
        }
        _ => {
            RuntimeErrors::NotSupportedOS(None).exit();
        }
    }
}

#[allow(dead_code)]
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
                info!("STDOUT:\n{}", out);
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
        // TODO: Error handling needs to be improved on linux as it's hard to detect what failed through STDERR
        "linux" => {
            let (command, args) = cmd_array.split_first().unwrap();

            let output = Command::new(command).args(args).output();

            match output {
                Ok(output) => process_output(output), // Process the output normally
                Err(e) => {
                    error!("Failed to execute command: {}", e);
                    format!("Command execution failed: {}", e)
                }
            }

            // let (command, args) = cmd_array.split_first().unwrap();

            // let current_path = std::env::var("PATH").unwrap_or_else(|_| "No PATH var set".to_string());
            // let current_dir = std::env::current_dir().unwrap_or_else(|_| "No current directory".into());
            // debug!("Current PATH: {}", current_path);
            // debug!("Current directory: {:?}", current_dir);

            // let output = Command::new(command)
            //     .args(args)
            //     .output();

            // match output {
            //     Ok(output) => process_output(output),
            //     Err(e) => {
            //         warn!("Failed to execute command: {}", e);
            //         warn!("Is the package manager in the PATH?");
            //         format!("Command execution failed: {}", e)
            //     }
            // }

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
        "linux" => env.to_string(),
        "macos" => {
            RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit();
            String::new()
        }
        "windows" => env.to_string(),
        _ => {
            RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit();
            String::new()
        }
    }
}

// experimental
fn process_output(output: Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout)
        .trim_end_matches(|c| c == '\r' || c == '\n')
        .to_string();

    if !output.stdout.is_empty() {
        debug!("STDOUT:\n{}", stdout);
    }

    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr)
            .trim_end_matches(|c| c == '\r' || c == '\n')
            .to_string();
        error!("STDERR:\n{}", stderr);
    }

    if output.status.success() {
        stdout
    } else {
        // You could choose to still return stdout if there is any, or a custom error message
        format!("Command failed with error. Check logs for more information.")
    }
}

// fn process_output(output: Output) -> String {
//     let stdout = String::from_utf8_lossy(&output.stdout).to_string();
//     let stderr = String::from_utf8_lossy(&output.stderr).to_string();

//     debug!("STDOUT:\n{}", stdout);
//     if !stderr.is_empty() {
//         error!("STDERR:\n{}", stderr);
//     }

//     if output.status.success() {
//         stdout
//     } else {
//         format!("Command failed with error: {}", stderr)
//     }
// }