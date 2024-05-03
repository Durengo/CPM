use serde::{ Serialize, Deserialize };
use std::fs::{ self, File };
use std::io::{ self, Read, Write };
use std::path::Path;
use std::path::PathBuf;
use spdlog::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    // Basic information
    pub os: String,
    pub os_release: String,
    pub os_version: String,
    pub exe_path: String,
    pub exe_dir: String,
    pub working_dir: String,
    pub initialized: bool,
    pub install_json_path: String,
    // Required for building project
    pub build_dir: String,
    pub install_dir: String,
    pub toolchain_path: String,
    // WINDOWS ONLY - VCPKG CMAKE TOOLCHAIN
    // <toolchain_path>/scripts/buildsystems/vcpkg.cmake
    pub vcpkg_path: String,
    pub cmake_system_type: String,
    pub cmake_build_type: String,
    // Cached commands
    pub last_cmake_configuration_command: Vec<String>,
}

impl Settings {
    pub fn new() -> io::Result<Self> {
        // Get the full executable path
        let exe_path_buf = std::env::current_exe()?;
        let exe_path = exe_path_buf
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or_default();

        // Get the executable directory by finding the parent of the executable path
        let exe_dir = exe_path_buf
            .parent()
            .and_then(|path| path.to_str())
            .map(|s| s.to_string())
            .unwrap_or_default();

        let working_dir = std::env
            ::current_dir()?
            .to_str()
            .map(|s| s.to_string());
        Ok(Settings {
            os: std::env::consts::OS.to_string(),
            os_release: sys_info::os_type().unwrap_or_default(),
            os_version: sys_info::os_release().unwrap_or_default(),
            exe_path: exe_path,
            exe_dir: exe_dir,
            working_dir: "".to_string(),
            initialized: false,
            install_json_path: "".to_string(),
            // Required for building project
            build_dir: "".to_string(),
            install_dir: "".to_string(),
            toolchain_path: "".to_string(),
            // WINDOWS ONLY - VCPKG CMAKE TOOLCHAIN
            // <toolchain_path>/scripts/buildsystems/vcpkg.cmake
            vcpkg_path: "".to_string(),
            cmake_system_type: "".to_string(),
            cmake_build_type: "".to_string(),
            // Cached commands
            last_cmake_configuration_command: vec![],
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

    // Saves the settings to the default path '<exe_dir>/settings.json'
    pub fn save_default(&self) -> io::Result<()> {
        let path = Self::get_settings_path()?;
        self.save(&path)
    }

    pub fn get_settings_path() -> io::Result<PathBuf> {
        let exe_path = std::env::current_exe()?;
        let dir = exe_path
            .parent()
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "Executable directory not found"))?;
        Ok(dir.join("settings.json"))
    }
}
