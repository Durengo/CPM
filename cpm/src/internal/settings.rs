use serde::{ Serialize, Deserialize };
use std::fs::{self, File};
use std::io::{ self, Read, Write };
use std::path::Path;
use std::path::PathBuf;
use spdlog::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub os: String,
    pub exe_path: String,
    pub exe_dir: String,
    pub working_dir: String,
    pub initialized: bool,
}

impl Settings {
    pub fn new() -> io::Result<Self> {
        // Get the full executable path
        let exe_path_buf = std::env::current_exe()?;
        let exe_path = exe_path_buf.to_str().map(|s| s.to_string()).unwrap_or_default();

        // Get the executable directory by finding the parent of the executable path
        let exe_dir = exe_path_buf.parent()
            .and_then(|path| path.to_str())
            .map(|s| s.to_string())
            .unwrap_or_default();

        let working_dir = std::env
            ::current_dir()?
            .to_str()
            .map(|s| s.to_string());
        Ok(Settings {
            os: std::env::consts::OS.to_string(),
            exe_path: exe_path,
            exe_dir: exe_dir,
            working_dir: "".to_string(),
            initialized: false,
            // working_dir: None,
        })
    }

    pub fn init(force_rewrite: bool) -> io::Result<Self> {
        let settings_path = Self::get_settings_path()?;

        if force_rewrite && settings_path.exists() {
            fs::remove_file(&settings_path)?;
        }

        Self::load_or_init(&settings_path)
    }

    pub fn load_or_init(path: &Path) -> io::Result<Self> {
        if path.exists() {
            Self::load(path)
        } else {
            let settings = Self::new()?;
            settings.save(path)?;
            Ok(settings)
        }
    }

    pub fn load(path: &Path) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        let contents = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(path)?;
        file.write_all(contents.as_bytes())
    }

    pub fn get_settings_path() -> io::Result<PathBuf> {
        let exe_path = std::env::current_exe()?;
        let dir = exe_path
            .parent()
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "Executable directory not found"))?;
        Ok(dir.join("settings.json"))
    }
}
