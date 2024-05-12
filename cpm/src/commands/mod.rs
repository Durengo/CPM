use clap::Parser;

pub mod init;
pub mod build;
pub mod cache;
pub mod setup;

#[derive(Parser)]
pub enum Commands {
    /// Initialize CPM in the current directory
    Init(InitArgs),
    /// Setup CPM in the current directory
    Setup(SetupArgs),
    /// Build CPM in the current directory
    Build(BuildArgs),
    /// Manage CPM Cache
    Cache(CacheArgs),
}

#[derive(Parser, Debug)]
pub struct InitArgs {}

#[derive(Parser, Debug)]
pub struct SetupArgs {
    /// Cache the toolchain path. Does not run the setup.
    #[clap(required = false, long, short, value_names = &["TOOLCHAIN-PATH"], verbatim_doc_comment)]
    pub toolchain: Option<String>,
    /// Tries to find an existing toolchain installation if it is added to the path. Runs the setup.
    /// Windows: uses where.exe to find toolchain.
    #[clap(required = false, long, short, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub auto_toolchain_path: bool,
    /// Tries to find an existing toolchain installation if it is added to the path, otherwise attempts to download and setup the toolchain. Runs the setup.
    /// Windows: uses where.exe to find toolchain.
    #[clap(required = false, long, short, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub no_toolchain_path: bool,
    /// Runs the setup with the specified vcpkg directory. Must be set to root directory of the toolchain. Runs the setup.
    #[clap(required = false, long, short, value_names = &["TOOLCHAIN-PATH"], verbatim_doc_comment)]
    pub use_toolchain_path: Option<String>,
    /// Skips package configuration when running 'auto_toolchain_path', 'no_toolchain_path', or 'toolchain_path'. Pass before running 'auto_toolchain_path', 'no_toolchain_path', or 'toolchain_path'.
    #[clap(required = false, long = "spc", action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub skip_package_configurations: bool,
    /// Runs the setup without checking for runtime dependencies (ONLY FOR CI USE). Pass before running 'auto_toolchain_path', 'no_toolchain_path', or 'toolchain_path'.
    #[clap(required = false, long = "ndc", action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub no_deps_check: bool,
    /// Forces vcpkg to install packages again (does not remove any existing packages). Pass before running 'auto_toolchain_path', 'no_toolchain_path', or 'toolchain_path'.
    #[clap(required = false, long = "fpi", action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub force_package_install: bool,

    /// Forces to use specific OS install.
    /// Supported OS types:
    ///     windows
    ///     linux
    ///     macos
    #[clap(
        required = false,
        long,
        short,
        num_args(1),
        value_names = &["TOOLCHAIN-PATH"],
        verbatim_doc_comment
    )]
    pub platform: Option<String>,
}

#[derive(Parser, Debug)]
pub struct BuildArgs {
    /// Sets Build Type to Debug.
    /// Must be set to either Debug or Release. Required shorthand for other commands.
    /// Build types:
    /// (Must mirror project-generate build types)
    ///     Debug       ---> Debug build
    ///     Release     ---> Release build
    #[clap(required = false, long, short, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub debug_build_type: bool,

    /// Sets Build Type to Release.
    /// Must be set to either Debug or Release. Required shorthand for other commands.
    /// Build types:
    /// (Must mirror project-generate build types)
    ///     Debug       ---> Debug build
    ///     Release     ---> Release build
    #[clap(required = false, long, short, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub release_build_type: bool,

    /// Generate CMake Project. Will not run without a build type set flag.
    /// System types:
    ///     nt/msvc     ---> Windows, MSVC compiler
    ///     unix/clang  ---> Unix, Clang compiler
    ///     unix/gcc    ---> Unix, GCC compiler
    ///     make/clang  ---> Unix, Clang compiler, Make build system
    ///     make/gcc    ---> Unix, GCC compiler, Make build system
    ///     NONE        ---> No system type. Uses last cached generate command.
    #[clap(
        required = false,
        long,
        short,
        value_names = &["SYSTEM_TYPE"],
        action = clap::ArgAction::Set,
        verbatim_doc_comment
    )]
    pub generate_project: Option<Option<String>>,

    /// Build CMake Project. Automatically uses CMAKE_BUILD_TYPE. Will not run without a build type set flag.
    #[clap(required = false, long, short, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub build_project: bool,

    /// Install CMake Project. Automatically uses CMAKE_BUILD_TYPE. Will not run without a build type set flag.
    #[clap(required = false, long, short, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub install_project: bool,

    /// Clean CMake Project
    /// WHAT_TO_CLEAN:
    /// (Combine characters to clean multiple things)
    ///     b   ---> Build directory
    ///     i   ---> Install directory
    #[clap(
        required = false,
        long,
        short,
        action = clap::ArgAction::Set,
        value_names = &["WHAT_TO_CLEAN"],
        verbatim_doc_comment
    )]
    pub clean_project: Option<Option<String>>,

    /// Source targets from CMake. Project must be generated first, otherwise it will fail.
    #[clap(required = false, long, short, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub source_targets: bool,
}

#[derive(Parser, Debug)]
pub struct CacheArgs {
    /// Print the cache
    /// If a key is provided, only that key will be printed
    /// If no key is provided, all keys will be printed
    #[clap(
        required = false,
        long,
        short,
        action = clap::ArgAction::Set,
        value_names = &["KEY"],
        verbatim_doc_comment,
        value_parser
    )]
    pub print_cache: Option<Option<String>>,
    /// Change a cache value
    #[clap(
        required = false,
        long,
        short,
        num_args(2),
        value_names = &["KEY", "VALUE"],
        verbatim_doc_comment
    )]
    pub edit_cache_key: Option<Vec<String>>,
    /// Open the cache file in the default editor
    #[clap(required = false, long, short, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    // No value needed
    pub open_cache: bool,

    /// Disable all logging and return only the value. Useful for scripts. Provide this as a flag before using 'print_cache'
    #[clap(required = false, long, short, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub raw_return: bool,
}
