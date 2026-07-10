use anyhow::{anyhow, Context, Result};
use std::process::{Child, Command};
use crate::commands::deps::resolve_qemu_path;

use super::backend::{Backend, BackendArgs};

pub struct QemuBackend;

impl QemuBackend {
    pub fn check_available() -> bool {
        resolve_qemu_path().is_some()
    }
}

impl Backend for QemuBackend {
    fn start(&mut self, args: BackendArgs) -> Result<Child> {
        let qemu = resolve_qemu_path().ok_or_else(|| {
            anyhow!("QEMU not found - please download it from settings or add it to PATH")
        })?;

        let mut cmd = Command::new(qemu);

        cmd.args(["-m", &args.ram_mb.to_string()]);
        cmd.args([
            "-drive",
            &format!("file={},format=raw,if=virtio", args.image_path),
        ]);
        cmd.args(["-serial", "tcp::5554,server,nowait"]);

        if args.disable_network {
            cmd.args(["-net", "none"]);
        } else {
            cmd.args(["-net", "nic", "-net", "user,hostfwd=tcp::5555-:5555"]);
        }

        match args.display_mode.as_str() {
            "headless" => {
                cmd.args(["-display", "none"]);
            }
            "vnc" => {
                cmd.args(["-display", "vnc=127.0.0.1:0"]); // Port 5900
            }
            _ => {
                // "windowed" or default: let QEMU pick default SDL/GTK window
            }
        }

        cmd.args(["-vga", "std"]); // Android hangs without a VGA adapter
        cmd.args(["-smp", "2"]);   // Give it 2 cores for faster boot

        #[cfg(target_os = "linux")]
        cmd.args(["-enable-kvm", "-cpu", "host"]);
        
        #[cfg(target_os = "windows")]
        cmd.args(["-machine", "q35", "-accel", "whpx,kernel-irqchip=off", "-accel", "hax", "-accel", "tcg"]);
        
        #[cfg(target_os = "macos")]
        cmd.args(["-machine", "q35", "-accel", "hvf", "-accel", "tcg", "-cpu", "max"]);

        if let Some(custom) = args.custom_args {
            let parts = custom.split_whitespace();
            for part in parts {
                cmd.arg(part);
            }
        }

        cmd.spawn().context("failed to spawn QEMU process")
    }

    fn is_running(&self, pid: u32) -> bool {
        #[cfg(windows)]
        {
            Command::new("tasklist")
                .args(["/FI", &format!("PID eq {pid}"), "/NH"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).contains(&pid.to_string()))
                .unwrap_or(false)
        }
        #[cfg(not(windows))]
        {
            std::path::Path::new(&format!("/proc/{pid}")).exists()
        }
    }
}

fn which_qemu() -> Option<String> {
    let name = "qemu-system-x86_64";
    // Windows
    if let Ok(out) = Command::new("where").arg(name).output() {
        if out.status.success() {
            let p = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !p.is_empty() { return Some(p); }
        }
    }
    // Unix
    if let Ok(out) = Command::new("which").arg(name).output() {
        if out.status.success() {
            let p = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !p.is_empty() { return Some(p); }
        }
    }
    None
}
