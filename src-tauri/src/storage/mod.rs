use anyhow::Result;
use std::path::PathBuf;

use crate::devices::DeviceConfig;

pub fn data_dir() -> PathBuf {
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

fn images_path() -> PathBuf {
    data_dir().join("images.json")
}

pub fn load_images() -> Result<Vec<crate::android::image_manager::ImageEntry>> {
    let path = images_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let bytes = std::fs::read(&path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn save_images(images: &[crate::android::image_manager::ImageEntry]) -> Result<()> {
    let path = images_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_vec_pretty(images)?;
    std::fs::write(&path, json)?;
    Ok(())
}

fn snapshots_path() -> PathBuf {
    data_dir().join("snapshots.json")
}

pub fn load_snapshots() -> Result<Vec<crate::android::snapshot::Snapshot>> {
    let path = snapshots_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let bytes = std::fs::read(&path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn save_snapshots(snapshots: &[crate::android::snapshot::Snapshot]) -> Result<()> {
    let path = snapshots_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_vec_pretty(snapshots)?;
    std::fs::write(&path, json)?;
    Ok(())
}
