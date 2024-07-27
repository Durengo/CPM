use spdlog::prelude::*;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::commands::BuildArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::cmd;
use crate::internal::codemodel_v2::{
    find_codemodel_file, generate_cmake_codemodel_v2, CMakeAPIResponse,
};
use crate::internal::settings::Settings;
// use crate::internal::codemodel_msvc::CMakeAPIResponse;

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

pub fn run(args: BuildArgs) {
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

    // Make sure to init artifacts
    let _ = settings.init_artifacts_dir();

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
                    "No arguments provided for cleaning. Cleaning both '{0}' and '{1}' folders.",
                    BUILD_DIR_NAME, INSTALL_DIR_NAME
                );
                clean_cmake_project(&settings, "bi");
            }
        }
    }

    // Check if cross-compilation flag was passed
    let mut currently_cross_compiling = false;

    if let Some(cross_compile) = &args.cross_compile {
        info!("Cross-compilation flag detected: {}", cross_compile);
        // In settings set cross_compile to true
        settings.cross_compile = true;
        settings.cross_compile_target_with_generator = cross_compile.clone();
        // This is the format target/generator, so we need to take everything before the '/'
        settings.cross_compile_target =
            cross_compile.split('/').collect::<Vec<&str>>()[0].to_string();
        let _ = settings.save_default();

        // Set an internal flag so that if we pass cross-compilation flag we do not initiate default build process
        currently_cross_compiling = true;
    }

    if currently_cross_compiling {
        info!("Cross-compilation detected. Running cross-compilation build process.");

        check_build_type(&args);

        let build_type = if args.debug_build_type {
            info!("Build Type: Debug");
            "Debug"
        } else {
            info!("Build Type: Release");
            "Release"
        };

        cache_cmake_build_type(&mut settings, build_type);

        export_crucial_variables_to_root_file(&settings);

        generate_cmake_codemodel_v2(&settings);

        if let Some(_maybe_generate_args) = &args.generate_project {
            generate_cmake_project_cross_compilation(&mut settings, build_type);

            cache_cmake_targets(&mut settings);
            info!("Project generated successfully.");
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

            cache_cmake_build_type(&mut settings, build_type);

            build_cmake_project_cross_compilation(&settings, build_type);

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

        // Turn this off after we are done with cross-compilation
        settings.cross_compile = false;
        let _ = settings.save_default();

        return;
    } else {
        info!("Cross-compilation not detected. Running default build process.");
        clean_cross_compile_target(&settings);
        export_crucial_variables_to_root_file(&settings);

        if let Some(maybe_generate_args) = &args.generate_project {
            check_build_type(&args);

            let build_type = if args.debug_build_type {
                info!("Build Type: Debug");
                "Debug"
            } else {
                info!("Build Type: Release");
                "Release"
            };

            cache_cmake_build_type(&mut settings, build_type);

            // clean_cross_compile_target(&settings);
            // export_crucial_variables_to_root_file(&settings);

            generate_cmake_codemodel_v2(&settings);

            match maybe_generate_args {
                Some(generate_args) if !generate_args.trim().is_empty() => {
                    info!(
                        "Generating CMake project for system type '{}' with build type '{}'",
                        generate_args, build_type
                    );
                    generate_cmake_project(&mut settings, generate_args, build_type);
                }
                _ => {
                    warn!(
                        "No system type provided or empty. Will attempt to use the last cmake configuration command."
                    );
                    let last_cmd = &settings.last_cmake_configuration_command;
                    if !last_cmd.is_empty() {
                        cmd::execute_and_display_output_live(last_cmd.clone());
                    } else {
                        error!("No previous CMake configuration command available.");
                    }
                }
            }

            cache_cmake_targets(&mut settings);

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

            cache_cmake_build_type(&mut settings, build_type);

            // clean_cross_compile_target(&settings);
            // export_crucial_variables_to_root_file(&settings);

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

            // clean_cross_compile_target(&settings);
            // export_crucial_variables_to_root_file(&settings);

            install_cmake_project(&settings, build_type);

            info!("Project installed successfully.");
        }
    }

    if args.source_targets {
        cache_cmake_targets(&mut settings);
    }
}

fn cache_cmake_build_type(settings: &mut Settings, build_type: &str) {
    // info!("Caching CMake build type: {}", build_type);
    settings.cmake_build_type = build_type.to_string();
    let _ = settings.save_default();
}

fn cache_cmake_targets(settings: &mut Settings) {
    info!(
        "Caching CMake targets based on build type: {}",
        settings.cmake_build_type
    );

    // Double check that the build has been generated
    // Simple check to see if the .cmake/api/v1/reply directory exists
    if !Path::new(&settings.build_dir)
        .join(".cmake/api/v1/reply")
        .exists()
    {
        error!("CMake project has not been generated. Run 'cpm build --generate-project' first.");
        RuntimeErrors::CMakeProjectNotGenerated.exit();
    }

    let reply_dir = Path::new(&settings.build_dir).join(".cmake/api/v1/reply");

    let mut targets = Vec::new();

    // Find the codemodel-v2-<RANDOM_HASH>.json file
    match find_codemodel_file(&reply_dir) {
        Ok(json_path) => {
            match fs::read_to_string(&json_path) {
                Ok(contents) => {
                    let api_response: Result<CMakeAPIResponse, _> = serde_json::from_str(&contents);
                    match api_response {
                        Ok(response) => {
                            // Log
                            // info!("Found CMake API response: {:#?}", response);
                            // Depending on the compiler and build system we need to capture targets differently

                            // NT/MSVC
                            if settings.cmake_system_type == "nt/msvc" {
                                // Filter configurations by the current build type
                                response
                                    .configurations
                                    .iter()
                                    .filter(|config| config.name == settings.cmake_build_type)
                                    .flat_map(|config| &config.targets)
                                    .for_each(|target| {
                                        // info!(
                                        //     "Found target for {} build: {}",
                                        //     settings.cmake_build_type, target.name
                                        // );
                                        targets.push(target.name.clone());
                                    });
                            }
                            // UNIX/Clang or UNIX/GCC
                            else if settings.cmake_system_type == "unix/clang"
                                || settings.cmake_system_type == "unix/gcc"
                            {
                                // Ignore the build type and just grab all targets as in unix codemodel is different
                                response
                                    .configurations
                                    .iter()
                                    .flat_map(|config| &config.targets)
                                    .for_each(|target| {
                                        // info!(
                                        //     "Found target for {} build: {}",
                                        //     settings.cmake_build_type, target.name
                                        // );
                                        targets.push(target.name.clone());
                                    });
                            }
                            // OSX/Clang
                            else if settings.cmake_system_type == "make/clang"
                                || settings.cmake_system_type == "make/gcc"
                            {
                                response
                                    .configurations
                                    .iter()
                                    .filter(|config| config.name == settings.cmake_build_type)
                                    .flat_map(|config| &config.targets)
                                    .for_each(|target| {
                                        // info!(
                                        //     "Found target for {} build: {}",
                                        //     settings.cmake_build_type, target.name
                                        // );
                                        targets.push(target.name.clone());
                                    });
                            }
                        }
                        Err(e) => error!("Failed to parse JSON: {}", e),
                    }
                }
                Err(e) => error!("Error reading JSON file: {}", e),
            }
        }
        Err(e) => error!("Error finding JSON file: {}", e),
    }

    info!(
        "Found targets for {} build: {:#?}",
        settings.cmake_build_type, targets
    );
    settings.cmake_targets = targets;
    let _ = settings.save_default();
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

#[allow(unreachable_code)]
#[allow(unused_variables)]
fn build_cmake_project_cross_compilation(settings: &Settings, build_type: &str) {
    let build_dir = settings.build_dir.clone();
    // let source_dir_wsl = cmd::convert_to_wsl_path(source_dir);
    let build_dir_wsl = cmd::convert_to_wsl_path(&build_dir);

    error!("Cross-compilation build not fully implemented yet.");
    return;

    cmd::execute_wsl_command(vec![
        "cmake".to_string(),
        "--build".to_string(),
        build_dir_wsl.clone(),
        "--config".to_string(),
        build_type.to_string(),
    ]);
}

fn generate_cmake_project_cross_compilation(settings: &mut Settings, build_type: &str) {
    let source_dir = settings.working_dir.clone();
    let build_dir = settings.build_dir.clone();
    let cross_compile_target_with_generator = settings.cross_compile_target_with_generator.clone();

    // Prepare the presets
    // Match system type string
    let preset = generate_preset_for_cross_compilation(
        &cross_compile_target_with_generator,
        &source_dir,
        &build_dir,
    );

    debug!("Preset: {:#?}", preset);

    // Cache system and build type and the last command.
    settings.cmake_system_type = cross_compile_target_with_generator.to_string();
    settings.cmake_build_type = build_type.to_string();
    settings.last_cmake_configuration_command = preset.clone();
    let _ = settings.save_default();

    // cmd::execute_and_display_output_live(preset);
    cmd::execute_wsl_command(preset);

    debug!("Settings: {:#?}", settings);
}

// Generate preset for cross compilation targets
fn generate_preset_for_cross_compilation(
    cross_compile_target_with_generator: &str,
    source_dir: &str,
    build_dir: &str,
) -> Vec<String> {
    let source_dir_wsl = cmd::convert_to_wsl_path(source_dir);
    let build_dir_wsl = cmd::convert_to_wsl_path(build_dir);

    match cross_compile_target_with_generator {
        "pi4/umake" => {
            vec![
                "cmake".to_string(),
                "-S".to_string(),
                format!("\"{}\"", source_dir_wsl),
                "-B".to_string(),
                format!("\"{}\"", build_dir_wsl),
                "-G".to_string(),
                "\"Unix Makefiles\"".to_string(),
                "-DCMAKE_C_COMPILER=/opt/cross-pi-gcc/bin/arm-linux-gnueabihf-gcc".to_string(),
                // "-DCMAKE_C_COMPILER=/home/{USER}/raspberrypi/rootfs/cross-pi-gcc/bin/arm-linux-gnueabihf-gcc".to_string(),
                "-DCMAKE_CXX_COMPILER=/opt/cross-pi-gcc/bin/arm-linux-gnueabihf-g++".to_string(),
                // "-DCMAKE_CXX_COMPILER=/home/{USER}/raspberrypi/rootfs/cross-pi-gcc/bin/arm-linux-gnueabihf-g++".to_string(),
                // "-DCMAKE_SYSROOT=/opt/cross-pi-gcc-10.3.0-64/aarch64-linux-gnu".to_string(),
                // "-DCMAKE_FIND_ROOT_PATH=/opt/cross-pi-gcc-10.3.0-64/aarch64-linux-gnu".to_string(),
                // "-DCMAKE_FIND_ROOT_PATH=/home/{USER}/raspberrypi/rootfs".to_string(),
                "-DCMAKE_FIND_ROOT_PATH_MODE_PROGRAM=NEVER".to_string(),
                "-DCMAKE_FIND_ROOT_PATH_MODE_LIBRARY=ONLY".to_string(),
                "-DCMAKE_FIND_ROOT_PATH_MODE_INCLUDE=ONLY".to_string(),
                // "-DCMAKE_SYSTEM_NAME=Linux".to_string(),
                // "-DCMAKE_SYSTEM_PROCESSOR=arm".to_string(),
                // "-DCMAKE_C_COMPILER=arm-linux-gnueabihf-gcc".to_string(),
                // "-DCMAKE_CXX_COMPILER=arm-linux-gnueabihf-g++".to_string(),
                // "-DCMAKE_FIND_ROOT_PATH=/usr/arm-linux-gnueabihf".to_string(),
                // "-DCMAKE_FIND_ROOT_PATH_MODE_PROGRAM=NEVER".to_string(),
                // "-DCMAKE_FIND_ROOT_PATH_MODE_LIBRARY=ONLY".to_string(),
                // "-DCMAKE_FIND_ROOT_PATH_MODE_INCLUDE=ONLY".to_string(),

                // "-DCMAKE_SYSTEM_NAME=Linux".to_string(),
                // "-DCMAKE_SYSTEM_PROCESSOR=arm".to_string(),
                // "-DCMAKE_C_COMPILER=/usr/bin/arm-linux-gnueabihf-gcc".to_string(),
                // "-DCMAKE_CXX_COMPILER=/usr/bin/arm-linux-gnueabihf-g++".to_string(),
                // // "-DCMAKE_SYSROOT=/usr/arm-linux-gnueabihf".to_string(),
                // "-DCMAKE_FIND_ROOT_PATH=/usr/arm-linux-gnueabihf".to_string(),
                // "-DCMAKE_FIND_ROOT_PATH_MODE_PROGRAM=NEVER".to_string(),
                // "-DCMAKE_FIND_ROOT_PATH_MODE_LIBRARY=ONLY".to_string(),
                // "-DCMAKE_FIND_ROOT_PATH_MODE_INCLUDE=ONLY".to_string(),
            ]
        }
        _ => {
            error!(
                "Invalid cross compilation target: {}",
                cross_compile_target_with_generator
            );
            RuntimeErrors::CrossCompilationGenerateProjectInvalidTarget(
                cross_compile_target_with_generator.to_string(),
            )
            .exit();
            vec![]
        }
    }
}

fn generate_cmake_project(settings: &mut Settings, system_type: &str, build_type: &str) {
    let source_dir = settings.working_dir.clone();
    let build_dir = settings.build_dir.clone();
    let toolchain_path = settings.vcpkg_path.clone();

    // Throw error if building msvc for non-windows
    if system_type == "nt/msvc" && !cfg!(target_os = "windows") {
        error!("Cannot build for 'nt/msvc' on a non-Windows system.");
        RuntimeErrors::GenerateProjectNtMsvcNonWindows.exit();
    }

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

    cmd::execute_and_display_output_live(preset);

    debug!("Settings: {:#?}", settings);
}

fn generate_preset(
    system_type: &str,
    source_dir: &str,
    build_dir: &str,
    toolchain_path: &str,
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
                format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain_path),
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
                "-DCMAKE_CXX_COMPILER=clang++".to_string(),
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
                "-DCMAKE_CXX_COMPILER=g++".to_string(),
            ]
        }
        "make/clang" => {
            vec![
                "cmake".to_string(),
                "-S".to_string(),
                source_dir.to_string(),
                "-B".to_string(),
                build_dir.to_string(),
                "-G".to_string(),
                "Unix Makefiles".to_string(),
                "-DCMAKE_C_COMPILER=clang".to_string(),
                "-DCMAKE_CXX_COMPILER=clang++".to_string(),
            ]
        }
        "make/gcc" => {
            vec![
                "cmake".to_string(),
                "-S".to_string(),
                source_dir.to_string(),
                "-B".to_string(),
                build_dir.to_string(),
                "-G".to_string(),
                "Unix Makefiles".to_string(),
                "-DCMAKE_C_COMPILER=gcc".to_string(),
                "-DCMAKE_CXX_COMPILER=g++".to_string(),
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

    cmd::execute_and_display_output_live(vec![
        "cmake".to_string(),
        "--build".to_string(),
        build_dir.clone(),
        "--config".to_string(),
        build_type.to_string(),
    ]);
}

fn install_cmake_project(settings: &Settings, build_type: &str) {
    let build_dir = settings.build_dir.clone();

    cmd::execute_and_display_output_live(vec![
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
        "-v".to_string(),
    ]);
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
                info!("Successfully removed the '{}' directory.", BUILD_DIR_NAME);
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                warn!("The build directory does not exist. Skipping this step.");
            }
            Err(e) => {
                error!("Error removing the '{}' directory: {}", BUILD_DIR_NAME, e);
            }
        }
    }

    if install_dir {
        match std::fs::remove_dir_all(&settings.install_dir) {
            Ok(_) => {
                info!("Successfully removed the '{}' directory.", INSTALL_DIR_NAME);
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                warn!("The install directory does not exist. Skipping this step.");
            }
            Err(e) => {
                error!("Error removing the '{}' directory: {}", INSTALL_DIR_NAME, e);
            }
        }
    }
}

// We will take cmake_build_type and cross_compile_target and export it to where our root CMakeLists.txt is. This will be done for each variable.
// cmake_build_type => .BUILD_TYPE
// cross_compile_target => .CROSS_COMPILE_TARGET
fn export_crucial_variables_to_root_file(settings: &Settings) {
    let artifacts = Path::new(&settings.working_dir).join("Artifacts");

    // artifacts path
    info!("Artifacts path: {}", artifacts.display());

    let build_type = format!("{}\n", settings.cmake_build_type);

    // The above is what we will include in the file but the files themselves will be called .BUILD_TYPE and .CROSS_COMPILE_TARGET for cmake_build_type and cross_compile_target respectively.
    let build_type_file = artifacts.join(".BUILD_TYPE");

    // Write the build type to the file
    match std::fs::write(&build_type_file, build_type) {
        Ok(_) => {
            info!(
                "Successfully wrote build type to file: {}",
                build_type_file.display()
            );
        }
        Err(e) => {
            error!("Failed to write build type to file: {}", e);
        }
    }

    // First check if cross_compile_target is set or not empty. If yes then just return.
    if settings.cross_compile_target.is_empty() {
        return;
    }

    // If we are not cross compiling then also return
    if !settings.cross_compile {
        return;
    }

    let cross_compile_target = format!("{}\n", settings.cross_compile_target);
    let cross_compile_target_file = artifacts.join(".CROSS_COMPILE_TARGET");

    // Write the cross compile target to the file
    match std::fs::write(&cross_compile_target_file, cross_compile_target) {
        Ok(_) => {
            info!(
                "Successfully wrote cross compile target to file: {}",
                cross_compile_target_file.display()
            );
        }
        Err(e) => {
            error!("Failed to write cross compile target to file: {}", e);
        }
    }
}

// Clean up .CROSS_COMPILE_TARGET
fn clean_cross_compile_target(settings: &Settings) {
    let artifacts = Path::new(&settings.working_dir).join("Artifacts");
    let cross_compile_target_file = artifacts.join(".CROSS_COMPILE_TARGET");

    match std::fs::remove_file(&cross_compile_target_file) {
        Ok(_) => {
            info!(
                "Successfully removed the cross compile target file: {}",
                cross_compile_target_file.display()
            );
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            warn!("The cross compile target file does not exist. Skipping this step.");
        }
        Err(e) => {
            error!("Error removing the cross compile target file: {}", e);
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
            create_platform_specific_symlink(path, &target_path)?;
        }
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn create_platform_specific_symlink(path: &Path, target_path: &Path) -> std::io::Result<()> {
    match std::os::windows::fs::symlink_file(path, target_path) {
        Ok(_) => {
            info!("Symlink created for {:?}", path);
            Ok(())
        }
        Err(e) => {
            error!("Failed to create symlink for {:?}: {}", path, e);
            Err(e)
        }
    }
}

#[cfg(target_os = "linux")]
fn create_platform_specific_symlink(path: &Path, target_path: &Path) -> std::io::Result<()> {
    match std::os::unix::fs::symlink(path, target_path) {
        Ok(_) => {
            info!("Symlink created for {:?}", path);
            Ok(())
        }
        Err(e) => {
            error!("Failed to create symlink for {:?}: {}", path, e);
            Err(e)
        }
    }
}

#[cfg(target_os = "macos")]
fn create_platform_specific_symlink(path: &Path, target_path: &Path) -> std::io::Result<()> {
    match std::os::unix::fs::symlink(path, target_path) {
        Ok(_) => {
            info!("Symlink created for {:?}", path);
            Ok(())
        }
        Err(e) => {
            error!("Failed to create symlink for {:?}: {}", path, e);
            Err(e)
        }
    }
}
