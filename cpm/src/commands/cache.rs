use spdlog::prelude::*;

use crate::commands::CacheArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;
use crate::internal::cmd;

pub fn run(args: CacheArgs) {
    debug!("Running the Initialization command with arguments: {:#?}", args);

    // Grab the settings file as it will be needed for the subcommands.
    let settings_path = match Settings::get_settings_path() {
        Ok(path) => path,
        Err(e) => {
            error!("Failed to get settings path: {}", e);
            return;
        }
    };

    let mut settings = match Settings::load(&settings_path) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to load settings: {}", e);
            return;
        }
    };

    // Print has optional argument to print key/value pairs of the cache
    if let Some(printing) = &args.print_cache {
        match printing {
            Some(key) => {
                if settings.contains_key(key) {
                    info!("{}: {}", key, settings.get_value(key).unwrap_or("None".to_string()));
                } else {
                    error!("Key '{}' not found in cache", key);
                }
            }
            None => {
                print_cache(&settings);
            }
        }
    } else if let Some(cache_key) = &args.edit_cache_key {
        if cache_key.len() == 2 {
            info!("Changing value of key '{}' to '{}'", cache_key[0], cache_key[1]);
            let _ = settings.set_value(&cache_key[0], cache_key[1].clone());
        }
    } else if args.open_cache {
        open_cache_in_explorer(&settings);
    } else {
        print_cache(&settings);
    }
}

fn print_cache(settings: &Settings) {
    match serde_json::to_string_pretty(&settings) {
        Ok(json) => {
            info!("Current Settings:\n{}", json);
        }
        Err(e) => {
            error!("Error serializing settings to JSON: {}", e);
        }
    }
}

fn open_cache_in_explorer(settings: &Settings) {
    let env = &settings.os;
    let cache_path = Settings::get_settings_path().unwrap();

    match env.as_str() {
        "linux" => {
            RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit();
            cmd::execute(vec!["xdg-open".to_string(), cache_path.to_str().unwrap().to_string()]);
        }
        "macos" => {
            RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit();
            cmd::execute(vec!["open".to_string(), cache_path.to_str().unwrap().to_string()]);
        }
        "windows" => {
            debug!("Opening cache in explorer");
            cmd::execute(vec!["explorer".to_string(), cache_path.to_str().unwrap().to_string()]);
        }
        _ => RuntimeErrors::NotSupportedOS(Some(env.to_string())).exit(),
    }
}
