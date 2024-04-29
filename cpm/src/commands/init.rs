use spdlog::prelude::*;

use crate::commands::InitArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::{ self, Settings };

pub fn run(args: InitArgs) {
    trace!("Running the Initialization command with arguments: {:?}", args);

    _ = entry();
}

fn entry() -> std::io::Result<()> {
    let mut settings = Settings::load(&Settings::get_settings_path()?)?;

    settings.working_dir = Some(std::env::current_dir()?.to_str().unwrap().to_string());
    settings.save(&Settings::get_settings_path()?)?;

    info!("Working directory: {:?}", settings.working_dir);

    os_specific();

    Ok(())
}

fn os_specific() {
    // Retrieve OS from cache
    debug!("hello");
}
