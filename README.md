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

## Troubleshooting

### QEMU Hangs on "ANDROID" Logo (Android-x86 ISO)
If you are booting an official Android-x86 `.iso` file and it gets stuck on the loading logo for more than a few minutes, it is likely a graphics driver conflict with QEMU's default VGA adapter.

**Option 1 (Recommended): Use a pre-installed image**
Android-x86 Live CDs often fail to boot on QEMU for Windows (WHPX) due to missing 3D hardware acceleration drivers in the Windows build of QEMU.
Instead of an ISO, download a pre-installed virtual machine image (like `.vdi` or `.vmdk`) from [OSBoxes](https://www.osboxes.org/android-x86/).
You can import `.vdi` and `.vmdk` files directly into Huskdroid! No conversion is needed.

(If it still not booting, try option 2)

**Option 2: QEMU Extra Arguments (Not guaranteed)**
If you still want to try booting the ISO, you can force QEMU to use a different virtual graphics card.
1. In Huskdroid, open your device's settings.
2. In the **Extra Args** field, type `-vga vmware`, `-vga qxl`, `-vga std` or `-vga virtio`.
3. Save and start the device.
*(Note: This fix is NOT guaranteed to work on Windows due to WHPX limitations, and the ISO still might hang).*
*(Also try increasing ram)*

### QEMU Error: "Could not open ... cannot find the specified path"
If you get this error when starting a device, even though the image file clearly exists, **it is likely because the path contains non-English (e.g., Cyrillic/Russian) characters.** (For example: `D:\документы\android.vdi`).
QEMU on Windows has limited support for non-ASCII characters in file paths.
**Fix:** Move your `.vdi` or `.iso` file to a folder that contains only English letters and no special characters (e.g., `D:\VMs\android.vdi`) and re-import it into Huskdroid.

## TODO

- **Root Access Integration:** The Root toggle is present in the UI but not yet fully integrated into the backend. In the future, this will inject `su` via ADB or patch the boot image on the fly.
- **Hardware Acceleration:** Native VirtIO GPU support and better 3D acceleration for Windows WHPX.
- **Automatic Image Conversion:** Transparently converting raw disks to optimal formats on import.

## License

MIT - see [LICENSE](LICENSE).
