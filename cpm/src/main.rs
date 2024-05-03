use clap::Parser;
use spdlog::prelude::*;

use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;
use crate::internal::logger::Loggers;

mod commands;
mod internal;
mod errors;

#[derive(Parser)]
#[clap(author, about, version, long_about)]
struct Cli {
    #[clap(subcommand)]
    command: commands::Commands,

    // Add a flag that if set it will not allow to call the init command. This is meant for when the entrypoint is created.
    /// Do not run the 'init' command
    #[clap(long, global = true, action = clap::ArgAction::SetTrue)]
    pub no_init: bool,
}

fn main() {
    check_supported_os();
    let loggers = Loggers::def();

    let cli = Cli::parse();

    match cli.command {
        commands::Commands::Version(add_args) => commands::version::run(add_args),
        commands::Commands::Init(add_args) => commands::init::run(add_args, cli.no_init),
        // commands::Commands::Init(add_args) => {
        //     if cli.no_init {
        //         RuntimeErrors::NoInitFlagSet.exit();
        //     } else {
        //         commands::init::run(add_args, cli.no_init);
        //     }
        // }
    }
}

fn check_supported_os() {
    let settings = Settings::init(true).unwrap();

    let env = settings.os;

    match env.as_str() {
        "linux" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "macos" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "windows" => info!("Running on Windows"),
        _ => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
    }
}
