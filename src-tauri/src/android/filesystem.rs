use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: Option<u64>,
}

/// List the contents of a directory on the host filesystem
pub fn list_dir(path: &Path) -> Result<Vec<FsEntry>> {
    let entries = std::fs::read_dir(path)
        .with_context(|| format!("cannot read directory: {}", path.display()))?;

    let mut result = Vec::new();
    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().to_string();
        let full_path = entry.path().to_string_lossy().to_string();
        result.push(FsEntry {
            name,
            path: full_path,
            is_dir: meta.is_dir(),
            size: if meta.is_file() { Some(meta.len()) } else { None },
        });
    }

    result.sort_by(|a, b| {
        // Directories first, then alphabetical
        b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name))
    });

    Ok(result)
}

/// Pull a file from the device filesystem via adb pull
pub fn pull_file(remote: &str, local: &Path) -> Result<()> {
    let out = std::process::Command::new("adb")
        .args(["pull", remote, &local.to_string_lossy()])
        .output()
        .context("adb pull failed - is adb in PATH?")?;

    if !out.status.success() {
        anyhow::bail!("adb pull: {}", String::from_utf8_lossy(&out.stderr));
    }
    Ok(())
}

/// Push a local file to the device filesystem via adb push
pub fn push_file(local: &Path, remote: &str) -> Result<()> {
    let out = std::process::Command::new("adb")
        .args(["push", &local.to_string_lossy(), remote])
        .output()
        .context("adb push failed - is adb in PATH?")?;

    if !out.status.success() {
        anyhow::bail!("adb push: {}", String::from_utf8_lossy(&out.stderr));
    }
    Ok(())
}

/// Delete a file on the device via adb shell rm
pub fn delete_device_file(remote: &str) -> Result<()> {
    let out = std::process::Command::new("adb")
        .args(["shell", "rm", "-rf", remote])
        .output()
        .context("adb shell rm failed")?;

    if !out.status.success() {
        anyhow::bail!("rm: {}", String::from_utf8_lossy(&out.stderr));
    }
    Ok(())
}

/// List files on the device at a given path via adb shell ls -la
pub fn list_device_dir(remote: &str) -> Result<Vec<String>> {
    let out = std::process::Command::new("adb")
        .args(["shell", "ls", "-la", remote])
        .output()
        .context("adb shell ls failed")?;

    if !out.status.success() {
        anyhow::bail!("ls: {}", String::from_utf8_lossy(&out.stderr));
    }
    let lines = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|l| l.to_string())
        .collect();
    Ok(lines)
}
