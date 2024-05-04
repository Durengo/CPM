use serde::de;
use spdlog::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

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

        let build_type = if args.debug_build_type {
            info!("Build Type: Debug");
            "Debug"
        } else {
            info!("Build Type: Release");
            "Release"
        };

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
        let build_type = if args.debug_build_type {
            info!("Build Type: Debug");
            "Debug"
        } else {
            info!("Build Type: Release");
            "Release"
        };

        build_cmake_project(&settings, build_type);
    }

    if args.install_project {
        check_build_type(&args);

        // Depending on build type set string variable as "Debug" or "Release"
        let build_type = if args.debug_build_type {
            info!("Build Type: Debug");
            "Debug"
        } else {
            info!("Build Type: Release");
            "Release"
        };

        install_cmake_project(&settings, build_type);
    }

    // 'b' is for 'Build' folder and 'i' is for 'Install' folder in the working directory.
    // Both chars can be used to clean the respective folders.
    // Example: 'bi' will clean both folders.
    // Need to parse the string and clean the respective folders.
    if let Some(what_to_clean) = &args.clean_project {
        clean_cmake_project(&settings, what_to_clean);
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

fn build_cmake_project(settings: &Settings, build_type: &str) {
    let build_dir = settings.build_dir.clone();

    cmd::execute(
        vec![
            "cmake".to_string(),
            "--build".to_string(),
            build_dir.clone(),
            "--config".to_string(),
            build_type.to_string()
        ]
    );
}

fn install_cmake_project(settings: &Settings, build_type: &str) {
    let build_dir = settings.build_dir.clone();

    cmd::execute(
        vec![
            "cmake".to_string(),
            "--install".to_string(),
            build_dir.clone(),
            "--prefix".to_string(),
            // Create a new path using settings.os_release and build_type
            // i.e. <install_dir>/<os_release>/<build_type>
            Path::new(&settings.install_dir)
                .join(&settings.os_release)
                .join(build_type)
                .to_str()
                .unwrap()
                .to_string(),
            "--config".to_string(),
            build_type.to_string(),
            "-v".to_string()
        ]
    );
}

fn clean_cmake_project(settings: &Settings, what_to_clean: &str) {
    let mut build_dir = false;
    let mut install_dir = false;

    for c in what_to_clean.chars() {
        match c {
            'b' => {
                build_dir = true;
            }
            'i' => {
                install_dir = true;
            }
            _ => {
                error!("Invalid character '{}' in clean command", c);
                RuntimeErrors::InvalidCleanCommand(c).exit();
            }
        }
    }

    if build_dir {
        match std::fs::remove_dir_all(&settings.build_dir) {
            Ok(_) => {
                info!("Successfully removed the 'Build' directory.");
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                info!("The build directory does not exist. Skipping this step.");
            }
            Err(e) => {
                error!("Error removing the 'Build' directory: {}", e);
            }
        }
    }

    if install_dir {
        match std::fs::remove_dir_all(&settings.install_dir) {
            Ok(_) => {
                info!("Successfully removed the 'Install' directory.");
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                info!("The install directory does not exist. Skipping this step.");
            }
            Err(e) => {
                error!("Error removing the 'Install' directory: {}", e);
            }
        }
    }
}

// Creates symlinks for all files in a given directory recursively.
// TODO: This needs more testing and development to be used in the project.
fn create_symlinks(src_dir: &Path, target_dir: &Path) -> std::io::Result<()> {
    for entry in WalkDir::new(src_dir) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            // Attempt to construct the target path for the symlink
            let relative_path = match path.strip_prefix(src_dir) {
                Ok(rel_path) => rel_path,
                Err(e) => {
                    // Convert the StripPrefixError to an io::Error
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                }
            };
            let target_path = target_dir.join(relative_path);

            // Ensure the target directory exists
            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Create the symlink
            match std::os::windows::fs::symlink_file(path, &target_path) {
                Ok(_) => println!("Symlink created for {:?}", path),
                Err(e) => eprintln!("Failed to create symlink for {:?}: {}", path, e),
            }
        }
    }
    Ok(())
}
