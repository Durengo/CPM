use serde::{ Serialize, Deserialize };
use std::fs::File;
use std::io::{ self, Read, Write };
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub os: String,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            os: std::env::consts::OS.to_string(),
        }
    }

    pub fn init(path: &str) -> io::Result<Self> {
        let path = Path::new(path);
        Self::load_or_init(path)
    }

    pub fn load_or_init(path: &Path) -> io::Result<Self> {
        if path.exists() {
            Self::load(path)
        } else {
            let settings = Self::new();
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
}
