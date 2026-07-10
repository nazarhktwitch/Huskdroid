use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

use crate::commands::devices::Devices;
use crate::engine::backend::{Backend, BackendArgs};
use crate::engine::runtime::Runtime;
use crate::engine::virtualization::QemuBackend;

pub struct AppRuntime(pub Mutex<Runtime>);

#[derive(serde::Serialize)]
pub struct DeviceStatus {
    pub id: String,
    pub status: String,
    pub qemu_available: bool,
}

#[tauri::command]
pub fn start_device(
    id: Uuid,
    runtime_state: State<AppRuntime>,
    devices_state: State<Devices>,
) -> Result<DeviceStatus, String> {
    let args = {
        let devices = devices_state.0.lock().map_err(|e| e.to_string())?;
        let device = devices
            .get(id)
            .ok_or_else(|| format!("device not found: {id}"))?;
        BackendArgs {
            image_path: device
                .image_path
                .clone()
                .ok_or("device has no image assigned")?,
            ram_mb: device.ram_mb,
            disable_network: device.sandbox.disable_network,
            enable_root: device.root.enabled,
        }
    };

    let child = QemuBackend
        .start(args)
        .map_err(|e| e.to_string())?;

    let mut rt = runtime_state.0.lock().map_err(|e| e.to_string())?;
    rt.insert(id, child);

    Ok(DeviceStatus {
        id: id.to_string(),
        status: "running".to_string(),
        qemu_available: true,
    })
}

#[tauri::command]
pub fn stop_device(
    id: Uuid,
    runtime_state: State<AppRuntime>,
) -> Result<DeviceStatus, String> {
    let mut rt = runtime_state.0.lock().map_err(|e| e.to_string())?;
    rt.remove(id).map_err(|e| e.to_string())?;
    Ok(DeviceStatus {
        id: id.to_string(),
        status: "stopped".to_string(),
        qemu_available: QemuBackend::check_available(),
    })
}

#[tauri::command]
pub fn device_status(
    id: Uuid,
    runtime_state: State<AppRuntime>,
) -> Result<DeviceStatus, String> {
    let mut rt = runtime_state.0.lock().map_err(|e| e.to_string())?;
    let status = rt.status(id).to_string();
    Ok(DeviceStatus {
        id: id.to_string(),
        status,
        qemu_available: QemuBackend::check_available(),
    })
}

#[tauri::command]
pub fn check_qemu() -> bool {
    QemuBackend::check_available()
}
