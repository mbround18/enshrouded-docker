use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::Path;

pub fn load_config_with_defaults<T>(path: &Path) -> T
where
    T: DeserializeOwned + Default,
{
    if path.exists() {
        tracing::debug!("Config file exists at: {:?}", path);
        match fs::read_to_string(path) {
            Ok(contents) => {
                tracing::debug!("Successfully read config file contents");
                match serde_json::from_str::<T>(&contents) {
                    Ok(config) => {
                        tracing::debug!("Successfully parsed config from file");
                        config
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse config file, using defaults. Error: {}", e);
                        T::default()
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to read config file, using defaults. Error: {}", e);
                T::default()
            }
        }
    } else {
        tracing::debug!("Config file does not exist at: {:?}, using defaults", path);
        T::default()
    }
}

pub fn save_config<T: Serialize>(path: &Path, config: &T) {
    if let Ok(json) = serde_json::to_string_pretty(config) {
        let _ = fs::write(path, json);
    }
}
