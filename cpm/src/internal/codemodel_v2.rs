// use serde_json::Value;
use serde::{ Deserialize, Serialize };
use spdlog::prelude::*;
use std::fs;
use std::path::{ Path, PathBuf };
use std::io::{ self, ErrorKind };

use crate::internal::settings::Settings;

#[derive(Serialize, Deserialize, Debug)]
pub struct CMakeAPIResponse {
    pub configurations: Vec<Configuration>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub name: String,
    pub targets: Vec<Target>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Target {
    pub name: String,
}

pub fn generate_cmake_codemodel_v2(settings: &Settings) {
    // Very simple. We need to generate 'codemodel-v2' empty file in <build_dir>/.cmake/api/v1/query

    let build_dir = settings.build_dir.clone();
    let codemodel_v2_path = Path::new(&build_dir).join(".cmake/api/v1/query/codemodel-v2");

    match fs::create_dir_all(&codemodel_v2_path) {
        Ok(_) => {
            info!("Successfully created {}.", codemodel_v2_path.display());
        }
        Err(e) => {
            error!("Error creating 'codemodel-v2' directory: {}", e);
        }
    }
}

pub fn find_codemodel_file(path: &Path) -> io::Result<PathBuf> {
    let entries = fs
        ::read_dir(path)?
        .filter_map(Result::ok)
        .find(|entry| entry.file_name().to_string_lossy().starts_with("codemodel-v2-"))
        .map(|entry| entry.path());

    match entries {
        Some(path) => Ok(path),
        None => Err(io::Error::new(ErrorKind::NotFound, "No codemodel-v2-*.json file found")),
    }
}
