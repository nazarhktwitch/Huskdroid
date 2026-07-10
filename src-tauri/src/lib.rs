pub mod commands;
pub mod devices;
pub mod storage;

use commands::devices::{
    create_device, delete_device, get_device, list_devices, Devices,
};
use devices::DeviceManager;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mgr = DeviceManager::load().unwrap_or_else(|e| {
        eprintln!("failed to load devices from disk: {e}");
        DeviceManager::empty()
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Devices(Mutex::new(mgr)))
        .invoke_handler(tauri::generate_handler![
            list_devices,
            create_device,
            delete_device,
            get_device,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
