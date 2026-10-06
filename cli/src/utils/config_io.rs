use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::Path;

/// Outcome of trying to read a config file off disk.
///
/// The distinction matters because the caller writes the config back out
/// afterwards: a file we failed to parse must never be overwritten, or a
/// single typo (or a field whose type we got wrong) silently destroys the
/// operator's hand-written server settings. That is exactly what issues #26,
/// #31, #32 and #33 all describe.
pub enum Loaded<T> {
    /// No config file on disk yet; `T` is the default config.
    Missing(T),
    /// The file parsed cleanly. Carries the raw JSON so unknown-to-us keys can
    /// be reasoned about by the caller if needed.
    Parsed(T),
    /// The file exists but could not be read or parsed. `T` is the default
    /// config, usable in-memory, but the file itself must be left alone.
    Unreadable(T),
}

impl<T> Loaded<T> {
    /// The config to work with, regardless of where it came from.
    pub fn into_config(self) -> T {
        match self {
            Self::Missing(config) | Self::Parsed(config) | Self::Unreadable(config) => config,
        }
    }

    /// Whether it is safe to write the config back to this path.
    pub const fn is_writable(&self) -> bool {
        !matches!(self, Self::Unreadable(_))
    }
}

pub fn load_config<T>(path: &Path) -> Loaded<T>
where
    T: DeserializeOwned + Default,
{
    if !path.exists() {
        tracing::debug!("Config file does not exist at: {:?}, using defaults", path);
        return Loaded::Missing(T::default());
    }

    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(e) => {
            tracing::error!(
                "Failed to read {:?} ({e}). Leaving the file untouched and running with defaults.",
                path
            );
            return Loaded::Unreadable(T::default());
        }
    };

    match serde_json::from_str::<T>(&contents) {
        Ok(config) => {
            tracing::debug!("Successfully parsed config from {:?}", path);
            Loaded::Parsed(config)
        }
        Err(e) => {
            tracing::error!(
                "Failed to parse {:?} ({e}). Your settings are NOT being changed -- fix the \
                 JSON (or delete the file to regenerate it) and restart. Running with defaults \
                 for this session.",
                path
            );
            Loaded::Unreadable(T::default())
        }
    }
}

pub fn save_config<T: Serialize>(path: &Path, config: &T) {
    match serde_json::to_string_pretty(config) {
        Ok(json) => {
            if let Err(e) = fs::write(path, json) {
                tracing::error!("Failed to write config to {:?}: {e}", path);
            }
        }
        Err(e) => tracing::error!("Failed to serialize config for {:?}: {e}", path),
    }
}
