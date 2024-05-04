use serde::{ Serialize, Deserialize };
use std::fs::{ self, File };
use std::io::{ self, Read, Write };
use std::path::Path;
use std::path::PathBuf;

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
    pub using_toolchain: bool,
    pub toolchain_path: String,
    // WINDOWS ONLY - VCPKG CMAKE TOOLCHAIN
    // <toolchain_path>/scripts/buildsystems/vcpkg.cmake
    pub vcpkg_path: String,
    // WINDOWS ONLY - VCPKG CMAKE TOOLCHAIN
    pub cmake_system_type: String,
    pub cmake_build_type: String,
    // Cached commands
    pub last_cmake_configuration_command: Vec<String>,
    pub last_command: Vec<String>,
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
            using_toolchain: false,
            toolchain_path: "".to_string(),
            // WINDOWS ONLY - VCPKG CMAKE TOOLCHAIN
            // <toolchain_path>/scripts/buildsystems/vcpkg.cmake
            vcpkg_path: "".to_string(),
            // WINDOWS ONLY - VCPKG CMAKE TOOLCHAIN
            cmake_system_type: "".to_string(),
            cmake_build_type: "".to_string(),
            // Cached commands
            last_cmake_configuration_command: vec![],
            last_command: vec![],
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

    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "os" => Some(self.os.clone()),
            "os_release" => Some(self.os_release.clone()),
            "os_version" => Some(self.os_version.clone()),
            "exe_path" => Some(self.exe_path.clone()),
            "exe_dir" => Some(self.exe_dir.clone()),
            "working_dir" => Some(self.working_dir.clone()),
            "install_json_path" => Some(self.install_json_path.clone()),
            "build_dir" => Some(self.build_dir.clone()),
            "install_dir" => Some(self.install_dir.clone()),
            "using_toolchain" => Some(self.using_toolchain.to_string()),
            "toolchain_path" => Some(self.toolchain_path.clone()),
            "vcpkg_path" => Some(self.vcpkg_path.clone()),
            "cmake_system_type" => Some(self.cmake_system_type.clone()),
            "cmake_build_type" => Some(self.cmake_build_type.clone()),
            // Cached commands are locked
            _ => None,
        }
    }

    pub fn set_value(&mut self, key: &str, value: String) -> Result<(), String> {
        match key {
            "os" => {
                self.os = value;
            }
            "os_release" => {
                self.os_release = value;
            }
            "os_version" => {
                self.os_version = value;
            }
            "exe_path" => {
                self.exe_path = value;
            }
            "exe_dir" => {
                self.exe_dir = value;
            }
            "working_dir" => {
                self.working_dir = value;
            }
            "install_json_path" => {
                self.install_json_path = value;
            }
            "build_dir" => {
                self.build_dir = value;
            }
            "install_dir" => {
                self.install_dir = value;
            }
            "using_toolchain" => {
                self.using_toolchain = value.parse().unwrap_or(false);
            }
            "toolchain_path" => {
                self.toolchain_path = value;
            }
            "vcpkg_path" => {
                self.vcpkg_path = value;
            }
            "cmake_system_type" => {
                self.cmake_system_type = value;
            }
            "cmake_build_type" => {
                self.cmake_build_type = value;
            }
            // Cached commands are locked
            _ => {
                return Err("Key not found".to_string());
            }
        }

        self.save_default();
        Ok(())
    }

    pub fn contains_key(&self, key: &str) -> bool {
        match key {
            | "os"
            | "os_release"
            | "os_version"
            | "exe_path"
            | "exe_dir"
            | "working_dir"
            | "install_json_path"
            | "build_dir"
            | "install_dir"
            | "using_toolchain"
            | "toolchain_path"
            | "vcpkg_path"
            | "cmake_system_type"
            | "cmake_build_type"
            | "last_cmake_configuration_command"
            | "last_command" => true,
            _ => false,
        }
    }
}
