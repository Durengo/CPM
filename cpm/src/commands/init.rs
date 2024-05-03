use spdlog::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::commands::InitArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;
use crate::internal::install::{ Presets, Config };

pub fn run(args: InitArgs) {
    trace!("Running the Initialization command with arguments: {:?}", args);

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
