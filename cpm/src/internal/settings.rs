use serde::{ Serialize, Deserialize };
use std::fs::File;
use std::io::{ self, Read, Write };
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub os: String,
    pub exe_path: String,
    pub working_dir: Option<String>,
    pub initialized: bool,
}

impl Settings {
    pub fn new() -> io::Result<Self> {
        let exe_path = std::env
            ::current_exe()?
            .to_str()
            .map(|s| s.to_string());
        let working_dir = std::env
            ::current_dir()?
            .to_str()
            .map(|s| s.to_string());
        Ok(Settings {
            os: std::env::consts::OS.to_string(),
            exe_path: exe_path.unwrap_or_default(),
            working_dir: None,
            initialized: false,
            // working_dir: None,
        })
    }

    pub fn init() -> io::Result<Self> {
        let exe_path_buf = std::env::current_exe()?;

        let exe_dir = exe_path_buf
            .parent()
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "Executable directory not found"))?;

        let settings_path = exe_dir.join("settings.json");

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
