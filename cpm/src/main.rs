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
}

fn main() {
    let settings = Settings::init().unwrap();

    let loggers = Loggers::def();

    // TODO: Some logic to cache OS to avoid calling it multiple times
    let env = settings.os;

    match env.as_str() {
        "linux" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "macos" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "windows" => info!("Running on Windows"),
        _ => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
    }

    let cli = Cli::parse();

    match cli.command {
        commands::Commands::Version(add_args) => commands::version::run(add_args),
        commands::Commands::Init(add_args) => commands::init::run(add_args),
    }
}
