use crate::android::filesystem::{
    delete_device_file, list_device_dir, list_dir, pull_file, push_file, FsEntry,
};
use std::path::Path;

#[tauri::command]
pub fn fs_list_dir(path: String) -> Result<Vec<FsEntry>, String> {
    list_dir(Path::new(&path)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn fs_list_device_dir(remote: String) -> Result<Vec<String>, String> {
    list_device_dir(&remote).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn fs_pull_file(remote: String, local: String) -> Result<(), String> {
    pull_file(&remote, Path::new(&local)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn fs_push_file(local: String, remote: String) -> Result<(), String> {
    push_file(Path::new(&local), &remote).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn fs_delete_file(remote: String) -> Result<(), String> {
    delete_device_file(&remote).map_err(|e| e.to_string())
}
