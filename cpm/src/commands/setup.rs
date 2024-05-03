use serde::de;
use spdlog::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::commands::SetupArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::Settings;
use crate::internal::install::{ Presets, Config };

pub fn run(args: SetupArgs) {
    debug!("Running the Initialization command with arguments: {:?}", args);

    _ = entry();
}

fn entry() -> std::io::Result<()> {
    let mut settings = Settings::load(&Settings::get_settings_path()?)?;

    info!("Working directory: {:?}", settings.working_dir);

    // os_specific(&mut settings);

    Ok(())
}