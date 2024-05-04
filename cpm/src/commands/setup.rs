use rust_embed::RustEmbed;
use spdlog::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{ Serialize, Deserialize };

use crate::commands::SetupArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::{ self, Settings };
use crate::internal::cmd;
use crate::internal::install::{
    WindowsConfig,
    LinuxConfig,
    MacOSConfig,
    Presets,
    Config,
    MultiOSConfig,
    Package,
};

pub fn run(args: SetupArgs) {
    debug!("Running the Initialization command with arguments: {:#?}", args);

    // Grab the settings file as it will be needed for the subcommands.
    let settings_path = match Settings::get_settings_path() {
        Ok(path) => path,
        Err(e) => {
            error!("Failed to get settings path: {}", e);
            return;
        }
    };

    let mut settings = match Settings::load(&settings_path) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to load settings: {}", e);
            return;
        }
    };

    // If not initialized, throw an error
    if !settings.initialized {
        error!("Project not initialized. Run 'init' command first.");
        RuntimeErrors::ProjectNotInitialized.exit();
    }

    let mut Config = match retrieve_install(settings.install_json_path.as_ref()) {
        Ok(config) => config,
        Err(_) => {
            RuntimeErrors::JSONFileNotFound(Some(settings.install_json_path.clone())).exit();
            return;
        }
    };

    let mut selected_os = String::new();

    // If platform flag is set, only run the command for the specified platform, otherwise run the current platform
    if let Some(platform) = &args.platform {
        match platform.as_str() {
            "windows" => {
                selected_os = platform.to_string();
            }
            "linux" => RuntimeErrors::NotSupportedOS(Some(platform.to_string())).exit(),
            "macos" => RuntimeErrors::NotSupportedOS(Some(platform.to_string())).exit(),
            _ => {
                RuntimeErrors::NotSupportedOS(Some(platform.to_string())).exit();
                return;
            }
        }
    } else {
        match settings.os.as_str() {
            "windows" => {
                selected_os = settings.os.to_string();
            }
            "linux" => RuntimeErrors::NotSupportedOS(Some(settings.os.to_string())).exit(),
            "macos" => RuntimeErrors::NotSupportedOS(Some(settings.os.to_string())).exit(),
            _ => {
                RuntimeErrors::NotSupportedOS(Some(settings.os.to_string())).exit();
                return;
            }
        }
    }

    if let Some(toolchain_path) = &args.toolchain {
        debug!("Path before trim: {}", toolchain_path);
        // If the provided path has any '/' or '\' characters at the very end, remove them
        let toolchain_path = toolchain_path.trim_end_matches(|c| (c == '/' || c == '\\'));
        debug!("Path after trim: {}", toolchain_path);
        settings.toolchain_path = toolchain_path.to_string();
        check_toolchain(&mut settings);
        return;
    }
    // Auto detect toolchain and run setup.
    if args.auto_toolchain_path {
        auto_toolchain_path(&mut settings, &Config);
    }
    // Auto detect toolchain and run setup otherwise manually set up toolchain.
    if args.no_toolchain_path {
    }
    // Use provided path and try to run setup.
    if let Some(toolchain_path) = &args.use_toolchain_path {
    }

    // debug!("Config:\n{:#?}", Config);
    // debug!("OS: {}", selected_os);
}

fn retrieve_install(file_path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    // Read the contents into a string
    let mut config_data = String::new();
    file.read_to_string(&mut config_data).map_err(|e| format!("Failed to read file data: {}", e))?;

    // Deserialize the JSON string into Config struct
    let config: Config = serde_json
        ::from_str(&config_data)
        .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

    // Return the deserialized config
    Ok(config)
}

fn check_toolchain(settings: &mut Settings) {
    // Run through a match of know toolchains and find their appropriate .cmake file.
    // Current list of know toolchains:
    // - VCPKG
    if !settings.toolchain_path.is_empty() {
        // Normalize the path to use consistent path separators
        let normalized_path = normalize_path_separator(&settings.toolchain_path);

        let toolchain_path = Path::new(&normalized_path);
        let toolchain_root = toolchain_path
            .file_name()
            .map(|name| name.to_string_lossy())
            .unwrap_or_default();

        // debug!("Toolchain root: {}", toolchain_root);

        match toolchain_root.as_ref() {
            "vcpkg" => {
                let vcpkg_cmake_path =
                    // Use backslashes for consistency in Windows
                    format!("{}\\scripts\\buildsystems\\vcpkg.cmake", normalized_path);
                if Path::new(&vcpkg_cmake_path).exists() {
                    info!("Detected VCPKG CMake toolchain file: {}", vcpkg_cmake_path);
                    settings.vcpkg_path = vcpkg_cmake_path;
                    settings.save_default();
                } else {
                    error!("VCPKG CMake toolchain file not found at: {}", vcpkg_cmake_path);
                    RuntimeErrors::ToolchainNotFound("VCPKG".to_string()).exit();
                }
            }
            _ => {
                RuntimeErrors::ToolchainNotFound(toolchain_root.to_string()).exit();
            }
        }
    }
}

fn normalize_path_separator(path: &str) -> String {
    if cfg!(target_os = "windows") {
        // Convert all forward slashes to backslashes on Windows
        path.replace("/", "\\")
    } else {
        // On non-Windows systems, just return the original
        path.to_string()
    }
}

fn auto_toolchain_path(settings: &mut Settings, config: &Config) {
    info!("Auto detecting toolchain path");
    // First check if the toolchain path is already set
    // if !settings.toolchain_path.is_empty() {
    //     debug!("Toolchain path already set: {}", settings.toolchain_path);
    //     return;
    // }

    // If not set we need to find the toolchain path. This is OS specific.
    // Run commands to find the toolchain path.
    // On Windows we can use 'where.exe' to find the path of a given executable.
    match settings.os.as_str() {
        "windows" => {
            // We need to look at config file to see what toolchain to look for.
            // Retrieve this from Config.windows.toolchain
            let toolchain: &str;
            if let Some(windows_config) = &config.config.windows {
                // Use windows_config by reference
                let toolchain = &windows_config.toolchain;
                debug!("Using Windows toolchain: {}", toolchain);

                let toolchain_path = cmd::execute_and_return_output(
                    vec!["where".to_string(), toolchain.to_string()]
                );
                if !toolchain_path.is_empty() {
                    debug!("{} path found: {}", toolchain.to_ascii_uppercase(), toolchain_path);
                    // Remove the trailing newline character and set the toolchain path to the root folder (not .exe file)
                    // Windows has \r\n line endings, so we need to trim both \r and \n
                    let toolchain_path = toolchain_path.trim_end_matches(
                        |c| (c == '\r' || c == '\n')
                    );
                    // Normalize the path to use consistent path separators
                    let normalized_path = normalize_path_separator(&toolchain_path);
                    // Remove '\\vcpkg.exe' from the path
                    let normalized_path = normalized_path.trim_end_matches("\\vcpkg.exe");
                    debug!("Normalized VCPKG path: {}", normalized_path);

                    settings.toolchain_path = normalized_path.to_string();
                    let _ = settings.save_default();
                    // Now we have the path to the toolchain but still need to find the .cmake file. We already have a function for this.
                    check_toolchain(settings);
                } else {
                    error!("VCPKG not found in PATH");
                    RuntimeErrors::ToolchainNotFound("VCPKG".to_string()).exit();
                }
            } else {
                error!("No Windows configuration found in the install config");
            }
        }
        _ => {
            RuntimeErrors::NotSupportedOS(Some(settings.os.to_string())).exit();
        }
    }
}

fn windows_install(settings: &Settings, config: &Config) {
    // Retrieve the WindowsConfig from the Config
    let windows_config = match &config.config.windows {
        Some(windows) => windows,
        None => {
            error!("No Windows configuration found in the install config");
            return;
        }
    };

    debug!("Windows Config:\n{:#?}", windows_config);
}
