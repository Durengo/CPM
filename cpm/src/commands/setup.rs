use serde::de;
use spdlog::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::commands::SetupArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;
use crate::internal::install::{ Presets, Config };

pub fn run(args: SetupArgs) {
    debug!("Running the Initialization command with arguments: {:?}", args);

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

    info!("Working directory: {:?}", settings.working_dir);

    if let Some(generate_args) = &args.generate_project {
        if generate_args.len() == 2 {
            println!(
                "Generating CMake project for system type '{}' with build type '{}'",
                generate_args[0],
                generate_args[1]
            );
            generate_cmake_project(&generate_args[0], &generate_args[1]);
        }
    }

    if let Some(build_type) = &args.build_project {
        println!("Building CMake project with build type '{}'", build_type);
        build_cmake_project(build_type);
    }

    if let Some(install_type) = &args.install_project {
        println!("Installing CMake project with install type '{}'", install_type);
        install_cmake_project(install_type);
    }

    if let Some(what_to_clean) = &args.clean_project {
        println!("Cleaning '{}' in the CMake project", what_to_clean);
        clean_cmake_project(what_to_clean);
    }
}

fn generate_cmake_project(system_type: &str, build_type: &str) {
    println!("Generating project for {} with {}", system_type, build_type);
}

fn build_cmake_project(build_type: &str) {
    println!("Building project with {}", build_type);
}

fn install_cmake_project(install_type: &str) {
    println!("Installing project with {}", install_type);
}

fn clean_cmake_project(what_to_clean: &str) {
    println!("Cleaning up {}", what_to_clean);
}
