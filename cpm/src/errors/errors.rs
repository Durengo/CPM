use std::process;
use spdlog::prelude::*;

pub enum RuntimeErrors {
    // OS related errors 1-9
    NotSupportedOS(Option<String>),
    WorkingDirSameAsExePath(String, String),
    CmdCaughtStdErr(Option<String>),
    // JSON file related errors 10-10
    JSONFileNotFound(Option<String>),
    ConfigParseError(Option<String>),
    // Logic related errors 21-30
    NoInitFlagSet,
    NotInitialized,
    NoCommandsProvided,
    // Setup Command related errors 31-40
    PrerequisiteNotFound(Option<String>),
    PackageInstallFailed(Option<String>),
    PostInstallFailed(Option<String>),
    PostInstallNoDefinition(Option<String>),
    // Build Command related errors 41-50
    GenerateProjectInvalidSystemType(Option<String>),
    GenerateProjectNtMsvcNoToolchain,
    ToolchainNotFound(String),
    BuildTypeNotSet,
    BuildTypeBothSet,
    InvalidCleanCommand(char),
    ProjectNotInitialized,
    CMakeProjectNotGenerated,
    // Not implemented 1000-1005
    NotImplemented,
}

impl RuntimeErrors {
    pub fn error_code(&self) -> i32 {
        match *self {
            // OS related errors 1-9
            RuntimeErrors::NotSupportedOS(_) => 1,
            RuntimeErrors::WorkingDirSameAsExePath(_, _) => 2,
            RuntimeErrors::CmdCaughtStdErr(_) => 3,
            // JSON file related errors 10-20
            RuntimeErrors::JSONFileNotFound(_) => 2,
            RuntimeErrors::ConfigParseError(_) => 3,
            // Logic related errors 21-30
            RuntimeErrors::NoInitFlagSet => 21,
            RuntimeErrors::NotInitialized => 22,
            RuntimeErrors::NoCommandsProvided => 23,
            // Setup Command related errors 31-40
            RuntimeErrors::PrerequisiteNotFound(_) => 31,
            RuntimeErrors::PackageInstallFailed(_) => 32,
            RuntimeErrors::PostInstallFailed(_) => 33,
            RuntimeErrors::PostInstallNoDefinition(_) => 34,
            // Build Command related errors 31-40
            RuntimeErrors::GenerateProjectInvalidSystemType(_) => 41,
            RuntimeErrors::GenerateProjectNtMsvcNoToolchain => 42,
            RuntimeErrors::ToolchainNotFound(_) => 43,
            RuntimeErrors::BuildTypeNotSet => 44,
            RuntimeErrors::BuildTypeBothSet => 45,
            RuntimeErrors::InvalidCleanCommand(_) => 46,
            RuntimeErrors::ProjectNotInitialized => 47,
            RuntimeErrors::CMakeProjectNotGenerated => 48,
            // Not implemented 1000-1005
            RuntimeErrors::NotImplemented => 1000,
        }
    }

    // Errors should also display the error code
    pub fn error_message(&self) -> String {
        match self {
            // OS related errors 1-9
            RuntimeErrors::NotSupportedOS(Some(message)) => {
                format!("|Error {}| The OS is not supported: {}", self.error_code(), message)
            }
            RuntimeErrors::NotSupportedOS(None) => {
                format!("|Error {}| The OS is not supported", self.error_code())
            }
            RuntimeErrors::WorkingDirSameAsExePath(working_dir, exe_path) => {
                format!(
                    "|Error {}| The working directory is the same as the executable directory: {} == {}",
                    self.error_code(),
                    working_dir,
                    exe_path
                )
            }
            RuntimeErrors::CmdCaughtStdErr(Some(message)) => {
                format!("|Error {}| Command caught stderr: {}", self.error_code(), message)
            }
            RuntimeErrors::CmdCaughtStdErr(None) => {
                format!("|Error {}| Command caught stderr", self.error_code())
            }
            // JSON file related errors 10-10
            RuntimeErrors::JSONFileNotFound(Some(message)) => {
                format!("|Error {}| The JSON file was not found: {}", self.error_code(), message)
            }
            RuntimeErrors::JSONFileNotFound(None) => {
                format!("|Error {}| The JSON file was not found", self.error_code())
            }
            RuntimeErrors::ConfigParseError(Some(message)) => {
                format!("|Error {}| Error parsing the config file: {}", self.error_code(), message)
            }
            RuntimeErrors::ConfigParseError(None) => {
                format!("|Error {}| Error parsing the config file", self.error_code())
            }
            // Logic related errors 21-30
            RuntimeErrors::NoInitFlagSet => {
                format!(
                    "|Error {}| The no-init flag was set. Do not run 'init' from entrypoint",
                    self.error_code()
                )
            }
            RuntimeErrors::NotInitialized => {
                format!(
                    "|Error {}| Project not initialized, run 'init' command first",
                    self.error_code()
                )
            }
            RuntimeErrors::NoCommandsProvided => {
                format!(
                    "|Error {}| No commands provided. Run 'cpm --help' for more information",
                    self.error_code()
                )
            }
            // Setup Command related errors 31-40
            RuntimeErrors::PrerequisiteNotFound(Some(prerequisite)) => {
                format!("|Error {}| Prerequisite '{}' not found", self.error_code(), prerequisite)
            }
            RuntimeErrors::PrerequisiteNotFound(None) => {
                format!("|Error {}| Prerequisite not found", self.error_code())
            }
            RuntimeErrors::PackageInstallFailed(Some(package)) => {
                format!("|Error {}| Failed to install package '{}'", self.error_code(), package)
            }
            RuntimeErrors::PackageInstallFailed(None) => {
                format!("|Error {}| Failed to install package", self.error_code())
            }
            RuntimeErrors::PostInstallFailed(Some(post_install)) => {
                format!("|Error {}| Post install failed: {}", self.error_code(), post_install)
            }
            RuntimeErrors::PostInstallFailed(None) => {
                format!("|Error {}| Post install failed", self.error_code())
            }
            RuntimeErrors::PostInstallNoDefinition(Some(post_install)) => {
                format!(
                    "|Error {}| Post install '{}' has no definition",
                    self.error_code(),
                    post_install
                )
            }
            RuntimeErrors::PostInstallNoDefinition(None) => {
                format!("|Error {}| Post install has no definition", self.error_code())
            }
            // Build Command related errors 31-40
            RuntimeErrors::GenerateProjectInvalidSystemType(Some(system_type)) => {
                format!(
                    "|Error {}| The system type '{}' is invalid",
                    self.error_code(),
                    system_type
                )
            }
            RuntimeErrors::GenerateProjectInvalidSystemType(None) => {
                format!("|Error {}| The system type is invalid", self.error_code())
            }
            RuntimeErrors::GenerateProjectNtMsvcNoToolchain => {
                format!(
                    "|Error {}| The system type 'nt/msvc' requires a toolchain path",
                    self.error_code()
                )
            }
            RuntimeErrors::ToolchainNotFound(toolchain) => {
                format!("|Error {}| Toolchain '{}' not found", self.error_code(), toolchain)
            }
            RuntimeErrors::BuildTypeNotSet => {
                format!("|Error {}| The build type was not set", self.error_code())
            }
            RuntimeErrors::BuildTypeBothSet => {
                format!("|Error {}| Both debug and release build types set", self.error_code())
            }
            RuntimeErrors::InvalidCleanCommand(command) => {
                format!("|Error {}| Invalid clean command: {}", self.error_code(), command)
            }
            RuntimeErrors::ProjectNotInitialized => {
                format!(
                    "|Error {}| Project not initialized, run 'init' command first",
                    self.error_code()
                )
            }
            RuntimeErrors::CMakeProjectNotGenerated => {
                format!("|Error {}| CMake project not generated", self.error_code())
            }
            // Not implemented 1000-1005
            RuntimeErrors::NotImplemented => {
                format!("|Error {}| This feature is not implemented", self.error_code())
            }
        }
    }

    pub fn exit(&self) {
        error!("{}", self.error_message());
        process::exit(self.error_code());
    }
}
