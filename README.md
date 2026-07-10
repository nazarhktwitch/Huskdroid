# Huskdroid

Android sandbox manager for developers, testers, and security researchers.

Run Android instances with root access, manage images, inspect and modify filesystems
offline, install APKs, and create snapshots -- without a heavy emulator.

## Getting Started

Releases are not yet available. To build from source, see [Build](#build).

## Features

- Multi-instance: run several Android instances independently, each with its own config
- Image support: `.img`, `.raw`, `.qcow2`, `.iso`, and multi-file image folders
- Root access: `adb root`, su shell, Magisk integration, and SELinux mode control
- Offline filesystem: browse, edit, copy, and delete files in a stopped device's image
- Snapshots: save and restore device state like a VM
- APK manager: drag-and-drop install, package list, uninstall
- ADB console: built-in terminal with root shell access
- Sandbox mode: disable network, reset on shutdown, fake device info and GPS
- CLI: `husk create`, `husk start`, `husk install`, `husk mount`, `husk snapshot`

## Build

Requirements: Rust, Node.js >= 18, QEMU (for device boot).

```sh
npm install
npm run tauri dev
```

## Requirements

- QEMU must be installed and in `PATH` to start devices
- `adb` must be installed and in `PATH` for APK and shell features
- Windows 10+, Linux, or macOS

## License

[MIT](LICENSE)
