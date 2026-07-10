use anyhow::{Context, Result};
use std::process::Command;

/// Run an adb command and return stdout as a String.
/// Checks that adb is in PATH before running.
pub fn adb_cmd(args: &[&str]) -> Result<String> {
    let output = Command::new("adb")
        .args(args)
        .output()
        .context("adb not found in PATH - install Android SDK platform-tools")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("adb error: {stderr}");
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Run a shell command on the connected device and return output
pub fn adb_shell(cmd: &str) -> Result<String> {
    adb_cmd(&["shell", cmd])
}

/// List connected ADB devices
pub fn adb_devices() -> Result<Vec<String>> {
    let out = adb_cmd(&["devices"])?;
    let devices = out
        .lines()
        .skip(1) // skip "List of devices attached"
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect();
    Ok(devices)
}

/// Returns true if adb is available in PATH
pub fn adb_available() -> bool {
    Command::new("adb")
        .arg("version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
