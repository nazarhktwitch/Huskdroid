use anyhow::Result;
use std::process::Child;

/// Common interface for all execution backends (QEMU, LXC, custom)
pub trait Backend: Send + Sync {
    /// Start the device and return the child process handle
    fn start(&mut self, args: BackendArgs) -> Result<Child>;
    /// Optional graceful shutdown signal before the process is killed
    fn stop_graceful(&mut self, _pid: u32) -> Result<()> { Ok(()) }
    /// Check if the process is still alive by PID
    fn is_running(&self, pid: u32) -> bool;
}

/// Normalized arguments passed from DeviceConfig to a backend
#[derive(Debug, Clone)]
pub struct BackendArgs {
    pub image_path: String,
    pub ram_mb: u32,
    pub disable_network: bool,
    pub enable_root: bool,
    pub display_mode: String,
    pub custom_args: Option<String>,
}
