use clap::Parser;

pub mod version;
pub mod init;

#[derive(Parser)]
pub enum Commands {
    /// Run the 'version' command
    Version(VersionArgs),
    /// Initialize CPM in the current directory
    Init(InitArgs),
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
