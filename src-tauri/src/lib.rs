pub mod android;
pub mod commands;
pub mod devices;
pub mod engine;
pub mod storage;

use android::image_manager::ImageManager;
use commands::devices::{create_device, delete_device, get_device, list_devices, Devices};
use commands::apk::{check_adb, exec_adb_root, exec_shell, get_selinux_mode, install_apk, list_adb_devices, list_packages, set_selinux_mode, uninstall_apk};
use commands::images::{delete_image, import_image, list_images, Images};
use commands::runtime::{check_qemu, device_status, start_device, stop_device, AppRuntime};
use devices::DeviceManager;
use engine::runtime::Runtime;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let devices = DeviceManager::load().unwrap_or_else(|e| {
        eprintln!("failed to load devices from disk: {e}");
        DeviceManager::empty()
    });

    let images = ImageManager::load().unwrap_or_else(|e| {
        eprintln!("failed to load images from disk: {e}");
        ImageManager::empty()
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Devices(Mutex::new(devices)))
        .manage(Images(Mutex::new(images)))
        .manage(AppRuntime(Mutex::new(Runtime::new())))
        .invoke_handler(tauri::generate_handler![
            list_devices,
            create_device,
            delete_device,
            get_device,
            list_images,
            import_image,
            delete_image,
            start_device,
            stop_device,
            device_status,
            check_qemu,
            check_adb,
            list_adb_devices,
            install_apk,
            uninstall_apk,
            list_packages,
            exec_adb_root,
            get_selinux_mode,
            set_selinux_mode,
            exec_shell,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
