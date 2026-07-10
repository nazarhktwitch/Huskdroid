use crate::settings::load_settings;
use futures_util::StreamExt;
use reqwest::Client;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use zip::ZipArchive;

#[derive(serde::Serialize)]
pub struct DepsStatus {
    pub adb_found: bool,
    pub adb_path: String,
    pub qemu_found: bool,
    pub qemu_path: String,
}

pub fn get_bin_dir() -> anyhow::Result<PathBuf> {
    let dir = dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("com.huskdroid.dev")
        .join("bin");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn resolve_adb_path() -> Option<String> {
    if let Ok(settings) = load_settings() {
        if let Some(custom) = settings.custom_adb_path {
            if !custom.trim().is_empty() {
                return Some(custom);
            }
        }
    }

    if let Ok(bin_dir) = get_bin_dir() {
        let adb_exe = if cfg!(windows) { "adb.exe" } else { "adb" };
        let local_adb = bin_dir.join("platform-tools").join(adb_exe);
        if local_adb.exists() {
            return Some(local_adb.to_string_lossy().to_string());
        }
    }
    
    if which::which("adb").is_ok() {
        return Some("adb".to_string());
    }
    None
}

pub fn resolve_qemu_path() -> Option<String> {
    let qemu_exe = if cfg!(windows) { "qemu-system-x86_64.exe" } else { "qemu-system-x86_64" };
    
    if let Ok(settings) = load_settings() {
        if let Some(custom) = settings.custom_qemu_path {
            if !custom.trim().is_empty() {
                return Some(custom);
            }
        }
    }

    if let Ok(bin_dir) = get_bin_dir() {
        let local_qemu = bin_dir.join("qemu").join(qemu_exe);
        if local_qemu.exists() {
            return Some(local_qemu.to_string_lossy().to_string());
        }
    }

    if which::which(qemu_exe).is_ok() {
        return Some(qemu_exe.to_string());
    }

    #[cfg(target_os = "windows")]
    {
        let fallback = std::path::Path::new("C:\\Program Files\\qemu").join(qemu_exe);
        if fallback.exists() {
            return Some(fallback.to_string_lossy().to_string());
        }
    }

    None
}

#[tauri::command]
pub async fn check_deps() -> Result<DepsStatus, String> {
    let adb_path = resolve_adb_path();
    let qemu_path = resolve_qemu_path();
    
    Ok(DepsStatus {
        adb_found: adb_path.is_some(),
        adb_path: adb_path.unwrap_or_default(),
        qemu_found: qemu_path.is_some(),
        qemu_path: qemu_path.unwrap_or_default(),
    })
}

#[tauri::command]
pub async fn download_adb(app: AppHandle) -> Result<(), String> {
    let url = if cfg!(windows) {
        "https://dl.google.com/android/repository/platform-tools-latest-windows.zip"
    } else if cfg!(target_os = "macos") {
        "https://dl.google.com/android/repository/platform-tools-latest-darwin.zip"
    } else {
        "https://dl.google.com/android/repository/platform-tools-latest-linux.zip"
    };

    let bin_dir = get_bin_dir().map_err(|e| e.to_string())?;
    let client = Client::new();
    let res = client.get(url).send().await.map_err(|e| e.to_string())?;
    
    let total_size = res.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    let mut buffer = Vec::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        buffer.extend_from_slice(&chunk);
        downloaded += chunk.len() as u64;
        
        let progress = if total_size > 0 {
            (downloaded as f64 / total_size as f64 * 100.0) as u8
        } else {
            0
        };
        let _ = app.emit("adb_download_progress", progress);
    }

    let reader = Cursor::new(buffer);
    let mut archive = ZipArchive::new(reader).map_err(|e| e.to_string())?;
    archive.extract(&bin_dir).map_err(|e| e.to_string())?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let adb_exe = bin_dir.join("platform-tools").join("adb");
        if let Ok(mut perms) = fs::metadata(&adb_exe).map(|m| m.permissions()) {
            perms.set_mode(0o755);
            let _ = fs::set_permissions(&adb_exe, perms);
        }
    }

    Ok(())
}
