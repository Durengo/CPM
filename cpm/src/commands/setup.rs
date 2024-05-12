use spdlog::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::commands::SetupArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::cmd;
use crate::internal::install::Config;
use crate::internal::settings::Settings;

pub fn run(args: SetupArgs) {
    debug!(
        "Running the Initialization command with arguments: {:#?}",
        args
    );

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

    let config = match retrieve_install(settings.install_json_path.as_ref()) {
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
            "linux" => {
                selected_os = platform.to_string();
            }
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
            "linux" => {
                selected_os = settings.os.to_string();
            }
            "macos" => RuntimeErrors::NotSupportedOS(Some(settings.os.to_string())).exit(),
            _ => {
                RuntimeErrors::NotSupportedOS(Some(settings.os.to_string())).exit();
                return;
            }
        }
    }

    debug!("Selected OS: {}", selected_os);

    if let Some(toolchain_path) = &args.toolchain {
        debug!("Path before trim: {}", toolchain_path);
        // If the provided path has any '/' or '\' characters at the very end, remove them
        let toolchain_path = toolchain_path.trim_end_matches(|c| (c == '/' || c == '\\'));
        debug!("Path after trim: {}", toolchain_path);
        check_toolchain(&mut settings, toolchain_path.to_string());
        // Set the toolchain path only if it's defined
        settings.toolchain_path = toolchain_path.to_string();
        let _ = settings.save_default();
        return;
    }
    // Auto detect toolchain and run setup.
    if args.auto_toolchain_path {
        auto_toolchain_path(&mut settings, &config, &selected_os);

        return;
    }
    // Auto detect toolchain and run setup otherwise manually set up toolchain.
    if args.no_toolchain_path {
        return;
    }
    // Use provided path and try to run setup.
    if let Some(_toolchain_path) = &args.use_toolchain_path {
        return;
    }
}

fn retrieve_install(file_path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    // Read the contents into a string
    let mut config_data = String::new();
    file.read_to_string(&mut config_data)
        .map_err(|e| format!("Failed to read file data: {}", e))?;

    // Deserialize the JSON string into Config struct
    let config: Config = serde_json::from_str(&config_data)
        .map_err(|e| format!("Failed to parse JSON data: {}", e))?;

    // Return the deserialized config
    Ok(config)
}

fn check_toolchain(settings: &mut Settings, toolchain_path: String) {
    // Run through a match of know toolchains and find their appropriate .cmake file.
    // Current list of know toolchains:
    // - VCPKG
    if !toolchain_path.is_empty() {
        // Normalize the path to use consistent path separators
        let normalized_path = normalize_path_separator(&toolchain_path);

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
                    let _ = settings.save_default();
                } else {
                    error!(
                        "VCPKG CMake toolchain file not found at: {}",
                        vcpkg_cmake_path
                    );
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

fn auto_toolchain_path(settings: &mut Settings, config: &Config, os: &str) {
    info!("Auto detecting toolchain path");

    let mut skip_toolchain = false;
    // First check if the toolchain path is already set
    if !settings.toolchain_path.is_empty() {
        warn!("Toolchain path already set: {}", settings.toolchain_path);
        settings.using_toolchain = true;
        let _ = settings.save_default();
        skip_toolchain = true;
    }

    // If not set we need to find the toolchain path. This is OS specific.
    // Run commands to find the toolchain path.
    // On Windows we can use 'where.exe' to find the path of a given executable.
    if !skip_toolchain {
        toolchain_usage(settings, config);
    }

    // OS specific setup

    match os {
        "windows" => {
            windows_install(settings, config);
        }
        "linux" => {
            linux_install(settings, config);
        }
        _ => {
            RuntimeErrors::NotSupportedOS(Some(settings.os.to_string())).exit();
        }
    }
}

fn toolchain_usage(settings: &mut Settings, config: &Config) {
    match settings.os.as_str() {
        "windows" => {
            // We need to look at config file to see what toolchain to look for.
            // Retrieve this from Config.windows.toolchain
            if let Some(windows_config) = &config.config.windows {
                // Use windows_config by reference
                let toolchain = &windows_config.toolchain;

                // If toolchain is empty, disable it
                if toolchain.is_empty() {
                    error!("No toolchain found. Turning off toolchain usage.");
                    settings.using_toolchain = false;
                    let _ = settings.save_default();
                    return;
                }

                info!("Using Windows toolchain: {}", toolchain);

                // Returns a trimmed string (without \r\n line endings)
                let toolchain_path = cmd::execute_and_return_output(vec![
                    "where".to_string(),
                    toolchain.to_string(),
                ]);
                if !toolchain_path.is_empty() {
                    debug!(
                        "{} path found: {}",
                        toolchain.to_uppercase(),
                        toolchain_path
                    );
                    // Normalize the path to use consistent path separators
                    let normalized_path = normalize_path_separator(&toolchain_path);
                    // Remove '\\vcpkg.exe' from the path
                    let normalized_path = normalized_path.trim_end_matches("\\vcpkg.exe");
                    debug!(
                        "Normalized {} path: {}",
                        toolchain.to_uppercase(),
                        normalized_path
                    );
                    info!(
                        "Toolchain {} found: {}",
                        toolchain.to_uppercase(),
                        normalized_path
                    );

                    settings.toolchain_path = normalized_path.to_string();
                    settings.using_toolchain = true;
                    let _ = settings.save_default();
                    // Now we have the path to the toolchain but still need to find the .cmake file. We already have a function for this.
                    check_toolchain(settings, settings.toolchain_path.clone());
                } else {
                    error!("No toolchain found. Turning off toolchain usage.");
                    settings.using_toolchain = false;
                    let _ = settings.save_default();
                    // No need for error.
                    // RuntimeErrors::ToolchainNotFound("VCPKG".to_string()).exit();
                }
            } else {
                error!("No Windows configuration found in the install config");
                RuntimeErrors::ConfigParseError(Some("Windows".to_string())).exit();
            }
        }
        "linux" => {
            // No supported toolchains for Linux yet.
            warn!("No supported toolchains for Linux yet.");
            settings.using_toolchain = false;
            let _ = settings.save_default();
        }
        _ => {
            RuntimeErrors::NotSupportedOS(Some(settings.os.to_string())).exit();
        }
    }
}

// LINUX

fn linux_install(settings: &Settings, config: &Config) {
    let linux_config = match &config.config.linux {
        Some(linux) => linux,
        None => {
            error!("No Linux configuration found in the install config");
            return;
        }
    };

    debug!("Linux Config:\n{:#?}", linux_config);

    linux_check_dependencies(config);
    linux_check_libraries(settings, config);
    linux_check_instructions(settings, config);
}

fn linux_check_dependencies(config: &Config) {
    // If needed have special mappings for specific prerequisites.
    // Example: To check cmake, we can use 'cmake --version' and check the output.
    // But the output has some additional text which we don't need.
    info!("Checking prerequisites");

    // Retrieve prerequisites from the Config
    if let Some(linux_config) = &config.config.linux {
        let deps = &linux_config.dependencies;

        // If there are no dependencies, return early
        if deps.is_empty() {
            trace!("No dependencies found");
            return;
        }

        // Iterate over each dependency
        for dep in deps {
            // Check against premade mappings
            match dep.as_str() {
                // Check if cmake is installed
                "cmake" => {
                    let cmake_version = cmd::execute_and_return_output(vec![
                        "cmake".to_string(),
                        "--version".to_string(),
                    ]);
                    if cmake_version.is_empty() {
                        error!("CMake not found. Please install CMake and try again.");
                        RuntimeErrors::PrerequisiteNotFound(Some("cmake".to_string())).exit();
                    } else {
                        // Might produce this in the output: 'CMake suite maintained and supported by Kitware (kitware.com/cmake).' remove this.
                        let cmake_version = cmake_version.lines().next().unwrap_or_default();
                        info!("CMake found: {}", cmake_version);
                    }
                }
                // Check if git is installed
                "git" => {
                    let git_version = cmd::execute_and_return_output(vec![
                        "git".to_string(),
                        "--version".to_string(),
                    ]);
                    if git_version.is_empty() {
                        error!("Git not found. Please install Git and try again.");
                        RuntimeErrors::PrerequisiteNotFound(Some("git".to_string())).exit();
                    } else {
                        info!("Git found: {}", git_version);
                    }
                }
                "rustc" => {
                    let rustc_version = cmd::execute_and_return_output(vec![
                        "rustc".to_string(),
                        "--version".to_string(),
                    ]);
                    if rustc_version.is_empty() {
                        error!("Rust not found. Please install Rust and try again.");
                        RuntimeErrors::PrerequisiteNotFound(Some("rustc".to_string())).exit();
                    } else {
                        info!("Rust found: {}", rustc_version);
                    }
                }
                "cargo" => {
                    let cargo_version = cmd::execute_and_return_output(vec![
                        "cargo".to_string(),
                        "--version".to_string(),
                    ]);
                    if cargo_version.is_empty() {
                        error!("Cargo not found. Please install Cargo and try again.");
                        RuntimeErrors::PrerequisiteNotFound(Some("cargo".to_string())).exit();
                    } else {
                        info!("Cargo found: {}", cargo_version);
                    }
                }
                // Since the prerequisite is not in the mappings, just check if the executable exists
                _ => {
                    let dep_path = cmd::execute_and_return_output(vec![
                        "whereis".to_string(),
                        dep.to_string(),
                    ]);
                    if dep_path.is_empty() {
                        RuntimeErrors::PrerequisiteNotFound(Some(dep.to_string())).exit();
                    }
                    // Need to handle the output of whereis if it's 'x_dep:' - i.e error
                    else if dep_path.ends_with(":") {
                        RuntimeErrors::PrerequisiteNotFound(Some(dep.to_string())).exit();
                    } else {
                        info!("{} found: {}", dep, dep_path);
                    }
                }
            }
        }
    }
}

fn linux_check_libraries(_settings: &Settings, config: &Config) {
    info!("Checking packages to install");

    // Retrieve packages from the Config
    if let Some(linux_config) = &config.config.linux {
        let libraries = &linux_config.libraries;

        // If there are no packages, return early
        if libraries.is_empty() {
            trace!("No libraries found");
            return;
        }

        for library in libraries {
            if !linux_is_library_installed(library) {
                linux_install_package(library);
            } else {
                info!("Package already installed: {:#}", library.library);
            }
        }
    }
}

fn linux_is_library_installed(library: &crate::internal::install::Library) -> bool {
    let check_command = match library.distribution.as_str() {
        "arch" => format!("pacman -Q {}", library.library),
        "ubuntu" => format!("dpkg -l {}", library.library),
        _ => {
            RuntimeErrors::UnsupportedLinuxDistribution(Some(library.distribution.clone())).exit();
            return false;
        }
    };

    // TODO: At the moment on Arch executing pacman will always throw an error
    let output = cmd::execute_and_return_output(vec![(check_command).to_string()]);

    if output.is_empty() {
        false
    } else {
        output.contains(&library.library)
    }
}

fn linux_install_package(library: &crate::internal::install::Library) {
    let _check_command = match library.distribution.as_str() {
        "arch" => format!("sudo pacman -S {}", library.library),
        "ubuntu" => format!("dpkg -l {}", library.library),
        _ => {
            RuntimeErrors::UnsupportedLinuxDistribution(Some(library.distribution.clone())).exit();
            return;
        }
    };

    // For now just throw error that the package should be installed manually.
    warn!(
        "Make sure to install the package manually: {}",
        library.library
    );

    // // TODO: At the moment on Arch executing pacman will always throw an error
    // let output = cmd::execute_and_return_output(vec![
    //     (check_command).to_string(),
    // ]);

    // if output.is_empty() {
    //     warn!("Failed to install package: {}", library.library);
    // } else {
    //     info!("Installed package: {}", library.library);
    // }
}

fn linux_check_instructions(_settings: &Settings, config: &Config) {
    info!("Checking post install instructions");

    // Retrieve instructions from the Config
    if let Some(linux_config) = &config.config.linux {
        let instructions = &linux_config.instructions;

        // If there are no instructions, return early
        if instructions.is_empty() {
            trace!("No instructions found");
            return;
        }

        for instruction in instructions {
            // Check against premade mappings
            match instruction.as_str() {
                _ => {
                    // No exit here as it's not a critical error.
                    RuntimeErrors::PostInstallNoDefinition(Some(instruction.to_string()));
                }
            }
        }
    }
}

// WINDOWS

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

    windows_check_prerequisites(config);
    windows_install_libraries(settings, config);
    windows_post_install(settings, config);
}

fn windows_check_prerequisites(config: &Config) {
    // If needed have special mappings for specific prerequisites.
    // Example: To check cmake, we can use 'cmake --version' and check the output.
    // But the output has some additional text which we don't need.
    info!("Checking prerequisites");

    // Retrieve prerequisites from the Config
    if let Some(windows_config) = &config.config.windows {
        // Use windows_config by reference
        let prereqs = &windows_config.prerequisites;

        // If there are no prerequisites, return early
        if prereqs.is_empty() {
            trace!("No prerequisistes found");
            return;
        }

        // Iterate over each prerequisite
        for prereq in prereqs {
            // Check against premade mappings
            match prereq.as_str() {
                // Check if cmake is installed
                "cmake" => {
                    let cmake_version = cmd::execute_and_return_output(vec![
                        "cmake".to_string(),
                        "--version".to_string(),
                    ]);
                    if cmake_version.is_empty() {
                        error!("CMake not found. Please install CMake and try again.");
                        RuntimeErrors::PrerequisiteNotFound(Some("cmake".to_string())).exit();
                    } else {
                        // Might produce this in the output: 'CMake suite maintained and supported by Kitware (kitware.com/cmake).' remove this.
                        let cmake_version = cmake_version.lines().next().unwrap_or_default();
                        info!("CMake found: {}", cmake_version);
                    }
                }
                // Check if git is installed
                "git" => {
                    let git_version = cmd::execute_and_return_output(vec![
                        "git".to_string(),
                        "--version".to_string(),
                    ]);
                    if git_version.is_empty() {
                        error!("Git not found. Please install Git and try again.");
                        RuntimeErrors::PrerequisiteNotFound(Some("git".to_string())).exit();
                    } else {
                        info!("Git found: {}", git_version);
                    }
                }
                "vcpkg" => {
                    let vcpkg_version = cmd::execute_and_return_output(vec![
                        "vcpkg".to_string(),
                        "--version".to_string(),
                    ]);
                    if vcpkg_version.is_empty() {
                        error!("VCPKG not found. Please install VCPKG and try again.");
                        RuntimeErrors::PrerequisiteNotFound(Some("vcpkg".to_string())).exit();
                    } else {
                        // Might produce this in the output: 'See LICENSE.txt for license information.' remove this.
                        let vcpkg_version = vcpkg_version.lines().next().unwrap_or_default();
                        info!("VCPKG found: {}", vcpkg_version);
                    }
                }
                "rustc" => {
                    let rustc_version = cmd::execute_and_return_output(vec![
                        "rustc".to_string(),
                        "--version".to_string(),
                    ]);
                    if rustc_version.is_empty() {
                        error!("Rust not found. Please install Rust and try again.");
                        RuntimeErrors::PrerequisiteNotFound(Some("rustc".to_string())).exit();
                    } else {
                        info!("Rust found: {}", rustc_version);
                    }
                }
                "cargo" => {
                    let cargo_version = cmd::execute_and_return_output(vec![
                        "cargo".to_string(),
                        "--version".to_string(),
                    ]);
                    if cargo_version.is_empty() {
                        error!("Cargo not found. Please install Cargo and try again.");
                        RuntimeErrors::PrerequisiteNotFound(Some("cargo".to_string())).exit();
                    } else {
                        info!("Cargo found: {}", cargo_version);
                    }
                }
                // Since the prerequisite is not in the mappings, just check if the executable exists
                _ => {
                    let prereq_path = cmd::execute_and_return_output(vec![
                        "where".to_string(),
                        prereq.to_string(),
                    ]);
                    if prereq_path.is_empty() {
                        RuntimeErrors::PrerequisiteNotFound(Some(prereq.to_string())).exit();
                    } else {
                        info!("{} found: {}", prereq, prereq_path);
                    }
                }
            }
        }
    }
}

fn windows_install_libraries(settings: &Settings, config: &Config) {
    // Nothing special here. We just run to toolchain commands (vcpkg install) against the specific triplet.
    // Of course we should check if the library is already installed beforehand.
    info!("Checking packages to install");

    // Retrieve packages from the Config
    if let Some(windows_config) = &config.config.windows {
        // Use windows_config by reference
        let packages = &windows_config.packages;

        // If there are no packages, return early
        if packages.is_empty() {
            trace!("No packages found");
            return;
        }

        // Retrieve vcpkg exe from settings
        // Combine with "/vcpkg.exe"
        let vcpkg_exe = format!("{}\\vcpkg.exe", settings.toolchain_path);

        // Iterate over each package
        for package in packages {
            // Check if the package is already installed
            let package_installed =
                cmd::execute_and_return_output(vec![vcpkg_exe.to_string(), "list".to_string()])
                    .contains(&package.library);

            if !package_installed {
                // Set triplet
                let triplet = format!("--triplet={}", package.triplet);
                // Install the package
                let output = cmd::execute_and_return_output(vec![
                    vcpkg_exe.to_string(),
                    "install".to_string(),
                    package.library.to_string(),
                    triplet,
                ]);
                if output.is_empty() {
                    RuntimeErrors::PackageInstallFailed(Some(package.library.clone())).exit();
                } else {
                    info!("Installed package: {}", package.library);
                }
            } else {
                info!("Package already installed: {}", package.library);
            }
        }
    }
}

fn windows_post_install(settings: &Settings, config: &Config) {
    // Only specially integrated matches should be here.
    info!("Checking post install commands");

    // Retrieve packages from the Config
    if let Some(windows_config) = &config.config.windows {
        // Use windows_config by reference
        let post_installs = &windows_config.post_install;

        // If there are no packages, return early
        if post_installs.is_empty() {
            trace!("No post install commands found");
            return;
        }

        // Iterate over each package
        for post_install in post_installs {
            match post_install.as_str() {
                // "intergrate_vcpkg"
                "vcpkg_integrate_install" => {
                    // Retrieve vcpkg exe from settings
                    // Combine with "/vcpkg.exe"
                    let vcpkg_exe = format!("{}\\vcpkg.exe", settings.toolchain_path);

                    let output = cmd::execute_and_return_output(vec![
                        vcpkg_exe.to_string(),
                        "integrate".to_string(),
                        "install".to_string(),
                    ]);
                    if output.is_empty() {
                        RuntimeErrors::PostInstallFailed(Some(
                            "vcpkg_integrate_install".to_string(),
                        ))
                        .exit();
                    } else {
                        info!("Post install: {}", "vcpkg_integrate_install");
                    }
                }
                _ => {
                    // No exit here as it's not a critical error.
                    RuntimeErrors::PostInstallNoDefinition(Some(post_install.to_string()));
                }
            }
        }
    }
}
