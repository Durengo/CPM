use clap::Parser;

pub mod version;
pub mod init;
pub mod setup;

#[derive(Parser)]
pub enum Commands {
    /// Run the 'version' command
    Version(VersionArgs),
    /// Initialize CPM in the current directory
    Init(InitArgs),
    /// Setup CPM in the current directory
    Setup(SetupArgs),
}

#[derive(Parser, Debug)]
pub struct VersionArgs {}

#[derive(Parser, Debug)]
pub struct InitArgs {
    // #[clap(required = true)]
    // pub working_directory: String,

    // #[clap(long, short, action = clap::ArgAction::SetTrue)]
    // pub verbose: bool,

    // #[clap(long, short, default_value = "info")]
    // pub log_level: String,

    // #[clap(long, short, required = true)]
    // pub config: String,
}

#[derive(Parser, Debug)]
pub struct SetupArgs {
    /// Toolchain path. Must be set to root directory of the toolchain.
    #[clap(
        required = false,
        long,
        short,
        value_names = &["TOOLCHAIN-PATH"],
        verbatim_doc_comment
    )]
    pub toolchain: Option<String>,
    /// Generate CMake Project
    /// System types:
    ///     nt/msvc     ---> Windows, MSVC compiler
    ///     unix/clang  ---> Unix, Clang compiler
    ///     unix/gcc    ---> Unix, GCC compiler
    /// Build types:
    ///     Debug       ---> Debug build
    ///     Release     ---> Release build
    #[clap(
        required = false,
        long,
        short,
        num_args(2),
        value_names = &["SYSTEM_TYPE", "BUILD_TYPE"],
        verbatim_doc_comment
    )]
    pub generate_project: Option<Vec<String>>,

    /// Build CMake Project
    /// CMAKE_BUILD_TYPE:
    /// (Mapped by project-generate system type)
    ///     nt/msvc
    ///     unix/clang
    ///     unix/gcc
    #[clap(
        required = false,
        long,
        short,
        value_names = &["CMAKE_BUILD_TYPE"],
        verbatim_doc_comment
    )]
    pub build_project: Option<String>,

    /// Install CMake Project
    /// CMAKE_INSTALL_TYPE:
    /// (Mapped by project-generate build type)
    ///     Debug
    ///     Release
    #[clap(
        required = false,
        long,
        short,
        value_names = &["CMAKE_INSTALL_TYPE"],
        verbatim_doc_comment
    )]
    pub install_project: Option<String>,

    /// Clean CMake Project
    /// WHAT_TO_CLEAN:
    /// (Combine characters to clean multiple things)
    ///     b   ---> Build directory
    ///     i   ---> Install directory
    #[clap(
        required = false,
        long,
        short,
        value_names = &["WHAT_TO_CLEAN"],
        verbatim_doc_comment
    )]
    pub clean_project: Option<String>,
}
