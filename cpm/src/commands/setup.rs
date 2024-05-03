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

    if let Some(toolchain_path) = &args.toolchain {
        settings.toolchain_path = toolchain_path.to_string();
        // info!("Settings: {:?}", settings);
    }

    if let Some(generate_args) = &args.generate_project {
        if generate_args.len() == 2 {
            info!(
                "Generating CMake project for system type '{}' with build type '{}'",
                generate_args[0],
                generate_args[1]
            );
            generate_cmake_project(&mut settings, &generate_args[0], &generate_args[1]);
        }
    }

    if let Some(build_type) = &args.build_project {
        info!("Building CMake project with build type '{}'", build_type);
        build_cmake_project(build_type);
    }

    if let Some(install_type) = &args.install_project {
        info!("Installing CMake project with install type '{}'", install_type);
        install_cmake_project(install_type);
    }

    if let Some(what_to_clean) = &args.clean_project {
        info!("Cleaning '{}' in the CMake project", what_to_clean);
        clean_cmake_project(what_to_clean);
    }
}

fn generate_cmake_project(settings: &mut Settings, system_type: &str, build_type: &str) {
    let source_dir = settings.working_dir.clone();
    let build_dir = settings.build_dir.clone();
    let mut toolchain_path = settings.toolchain_path.clone();

    // If system_type is "nt/msvc", then the toolchain path must be set.
    if system_type == "nt/msvc" && toolchain_path.is_empty() {
        info!(
            "Please set the toolchain (VCPKG) path for system type 'nt/msvc', using 'setup --toolchain <path>'."
        );
        RuntimeErrors::GenerateProjectNtMsvcNoToolchain.exit();
    }

    // Prepare the presets
    // Match system type string
    let mut preset = generate_preset(&system_type, &source_dir, &build_dir, &toolchain_path);

    // Cache system and build type and the last command.
    settings.cmake_system_type = system_type.to_string();
    settings.cmake_build_type = build_type.to_string();
    settings.last_cmake_configuration_command = preset;

    debug!("Settings: {:?}", settings);
}

fn generate_preset(
    system_type: &str,
    source_dir: &str,
    build_dir: &str,
    toolchain_path: &str
) -> String {
    match system_type {
        "nt/msvc" => {
            vec![
                "cmake",
                "-S",
                source_dir,
                "-B",
                build_dir,
                "-G",
                "Visual Studio 17 2022",
                &format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain_path)
            ].join(" ")
        }
        "unix/clang" => {
            vec![
                "cmake",
                "-S",
                source_dir,
                "-B",
                build_dir,
                "-G",
                "Ninja",
                "-DCMAKE_C_COMPILER=clang",
                "-DCMAKE_CXX_COMPILER=clang++"
            ].join(" ")
        }
        "unix/gcc" => {
            vec![
                "cmake",
                "-S",
                source_dir,
                "-B",
                build_dir,
                "-G",
                "Ninja",
                "-DCMAKE_C_COMPILER=gcc",
                "-DCMAKE_CXX_COMPILER=g++"
            ].join(" ")
        }
        _ => {
            eprintln!("Invalid system type: {}", system_type);
            String::new() // Return an empty string if the system type is not recognized
        }
    }
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
