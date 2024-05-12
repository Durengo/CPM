use spdlog::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::commands::InitArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::install::Presets;
use crate::internal::settings::Settings;

#[cfg(target_os = "windows")]
const BUILD_DIR_NAME: &str = "Build";
#[cfg(target_os = "linux")]
const BUILD_DIR_NAME: &str = "build";
#[cfg(target_os = "macos")]
const BUILD_DIR_NAME: &str = "build";


#[cfg(target_os = "windows")]
const INSTALL_DIR_NAME: &str = "Install";
#[cfg(target_os = "linux")]
const INSTALL_DIR_NAME: &str = "install";
#[cfg(target_os = "macos")]
const INSTALL_DIR_NAME: &str = "install";

pub fn run(args: InitArgs, _no_init: bool) {
    debug!(
        "Running the Initialization command with arguments: {:#?}",
        args
    );
    debug!("No init flag: {}", _no_init);

    if _no_init {
        RuntimeErrors::NoInitFlagSet.exit();
    }

    _ = entry();
}

fn entry() -> std::io::Result<()> {
    let mut settings = Settings::load(&Settings::get_settings_path()?)?;

    debug!("Before:\n{:#?}", settings);

    settings.working_dir = std::env::current_dir()?.to_str().unwrap().to_string();
    // If working directory is the same as the executable directory, throw an error
    if settings.working_dir == settings.exe_dir {
        RuntimeErrors::WorkingDirSameAsExePath(
            settings.working_dir.clone(),
            settings.exe_dir.clone(),
        )
        .exit();
    }

    settings.save(&Settings::get_settings_path()?)?;

    info!("Working directory set: {:#?}", settings.working_dir);

    os_specific(&mut settings);

    settings.initialized = true;
    settings.save(&Settings::get_settings_path()?)?;

    debug!("After:\n{:#?}", settings);

    Ok(())
}

fn os_specific(settings: &mut Settings) {
    let env = &settings.os;

    match env.as_str() {
        "linux" => linux(settings),
        "macos" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "windows" => windows(settings),
        _ => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
    }
}

// LINUX

fn linux(settings: &mut Settings) {
    get_and_load_preset_config(settings);
    create_entrypoint();
    set_build_dir(settings);
    set_install_dir(settings);
}

#[cfg(target_os = "linux")]
fn chmod_file(file_path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let permissions = std::fs::Permissions::from_mode(0o755); // 0o755 sets the owner to read, write, and execute, and others to read and execute
    match std::fs::set_permissions(&file_path, permissions) {
        Ok(_) => info!("Set executable permissions successfully."),
        Err(e) => error!("Failed to set executable permissions: {}", e),
    }
}

// WINDOWS

fn windows(settings: &mut Settings) {
    get_and_load_preset_config(settings);
    create_entrypoint();
    set_build_dir(settings);
    set_install_dir(settings);
}

fn set_build_dir(settings: &mut Settings) {
    // Create a build directory in the working directory, check if it exists first, then save the path to the settings file.

    let build_dir = Path::new(&settings.working_dir).join(BUILD_DIR_NAME);
    settings.build_dir = build_dir.to_str().unwrap().to_string();
    // Create the build directory. If it already exists, it will just skip this step.
    std::fs::create_dir(&settings.build_dir).unwrap_or_else(|e| {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            warn!("The '{}' directory already exists. Skipping this step.", BUILD_DIR_NAME);
        } else {
            error!("Error creating the build directory: {}", e);
        }
    });
}

fn set_install_dir(settings: &mut Settings) {
    // Create an install directory in the working directory, check if it exists first, then save the path to the settings file.

    let install_dir = Path::new(&settings.working_dir).join(INSTALL_DIR_NAME);

    settings.install_dir = install_dir.to_str().unwrap().to_string();
    // Create the install directory
    std::fs::create_dir(&settings.install_dir).unwrap_or_else(|e| {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            warn!("The '{}' directory already exists. Skipping this step.", INSTALL_DIR_NAME);
        } else {
            error!("Error creating the install directory: {}", e);
        }
    });
}

fn create_entrypoint() {
    /*
    Create an appropriate file for an os to have an entry point at the project location.
    i.e. on windows it would be a .bat file, on linux it would be a .sh file, etc.
    It's a very simple file which we can just write to disk.
    Example:
        @echo off
        <path to exe> %*
    The path to the exe is already stored in the settings file.
    The location of this entrypoint file should be the same as the working directory (just like 'cpm_install.json').
    Also, it's important to set the no_init flag to true for the entrypoint.
    */
    let settings = Settings::init(false).unwrap();
    let env = settings.os;

    match env.as_str() {
        "linux" => {
            let entrypoint_path = Path::new(&settings.working_dir).join("cpm.sh");
            let entrypoint_content = format!("#!/bin/bash\n{} --no-init $@", settings.exe_path);
            match File::create(entrypoint_path.clone()) {
                Ok(mut file) => match file.write_all(entrypoint_content.as_bytes()) {
                    Ok(_) => info!(
                        "Successfully wrote the entrypoint file to disk: {:?}",
                        entrypoint_path.clone().to_str().unwrap()
                    ),
                    Err(e) => error!("Error writing the entrypoint file to disk: {}", e),
                },
                Err(e) => error!("Error creating the entrypoint file: {}", e),
            }
            // Make the file executable
            #[cfg(target_os = "linux")]
            chmod_file(&entrypoint_path);
        }
        "macos" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "windows" => {
            let entrypoint_path = Path::new(&settings.working_dir).join("cpm.bat");
            let entrypoint_content = format!("@echo off\n{} --no-init %*", settings.exe_path);
            match File::create(entrypoint_path.clone()) {
                Ok(mut file) => match file.write_all(entrypoint_content.as_bytes()) {
                    Ok(_) => info!(
                        "Successfully wrote the entrypoint file to disk: {:?}",
                        entrypoint_path.clone().to_str().unwrap()
                    ),
                    Err(e) => error!("Error writing the entrypoint file to disk: {}", e),
                },
                Err(e) => error!("Error creating the entrypoint file: {}", e),
            }
        }
        _ => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
    }
}

// GENERAL

fn get_and_load_preset_config(settings: &mut Settings) {
    // If preset already exists skip this step
    // It will most likely not be cached so we should hard check for the file in the working directory
    let config_path = Path::new(&settings.working_dir).join("cpm_install.json");
    if config_path.exists() {
        // Make sure to add the preset path to the settings file
        settings.install_json_path = config_path.to_str().unwrap().to_string();
        warn!("Preset already exists. Skipping this step.");
        return;
    }
    if let Some(_file_content) = Presets::get("cpm_install.json") {
        let destination_path = Path::new(&settings.working_dir).join("cpm_install.json");
        match write_embedded_file_to_disk("cpm_install.json", &destination_path) {
            Ok(_) => {
                info!(
                    "Successfully wrote the JSON file to disk: {:?}",
                    destination_path.to_str().unwrap()
                );
                settings.install_json_path = destination_path.to_str().unwrap().to_string();
            }
            Err(e) => error!("Error writing the JSON file to disk: {}", e),
        }
    } else {
        error!("Failed to load the JSON file.");
    }
}

fn write_embedded_file_to_disk(
    embedded_file_name: &str,
    output_file_path: &Path,
) -> std::io::Result<()> {
    if let Some(embedded_file) = Presets::get(embedded_file_name) {
        info!("Writing embedded file to disk: {:?}", output_file_path);
        let mut output_file = File::create(output_file_path)?;
        output_file.write_all(&embedded_file.data)?;
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Embedded file not found",
        ))
    }
}
