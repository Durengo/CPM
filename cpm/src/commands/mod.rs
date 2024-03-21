use clap::Parser;

pub mod version;

#[derive(Parser)]
pub enum Commands {
    Version(VersionArgs), // Add this variant
    // ... more subcommands
}

#[derive(Parser, Debug)]
pub struct VersionArgs {
    // Add arguments for the 'version' subcommand
}
