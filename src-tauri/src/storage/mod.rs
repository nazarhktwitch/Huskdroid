use anyhow::Result;
use std::path::PathBuf;

use crate::devices::DeviceConfig;

fn data_dir() -> PathBuf {
    // tauri::api::path is not available here (no app handle), so use dirs crate pattern
    // In practice this is called from Tauri commands that pass the app data dir
    dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("huskdroid")
}

fn devices_path() -> PathBuf {
    data_dir().join("devices.json")
}

pub fn load_devices() -> Result<Vec<DeviceConfig>> {
    let path = devices_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let bytes = std::fs::read(&path)?;
    let devices = serde_json::from_slice(&bytes)?;
    Ok(devices)
}

pub fn save_devices(devices: &[DeviceConfig]) -> Result<()> {
    let path = devices_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_vec_pretty(devices)?;
    std::fs::write(&path, json)?;
    Ok(())
}
