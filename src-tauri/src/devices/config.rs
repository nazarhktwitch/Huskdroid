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
    pub android_version: String,
    pub image_path: Option<String>,
    pub ram_mb: u32,
    #[serde(default)]
    pub root: RootConfig,
    #[serde(default)]
    pub sandbox: SandboxConfig,
    /// windowed, headless, vnc
    #[serde(default = "default_display_mode")]
    pub display_mode: String,
    #[serde(default)]
    pub custom_qemu_args: Option<String>,
    pub created_at: DateTime<Utc>,
}

fn default_display_mode() -> String {
    "windowed".to_string()
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
            display_mode: "windowed".to_string(),
            custom_qemu_args: None,
            created_at: Utc::now(),
        }
    }
}
