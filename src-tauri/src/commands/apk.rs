use crate::android::adb::{adb_available, adb_cmd, adb_devices, adb_shell};
use crate::android::root::{adb_root, get_selinux, set_selinux};

pub struct AdbState; // placeholder for future per-device serial tracking

#[tauri::command]
pub fn check_adb() -> bool {
    adb_available()
}

#[tauri::command]
pub fn list_adb_devices() -> Result<Vec<String>, String> {
    adb_devices().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn install_apk(path: String) -> Result<String, String> {
    adb_cmd(&["install", "-r", &path])
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn uninstall_apk(package: String) -> Result<String, String> {
    adb_cmd(&["uninstall", &package])
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_packages() -> Result<Vec<String>, String> {
    let out = adb_shell("pm list packages")
        .map_err(|e| e.to_string())?;
    let packages = out
        .lines()
        .filter_map(|l| l.strip_prefix("package:"))
        .map(|p| p.trim().to_string())
        .collect();
    Ok(packages)
}

#[tauri::command]
pub fn exec_adb_root() -> Result<(), String> {
    adb_root().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_selinux_mode() -> Result<String, String> {
    get_selinux().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_selinux_mode(mode: String) -> Result<(), String> {
    set_selinux(&mode).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn exec_shell(cmd: String) -> Result<String, String> {
    adb_shell(&cmd).map_err(|e| e.to_string())
}
