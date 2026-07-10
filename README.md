# Huskdroid

A lightweight Android emulator without bloat.

Run Android instances with root access, manage images, inspect and modify filesystems
offline, install APKs, and create snapshots without a heavy emulator.

## Features

- **Device Manager** - create and manage multiple Android device configurations
- **Image Support** - import raw `.img`, `.qcow2`, `.iso`, and folder images
- **QEMU Backend** - run Android-x86 images via QEMU with hardware acceleration on Linux
- **Root Access** - configure root and SELinux mode per device
- **APK Manager** - install, uninstall, and browse installed packages via ADB
- **Snapshots** - take and restore point-in-time snapshots of any device image
- **Filesystem Browser** - browse and manage device files over ADB shell
- **ADB Console** - run shell commands directly from the UI
- **Sandbox Mode** - isolate network, reset on shutdown, fake device info

## Getting Started

Releases are **not yet** available. To build from source, see [Build](#build).

## Requirements

| Tool | Purpose |
|------|---------|
| [Rust](https://rustup.rs) ≥ 1.75 | Backend compilation |
| [Node.js](https://nodejs.org) ≥ 18 | Frontend build |
| [QEMU](https://www.qemu.org/download/) | Device boot (optional for UI development) |
| [Android SDK platform-tools](https://developer.android.com/tools/releases/platform-tools) | ADB / APK features |

## Build

```sh
npm install
npm run tauri dev        # development (hot reload)
npm run tauri build      # production bundle
```

## Android Images

No images are bundled. Download separately and import via the **Images** tab:

- [Android-x86](https://www.android-x86.org/download)
- [Waydroid images](https://sourceforge.net/projects/waydroid/files/images/)
- [LineageOS](https://download.lineageos.org)

(You can use ANY android image you want, even self-compiled ones!)

## License

MIT - see [LICENSE](LICENSE).
