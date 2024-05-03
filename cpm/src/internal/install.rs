use rust_embed::RustEmbed;
use serde::{ Deserialize, Serialize };

#[derive(RustEmbed)]
#[folder = "presets/"]
pub struct Presets;
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    os_target: String,
    config: MultiOSConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiOSConfig {
    windows: Option<WindowsConfig>,
    linux: Option<LinuxConfig>,
    macos: Option<MacOSConfig>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsConfig {
    prerequisites: Vec<String>,
    packages: Vec<Package>,
    post_install: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxConfig {
    dependencies: Vec<String>,
    instructions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MacOSConfig {
    tools: Vec<String>,
    setup_steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    library: String,
    triplet: String,
}
