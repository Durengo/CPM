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
    /// Do not run the 'init' command. This is meant for when the entrypoint is created. Otherwise, the 'init' command will be run in commands that require it to be run beforehand.
    #[clap(long, global = true, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub no_init: bool,

    // TEST FLAG. DO NOT USE IN PRODUCTION.
    /// Force settings reinitialization.
    /// WARNING: This will overwrite the current settings file. This will break the current state if already initialized in the current directory.
    #[clap(long, short, global = true, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]
    pub force_reinit: bool,

    // TODO: Find a better logging solution. SPDLOG is not working as expected.
    // // Flag to show debug logs
    // /// Toggle verbose output
    // /// WARNING: This will print a lot of information to the console.
    // #[clap(long, short, global = true, verbatim_doc_comment)]
    // pub verbose: bool,
}

fn main() {
    let _loggers = Loggers::def();

    let cli = Cli::parse();

    // let mut settings = Settings::init(false).unwrap();
    let settings = check_cache(cli.force_reinit).unwrap();

    // if cli.force_reinit {
    //     // Extract the PathBuf if getting the settings path is successful, otherwise handle the error.
    //     let settings_path = Settings::get_settings_path()?;
    //     // Pass a reference to PathBuf (which coerces to &Path)
    //     Settings::delete(&settings_path)?;
    //     settings = Settings::init(true).unwrap();
    // }

    check_supported_os(&settings);
    debug!("Settings:\n{:#?}", settings);

    match cli.command {
        Some(commands::Commands::Init(add_args)) => commands::init::run(add_args, cli.no_init),
        Some(commands::Commands::Setup(add_args)) => commands::setup::run(add_args),
        Some(commands::Commands::Build(add_args)) => {
            if settings.initialized == false {
                RuntimeErrors::NotInitialized.exit();
            } else {
                commands::build::run(add_args);
            }
        }
        Some(commands::Commands::Cache(add_args)) => commands::cache::run(add_args),
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
        "linux" => trace!("Running on Linux"),
        "macos" => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
        "windows" => trace!("Running on Windows"),
        _ => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
    }
}

fn check_cache(reinit: bool) -> std::io::Result<Settings> {
    if reinit {
        let settings_path = Settings::get_settings_path()?;
        // If it does not exist, then create it.
        if settings_path.exists() {
            Settings::delete(&settings_path)?;
            let settings = Settings::init(true)?;
            Ok(settings)
        }
        else {
            let settings = Settings::init(false)?;
            Ok(settings)
        }
    } else {
        let settings = Settings::init(false)?;
        Ok(settings)
    }
}
