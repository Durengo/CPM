use clap::{ CommandFactory, Parser };
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
    command: Option<commands::Commands>,

    // Add a flag that if set it will not allow to call the init command. This is meant for when the entrypoint is created.
    /// Do not run the 'init' command
    #[clap(long, global = true, action = clap::ArgAction::SetTrue)]
    pub no_init: bool,

    // TEST FLAG. DO NOT USE IN PRODUCTION.
    // Force settings reinitialization
    /// Force settings reinitialization
    #[clap(long, short, global = true, action = clap::ArgAction::SetTrue)]
    pub force_reinit: bool,
}

fn main() {
    let loggers = Loggers::def();

    let cli = Cli::parse();

    let mut settings = Settings::init(false).unwrap();

    if cli.force_reinit {
        settings = Settings::init(true).unwrap();
    }

    check_supported_os(&settings);
    debug!("Settings: {:?}", settings);

    match cli.command {
        Some(commands::Commands::Version(add_args)) => commands::version::run(add_args),
        Some(commands::Commands::Init(add_args)) => commands::init::run(add_args, cli.no_init),
        Some(commands::Commands::Setup(add_args)) => {
            if settings.initialized == false {
                RuntimeErrors::NotInitialized.exit();
            } else {
                commands::setup::run(add_args);
            }
        }
        None => {
            if let Err(e) = Cli::command().print_help() {
                eprintln!("Failed to print help information: {}", e);
            }
            // Since on the entrypoint we will always pass the no-init flag, we will not get the default help message. So this is not an error if we reach this point.
            std::process::exit(0);
        }
    }
}

fn check_supported_os(settings: &Settings) {
    let env = &settings.os;

    match env.as_str() {
        "linux" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "macos" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "windows" => info!("Running on Windows"),
        _ => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
    }
}
