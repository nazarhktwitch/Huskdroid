use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

use crate::devices::{DeviceConfig, DeviceManager};
use crate::devices::config::{RootConfig, SandboxConfig};

/// Wrapper for Tauri managed state
pub struct Devices(pub Mutex<DeviceManager>);

/// Payload for creating a new device from the UI
#[derive(serde::Deserialize)]
pub struct CreateDevicePayload {
    pub name: String,
    pub android_version: String,
    pub ram_mb: u32,
    pub cpu_cores: u32,
    pub root_enabled: bool,
}

#[tauri::command]
pub fn list_devices(state: State<Devices>) -> Result<Vec<DeviceConfig>, String> {
    let mgr = state.0.lock().map_err(|e| e.to_string())?;
    Ok(mgr.list().to_vec())
}

#[tauri::command]
pub fn create_device(
    state: State<Devices>,
    payload: CreateDevicePayload,
) -> Result<DeviceConfig, String> {
    let mut mgr = state.0.lock().map_err(|e| e.to_string())?;
    let mut config = DeviceConfig::new(payload.name, payload.android_version, payload.ram_mb, payload.cpu_cores);
    config.root = RootConfig {
        enabled: payload.root_enabled,
        ..RootConfig::default()
    };
    config.sandbox = SandboxConfig::default();
    let created = mgr.create(config).map_err(|e| e.to_string())?;
    Ok(created.clone())
}

#[tauri::command]
pub fn delete_device(state: State<Devices>, id: Uuid) -> Result<(), String> {
    let mut mgr = state.0.lock().map_err(|e| e.to_string())?;
    mgr.delete(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_device(state: State<Devices>, id: Uuid) -> Result<Option<DeviceConfig>, String> {
    let mgr = state.0.lock().map_err(|e| e.to_string())?;
    Ok(mgr.get(id).cloned())
}

#[tauri::command]
pub fn update_device(
    state: State<Devices>,
    device: DeviceConfig,
) -> Result<(), String> {
    let mut mgr = state.0.lock().map_err(|e| e.to_string())?;
    mgr.update(device).map_err(|e| e.to_string())
}
