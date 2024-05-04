use spdlog::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

use crate::commands::BuildArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;
use crate::internal::cmd;

pub fn run(args: BuildArgs) {
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
        RuntimeErrors::ProjectNotInitialized.exit();
    }

    // 'b' is for 'Build' folder and 'i' is for 'Install' folder in the working directory.
    // Both chars can be used to clean the respective folders.
    // Example: 'bi' will clean both folders.
    // Need to parse the string and clean the respective folders.
    if let Some(maybe_what_to_clean) = &args.clean_project {
        match maybe_what_to_clean {
            Some(what_to_clean) if !what_to_clean.trim().is_empty() => {
                clean_cmake_project(&settings, what_to_clean);
            }
            _ => {
                warn!(
                    "No arguments provided for cleaning. Cleaning both 'Build' and 'Install' folders."
                );
                clean_cmake_project(&settings, "bi");
            }
        }
    }

    if let Some(maybe_generate_args) = &args.generate_project {
        check_build_type(&args);

        let build_type = if args.debug_build_type {
            info!("Build Type: Debug");
            "Debug"
        } else {
            info!("Build Type: Release");
            "Release"
        };

        match maybe_generate_args {
            Some(generate_args) if !generate_args.trim().is_empty() => {
                info!(
                    "Generating CMake project for system type '{}' with build type '{}'",
                    generate_args,
                    build_type
                );
                generate_cmake_project(&mut settings, generate_args, build_type);
            }
            _ => {
                warn!(
                    "No system type provided or empty. Will attempt to use the last cmake configuration command."
                );
                let last_cmd = &settings.last_cmake_configuration_command;
                if !last_cmd.is_empty() {
                    cmd::execute_and_display_output(last_cmd.clone());
                } else {
                    error!("No previous CMake configuration command available.");
                }
            }
        }

        // if generate_args.trim().is_empty() {
        //     warn!("No system type provided. Will attempt to use last cmake configuration command.");

        //     let last_cmd = &settings.last_cmake_configuration_command;

        //     cmd::execute_and_display_output(last_cmd.clone());
        // } else {
        //     info!(
        //         "Generating CMake project for system type '{}' with build type '{}'",
        //         generate_args,
        //         build_type
        //     );
        //     generate_cmake_project(&mut settings, &generate_args, &build_type);
        // }

        info!("Project generated successfully.");
    }
    // else if !args.generate_project.is_empty() {
    //     check_build_type(&args);

    //     let build_type = if args.debug_build_type {
    //         info!("Build Type: Debug");
    //         "Debug"
    //     } else {
    //         info!("Build Type: Release");
    //         "Release"
    //     };

    //     warn!("No system type provided. Will attempt to use last cmake configuration command.");

    //     let last_cmd = &settings.last_cmake_configuration_command;

    //     cmd::execute_and_display_output(last_cmd.clone());

    //     info!("Project generated successfully.");
    // }

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

        info!("Project built successfully.");
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

        info!("Project installed successfully.");
    }
}

fn check_build_type(args: &BuildArgs) {
    // If none are set, throw an error
    if !(args.debug_build_type || args.release_build_type) {
        error!("Build type not set. Pass the appropriate flag (-r or -d).");
        RuntimeErrors::BuildTypeNotSet.exit();
    }
    // If both are set, throw an error
    if args.debug_build_type && args.release_build_type {
        error!("Both debug and release build types set. Use only one.");
        RuntimeErrors::BuildTypeBothSet.exit();
    }
}

fn generate_cmake_project(settings: &mut Settings, system_type: &str, build_type: &str) {
    let source_dir = settings.working_dir.clone();
    let build_dir = settings.build_dir.clone();
    let toolchain_path = settings.vcpkg_path.clone();

    // If system_type is "nt/msvc", then the toolchain path must be set.
    if system_type == "nt/msvc" && toolchain_path.is_empty() {
        error!(
            "Please set the toolchain (VCPKG) path for system type 'nt/msvc', using 'setup --toolchain <path>' or 'setup -a'."
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
    let _ = settings.save_default();

    cmd::execute_and_display_output(preset);

    debug!("Settings: {:#?}", settings);
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
            error!("Invalid system type: {}", system_type);
            RuntimeErrors::GenerateProjectInvalidSystemType(Some(system_type.to_string())).exit();
            vec![]
        }
    }
}

fn build_cmake_project(settings: &Settings, build_type: &str) {
    let build_dir = settings.build_dir.clone();

    cmd::execute_and_display_output(
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

    cmd::execute_and_display_output(
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
                warn!("The build directory does not exist. Skipping this step.");
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
                warn!("The install directory does not exist. Skipping this step.");
            }
            Err(e) => {
                error!("Error removing the 'Install' directory: {}", e);
            }
        }
    }
}

// Creates symlinks for all files in a given directory recursively.
// TODO: This needs more testing and development to be used in the project.
#[allow(dead_code)]
fn create_symlinks(src_dir: &Path, target_dir: &Path) -> std::io::Result<()> {
    RuntimeErrors::NotImplemented.exit();

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
                Ok(_) => info!("Symlink created for {:?}", path),
                Err(e) => error!("Failed to create symlink for {:?}: {}", path, e),
            }
        }
    }
    Ok(())
}
