use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// How root access is configured for a device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootConfig {
    pub enabled: bool,
    /// "enforcing" or "permissive"
    pub selinux: String,
    pub magisk: bool,
}

impl Default for RootConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            selinux: "enforcing".to_string(),
            magisk: false,
        }
    }
}

/// Security sandbox options applied at device start
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SandboxConfig {
    pub disable_network: bool,
    pub reset_on_shutdown: bool,
    pub fake_device_info: bool,
    pub fake_gps: bool,
    pub isolated_storage: bool,
}

/// Persistent configuration for a single Android device instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    pub id: Uuid,
    pub name: String,
    /// Display label for the Android version, e.g. "Android 14"
    pub android_version: String,
    /// Path to the image file or directory
    pub image_path: Option<String>,
    /// RAM in megabytes
    pub ram_mb: u32,
    pub root: RootConfig,
    pub sandbox: SandboxConfig,
    pub created_at: DateTime<Utc>,
}

impl DeviceConfig {
    pub fn new(name: impl Into<String>, android_version: impl Into<String>, ram_mb: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            android_version: android_version.into(),
            image_path: None,
            ram_mb,
            root: RootConfig::default(),
            sandbox: SandboxConfig::default(),
            created_at: Utc::now(),
        }
    }
}
