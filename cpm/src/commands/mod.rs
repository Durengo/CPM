use clap::Parser;

pub mod version;
pub mod init;

#[derive(Parser)]
pub enum Commands {
    Version(VersionArgs),
    Init(InitArgs),
}

#[derive(Parser, Debug)]
pub struct VersionArgs {}

#[derive(Parser, Debug)]
pub struct InitArgs {
    #[clap(long, short, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    #[clap(long, short, default_value = "info")]
    pub log_level: String,

    #[clap(long, short, required = true)]
    pub config: String,

    #[clap(required = true)]
    pub directory: String,
}
