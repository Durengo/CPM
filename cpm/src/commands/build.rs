use serde::de;
use spdlog::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::commands::BuildArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;
use crate::internal::install::{ Presets, Config };
use crate::internal::cmd;

pub fn run(args: BuildArgs) {
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
        check_toolchain(&mut settings);
    }

    if let Some(generate_args) = &args.generate_project {
        check_build_type(&args);

        let build_type = if args.debug_build_type { "Debug" } else { "Release" };

        info!(
            "Generating CMake project for system type '{}' with build type '{}'",
            generate_args,
            build_type
        );
        generate_cmake_project(&mut settings, &generate_args, &build_type);
    }

    if args.build_project {
        check_build_type(&args);

        // Depending on build type set string variable as "Debug" or "Release"
        let build_type = if args.debug_build_type { "Debug" } else { "Release" };

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

fn check_build_type(args: &BuildArgs) {
    // If none are set, throw an error
    if !(args.debug_build_type || args.release_build_type) {
        error!("Build type not set. Use 'build --set-build-type <type>' to set the build type.");
        RuntimeErrors::BuildTypeNotSet.exit();
    }
    // If both are set, throw an error
    if args.debug_build_type && args.release_build_type {
        error!("Both debug and release build types set. Use only one.");
        RuntimeErrors::BuildTypeBothSet.exit();
    }
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

        debug!("Toolchain root: {}", toolchain_root);

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
                error!("Toolchain '{}' not found", toolchain_root);
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

fn generate_cmake_project(settings: &mut Settings, system_type: &str, build_type: &str) {
    let source_dir = settings.working_dir.clone();
    let build_dir = settings.build_dir.clone();
    let toolchain_path = settings.vcpkg_path.clone();

    // If system_type is "nt/msvc", then the toolchain path must be set.
    if system_type == "nt/msvc" && toolchain_path.is_empty() {
        info!(
            "Please set the toolchain (VCPKG) path for system type 'nt/msvc', using 'setup --toolchain <path>'."
        );
        RuntimeErrors::GenerateProjectNtMsvcNoToolchain.exit();
    }

    // Prepare the presets
    // Match system type string
    let preset = generate_preset(&system_type, &source_dir, &build_dir, &toolchain_path);

    // Cache system and build type and the last command.
    settings.cmake_system_type = system_type.to_string();
    settings.cmake_build_type = build_type.to_string();
    settings.last_cmake_configuration_command = preset.clone();
    settings.save_default();

    cmd::execute(preset);

    debug!("Settings: {:?}", settings);
}

fn generate_preset(
    system_type: &str,
    source_dir: &str,
    build_dir: &str,
    toolchain_path: &str
) -> Vec<String> {
    match system_type {
        "nt/msvc" => {
            vec![
                "cmake".to_string(),
                "-S".to_string(),
                source_dir.to_string(),
                "-B".to_string(),
                build_dir.to_string(),
                "-G".to_string(),
                "Visual Studio 17 2022".to_string(),
                format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain_path)
            ]
        }
        "unix/clang" => {
            vec![
                "cmake".to_string(),
                "-S".to_string(),
                source_dir.to_string(),
                "-B".to_string(),
                build_dir.to_string(),
                "-G".to_string(),
                "Ninja".to_string(),
                "-DCMAKE_C_COMPILER=clang".to_string(),
                "-DCMAKE_CXX_COMPILER=clang++".to_string()
            ]
        }
        "unix/gcc" => {
            vec![
                "cmake".to_string(),
                "-S".to_string(),
                source_dir.to_string(),
                "-B".to_string(),
                build_dir.to_string(),
                "-G".to_string(),
                "Ninja".to_string(),
                "-DCMAKE_C_COMPILER=gcc".to_string(),
                "-DCMAKE_CXX_COMPILER=g++".to_string()
            ]
        }
        _ => {
            eprintln!("Invalid system type: {}", system_type);
            vec![]
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
