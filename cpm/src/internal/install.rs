use rust_embed::RustEmbed;
use serde::{ Deserialize, Serialize };

#[derive(RustEmbed)]
#[folder = "presets/"]
pub struct Presets;
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub os_target: String,
    pub config: MultiOSConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiOSConfig {
    pub windows: Option<WindowsConfig>,
    pub linux: Option<LinuxConfig>,
    pub macos: Option<MacOSConfig>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsConfig {
    pub prerequisites: Vec<String>,
    pub toolchain: String,
    pub packages: Vec<Package>,
    pub post_install: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxConfig {
    pub dependencies: Vec<String>,
    pub toolchain: String,
    pub instructions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MacOSConfig {
    pub tools: Vec<String>,
    pub toolchain: String,
    pub setup_steps: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub library: String,
    pub triplet: String,
}
