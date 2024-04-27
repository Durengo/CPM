use std::os;

use crate::commands::InitArgs;
use crate::errors::errors::RuntimeErrors;
use crate::internal::settings::{ self, Settings };

pub fn run(args: InitArgs) {
    println!("Running the Initialization command with arguments: {:?}", args);

    let _ = main();
}

fn main() -> std::io::Result<()> {
    let settings = Settings::load(&Settings::get_settings_path()?)?;
    os_specific();

    Ok(())
}

fn os_specific() {
    // Retrieve OS from cache
}
