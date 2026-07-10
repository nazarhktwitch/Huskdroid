use anyhow::Result;

use super::adb::adb_cmd;

/// Run `adb root` to restart adbd with root privileges
pub fn adb_root() -> Result<()> {
    adb_cmd(&["root"])?;
    Ok(())
}

/// Set SELinux enforcement mode on the device
pub fn set_selinux(mode: &str) -> Result<()> {
    match mode {
        "permissive" => adb_cmd(&["shell", "setenforce", "0"])?,
        "enforcing"  => adb_cmd(&["shell", "setenforce", "1"])?,
        other => anyhow::bail!("unknown SELinux mode: {other}"),
    };
    Ok(())
}

/// Get current SELinux mode from the device
pub fn get_selinux() -> Result<String> {
    let out = adb_cmd(&["shell", "getenforce"])?;
    Ok(out.trim().to_lowercase())
}
