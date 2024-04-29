use spdlog::prelude::*;

use crate::commands::InitArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;

pub fn run(args: InitArgs) {
    trace!("Running the Initialization command with arguments: {:?}", args);

    _ = entry();
}

fn entry() -> std::io::Result<()> {
    let mut settings = Settings::load(&Settings::get_settings_path()?)?;

    debug!("Before:\n{:?}", settings);

    settings.working_dir = std::env::current_dir()?.to_str().unwrap().to_string();
    // If working directory is the same as the executable directory, throw an error
    if settings.working_dir == settings.exe_dir {
        RuntimeErrors::WorkingDirSameAsExePath(settings.working_dir.clone(), settings.exe_dir.clone()).exit();
    }

    settings.save(&Settings::get_settings_path()?)?;

    info!("Working directory: {:?}", settings.working_dir);

    os_specific();

    settings.initialized = true;
    settings.save(&Settings::get_settings_path()?)?;

    debug!("After:\n{:?}", settings);

    Ok(())
}

fn os_specific() {
    // Retrieve OS from cache
    debug!("hello");
}
