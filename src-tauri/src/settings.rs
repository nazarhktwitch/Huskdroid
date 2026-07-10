use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default)]
    pub custom_qemu_path: Option<String>,
    #[serde(default)]
    pub custom_adb_path: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            custom_qemu_path: None,
            custom_adb_path: None,
        }
    }
}

pub fn get_settings_path() -> Result<PathBuf> {
    let dir = dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("com.huskdroid.dev");
    fs::create_dir_all(&dir)?;
    Ok(dir.join("settings.json"))
}

pub fn load_settings() -> Result<AppSettings> {
    let path = get_settings_path()?;
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let data = fs::read_to_string(path)?;
    let settings = serde_json::from_str(&data).unwrap_or_default();
    Ok(settings)
}

pub fn save_settings(settings: &AppSettings) -> Result<()> {
    let path = get_settings_path()?;
    let data = serde_json::to_string_pretty(settings)?;
    fs::write(path, data)?;
    Ok(())
}
