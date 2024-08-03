use spdlog::prelude::*;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::commands::CrossBuildArgs;
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

pub fn run(args: CrossBuildArgs) {
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

    settings.cross_compile = true;
    let _ = settings.save_default();

    if args.processor.is_some() {
        let processor = args.processor.clone().unwrap();
        info!("Processor set: {}", processor);

        // Currently from here we can also set the compiler
        // i.e. "{processor}-linux" for aarch64
        let compiler = format!("{}-linux", processor);

        settings.cross_compile_processor = processor;
        settings.cross_compile_compiler = compiler;
        let _ = settings.save_default();
    }

    if args.sysroot.is_some() {
        let sysroot = args.sysroot.clone().unwrap();
        info!("Sysroot path set: {}", sysroot);
        settings.cross_compile_sysroot = sysroot;
        let _ = settings.save_default();
    }

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

    if let Some(maybe_generate_args) = &args.generate_project {
        match maybe_generate_args {
            Some(generate_args) if !generate_args.trim().is_empty() => {
                info!(
                    "Generating CMake project for system type '{}' with build type '{}'",
                    generate_args, build_type
                );

                generate_cmake_project_cross_compilation(&mut settings, generate_args, build_type);
            }
            _ => {
                warn!(
                        "No system type provided or empty. Will attempt to use the last cmake configuration command."
                    );
                let last_cmd = &settings.last_cmake_configuration_command_cross_compile;
                if !last_cmd.is_empty() {
                    cmd::execute_and_display_output_live(last_cmd.clone());
                } else {
                    error!("No previous CMake configuration command available.");
                }
            }
        }

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

                            // RPI4/UMake
                            if settings.cmake_system_type == "rpi4/umake" {
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

fn check_build_type(args: &CrossBuildArgs) {
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
    // let build_dir_wsl = cmd::convert_to_wsl_path(&build_dir);
    // we should provide UNIX paths here

    cmd::execute_and_display_output_live(vec![
        "cmake".to_string(),
        "--build".to_string(),
        build_dir.clone(),
        "--config".to_string(),
        build_type.to_string(),
    ]);
}

fn generate_cmake_project_cross_compilation(
    settings: &mut Settings,
    system_type: &str,
    build_type: &str,
) {
    let source_dir = settings.working_dir.clone();
    let build_dir = settings.build_dir.clone();

    // In settings set cross_compile to true
    settings.cross_compile_target_with_generator = system_type.to_string();
    // This is the format target/generator, so we need to take everything before the '/'
    settings.cross_compile_target = system_type.split('/').collect::<Vec<&str>>()[0].to_string();

    let _ = settings.save_default();

    export_crucial_variables_to_root_file(&settings);

    generate_cmake_codemodel_v2(&settings);

    let cross_compile_target_with_generator = settings.cross_compile_target_with_generator.clone();

    // Check if processor and sysroot path are set
    if settings.cross_compile_processor.is_empty() {
        error!(
            "Cross-compile processor set. Please use the 'processor' command to set the processor."
        );
        RuntimeErrors::CrossCompilationProcessorNotSet(
            cross_compile_target_with_generator.to_string(),
        )
        .exit();
    }
    if settings.cross_compile_compiler.is_empty() {
        error!(
            "Cross-compile compiler set. Please use the 'compiler' command to set the compiler."
        );
        RuntimeErrors::CrossCompilationCompilerNotSet(
            cross_compile_target_with_generator.to_string(),
        )
        .exit();
    }
    if settings.cross_compile_sysroot.is_empty() {
        error!("Cross-compile sysroot set. Please use the 'sysroot' command to set the sysroot.");
        RuntimeErrors::CrossCompilationSysrootPathNotSet(
            cross_compile_target_with_generator.to_string(),
        )
        .exit();
    }

    // Prepare the presets
    // Match system type string
    let preset = generate_preset_for_cross_compilation(
        &cross_compile_target_with_generator,
        &source_dir,
        &build_dir,
        settings,
    );

    debug!("Preset: {:#?}", preset);

    // Cache system and build type and the last command.
    settings.cmake_system_type = cross_compile_target_with_generator.to_string();
    settings.cmake_build_type = build_type.to_string();
    settings.last_cmake_configuration_command_cross_compile = preset.clone();
    let _ = settings.save_default();

    // cmd::execute_and_display_output_live(preset);
    // cmd::execute_wsl_command(preset);
    cmd::execute_and_display_output_live(preset.clone());

    debug!("Settings: {:#?}", settings);
}

// Generate preset for cross compilation targets
fn generate_preset_for_cross_compilation(
    cross_compile_target_with_generator: &str,
    source_dir: &str,
    build_dir: &str,
    settings: &mut Settings,
) -> Vec<String> {
    // These paths do not need to be converted if we are operating in WSL
    // let source_dir_wsl = cmd::convert_to_wsl_path(source_dir);
    // let build_dir_wsl = cmd::convert_to_wsl_path(build_dir);
    // let source_dir = settings.working_dir.clone();
    // let build_dir = settings.build_dir.clone();

    match cross_compile_target_with_generator {
        "rpi4/umake" => {
            vec![
                "cmake".to_string(),
                // Code source
                "-S".to_string(),
                format!("{}", source_dir),
                // Build destination
                "-B".to_string(),
                format!("{}", build_dir),
                // Generator
                "-G".to_string(),
                "Unix Makefiles".to_string(),
                // Target system type
                "-DCMAKE_SYSTEM_NAME=Linux".to_string(),
                // Target system version
                "-DCMAKE_SYSTEM_VERSION=1".to_string(),
                // Target system processor
                format!(
                    "-DCMAKE_SYSTEM_PROCESSOR={}",
                    settings.cross_compile_processor.to_string()
                ),
                // Target system C compiler
                format!(
                    "-DCMAKE_C_COMPILER=/usr/bin/{}-gnu-gcc",
                    settings.cross_compile_compiler.to_string(),
                ),
                // Target system C++ compiler
                format!(
                    "-DCMAKE_CXX_COMPILER=/usr/bin/{}-gnu-g++",
                    settings.cross_compile_compiler.to_string(),
                ),
                // Target system linker
                format!(
                    "-DCMAKE_SYSROOT={}",
                    settings.cross_compile_sysroot.to_string(),
                ),
                // Target system linker
                format!(
                    "-DCMAKE_FIND_ROOT_PATH={}",
                    settings.cross_compile_sysroot.to_string(),
                ),
                // Target system linker flags
                "-DCMAKE_FIND_ROOT_PATH_MODE_PROGRAM=NEVER".to_string(),
                "-DCMAKE_FIND_ROOT_PATH_MODE_LIBRARY=ONLY".to_string(),
                "-DCMAKE_FIND_ROOT_PATH_MODE_INCLUDE=ONLY".to_string(),
                "-DCMAKE_FIND_ROOT_PATH_MODE_PACKAGE=ONLY".to_string(),
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
#[allow(dead_code)]
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
