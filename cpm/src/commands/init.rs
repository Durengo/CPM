use serde::de;
use spdlog::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::commands::InitArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;
use crate::internal::install::{ Presets, Config };

pub fn run(args: InitArgs, _no_init: bool) {
    debug!("Running the Initialization command with arguments: {:?}", args);
    debug!("No init flag: {}", _no_init);

    if _no_init {
        RuntimeErrors::NoInitFlagSet.exit();
    }

    _ = entry();
}

fn entry() -> std::io::Result<()> {
    let mut settings = Settings::load(&Settings::get_settings_path()?)?;

    debug!("Before:\n{:?}", settings);

    settings.working_dir = std::env::current_dir()?.to_str().unwrap().to_string();
    // If working directory is the same as the executable directory, throw an error
    if settings.working_dir == settings.exe_dir {
        RuntimeErrors::WorkingDirSameAsExePath(
            settings.working_dir.clone(),
            settings.exe_dir.clone()
        ).exit();
    }

    settings.save(&Settings::get_settings_path()?)?;

    info!("Working directory: {:?}", settings.working_dir);

    os_specific();

    settings.initialized = true;
    settings.save(&Settings::get_settings_path()?)?;

    debug!("After:\n{:?}", settings);

    Ok(())
}

fn os_specific() {
    let settings = Settings::init(true).unwrap();
    let env = settings.os;

    match env.as_str() {
        "linux" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "macos" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "windows" => windows(),
        _ => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
    }
}

fn windows() {
    get_and_load_preset_config();
    create_entrypoint();
    // if let Some(file_content) = Presets::get("cpm_install.json") {
    //     let file_str = std::str::from_utf8(&file_content.data).unwrap();
    //     match serde_json::from_str::<Config>(file_str) {
    //         Ok(config) => println!("Parsed JSON: {:?}", config),
    //         Err(e) => eprintln!("Error parsing JSON: {}", e),
    //     }
    // } else {
    //     println!("Failed to load the JSON file.");
    // }
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
    let settings = Settings::init(true).unwrap();
    let env = settings.os;

    match env.as_str() {
        "linux" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "macos" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "windows" => {
            let entrypoint_path = Path::new(&settings.working_dir).join("cpm.bat");
            let entrypoint_content = format!("@echo off\n{} --no-init %*", settings.exe_path);
            match File::create(entrypoint_path) {
                Ok(mut file) => {
                    match file.write_all(entrypoint_content.as_bytes()) {
                        Ok(_) => info!("Successfully wrote the entrypoint file to disk."),
                        Err(e) => error!("Error writing the entrypoint file to disk: {}", e),
                    }
                }
                Err(e) => error!("Error creating the entrypoint file: {}", e),
            }
        }
        _ => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
    }
}

fn get_and_load_preset_config() {
    if let Some(file_content) = Presets::get("cpm_install.json") {
        // let file_str = std::str::from_utf8(&file_content.data).unwrap();
        let settings = Settings::init(false).unwrap();
        info!("Working directory: {:?}", settings.working_dir);
        let destination_path = Path::new(&settings.working_dir).join("cpm_install.json");
        match ({ write_embedded_file_to_disk("cpm_install.json", Path::new(&destination_path)) }) {
            Ok(_) => debug!("Successfully wrote the JSON file to disk."),
            Err(e) => error!("Error writing the JSON file to disk: {}", e),
        }
    } else {
        error!("Failed to load the JSON file.");
    }
}

fn write_embedded_file_to_disk(
    embedded_file_name: &str,
    output_file_path: &Path
) -> std::io::Result<()> {
    if let Some(embedded_file) = Presets::get(embedded_file_name) {
        debug!("Writing embedded file to disk: {:?}", output_file_path);
        let mut output_file = File::create(output_file_path)?;
        output_file.write_all(&embedded_file.data)?;
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Embedded file not found"))
    }
}
