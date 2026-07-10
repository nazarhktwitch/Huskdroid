pub mod android;
pub mod commands;
pub mod devices;
pub mod storage;

use android::image_manager::ImageManager;
use commands::devices::{create_device, delete_device, get_device, list_devices, Devices};
use commands::images::{delete_image, import_image, list_images, Images};
use devices::DeviceManager;
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
        .invoke_handler(tauri::generate_handler![
            list_devices,
            create_device,
            delete_device,
            get_device,
            list_images,
            import_image,
            delete_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
