use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

use crate::android::image_manager::{ImageEntry, ImageManager};

pub struct Images(pub Mutex<ImageManager>);

#[tauri::command]
pub fn list_images(state: State<Images>) -> Result<Vec<ImageEntry>, String> {
    let mgr = state.0.lock().map_err(|e| e.to_string())?;
    Ok(mgr.list().to_vec())
}

#[tauri::command]
pub fn import_image(
    state: State<Images>,
    name: String,
    path: String,
) -> Result<ImageEntry, String> {
    let mut mgr = state.0.lock().map_err(|e| e.to_string())?;
    mgr.import(name, path)
        .map(|e| e.clone())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_image(state: State<Images>, id: Uuid) -> Result<(), String> {
    let mut mgr = state.0.lock().map_err(|e| e.to_string())?;
    mgr.delete(id).map_err(|e| e.to_string())
}
