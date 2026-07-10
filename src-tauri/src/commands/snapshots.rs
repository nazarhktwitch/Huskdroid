use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

use crate::android::snapshot::{Snapshot, SnapshotManager};
use crate::commands::devices::Devices;

pub struct Snapshots(pub Mutex<SnapshotManager>);

#[tauri::command]
pub fn list_snapshots(
    state: State<Snapshots>,
    device_id: Uuid,
) -> Result<Vec<Snapshot>, String> {
    let mgr = state.0.lock().map_err(|e| e.to_string())?;
    Ok(mgr.list_for_device(device_id).into_iter().cloned().collect())
}

#[tauri::command]
pub fn create_snapshot(
    state: State<Snapshots>,
    devices_state: State<Devices>,
    device_id: Uuid,
    name: String,
) -> Result<Snapshot, String> {
    let image_path = {
        let devices = devices_state.0.lock().map_err(|e| e.to_string())?;
        let device = devices
            .get(device_id)
            .ok_or_else(|| format!("device not found: {device_id}"))?;
        device
            .image_path
            .clone()
            .ok_or("device has no image - cannot snapshot")?
    };

    let mut mgr = state.0.lock().map_err(|e| e.to_string())?;
    mgr.create(device_id, name, &image_path)
        .map(|s| s.clone())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn restore_snapshot(
    state: State<Snapshots>,
    devices_state: State<Devices>,
    snapshot_id: Uuid,
    device_id: Uuid,
) -> Result<(), String> {
    let image_path = {
        let devices = devices_state.0.lock().map_err(|e| e.to_string())?;
        let device = devices
            .get(device_id)
            .ok_or_else(|| format!("device not found: {device_id}"))?;
        device
            .image_path
            .clone()
            .ok_or("device has no image path")?
    };

    let mgr = state.0.lock().map_err(|e| e.to_string())?;
    mgr.restore(snapshot_id, &image_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_snapshot(
    state: State<Snapshots>,
    snapshot_id: Uuid,
) -> Result<(), String> {
    let mut mgr = state.0.lock().map_err(|e| e.to_string())?;
    mgr.delete(snapshot_id).map_err(|e| e.to_string())
}
