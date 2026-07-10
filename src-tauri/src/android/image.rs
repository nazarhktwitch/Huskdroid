    use std::path::Path;

/// Detected image format
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageFormat {
    Raw,
    Qcow2,
    Iso,
    Vdi,
    Vmdk,
    Folder,
    Unknown,
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw    => write!(f, "raw"),
            Self::Qcow2  => write!(f, "qcow2"),
            Self::Iso    => write!(f, "iso"),
            Self::Vdi    => write!(f, "vdi"),
            Self::Vmdk   => write!(f, "vmdk"),
            Self::Folder => write!(f, "folder"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Detect the format of an Android image at the given path
pub fn detect_image(path: &Path) -> ImageFormat {
    if path.is_dir() {
        return detect_image_folder(path);
    }

    match path.extension().and_then(|e| e.to_str()) {
        Some("img") | Some("raw") => ImageFormat::Raw,
        Some("qcow2")             => ImageFormat::Qcow2,
        Some("iso")               => ImageFormat::Iso,
        Some("vdi")               => ImageFormat::Vdi,
        Some("vmdk")              => ImageFormat::Vmdk,
        _                         => ImageFormat::Unknown,
    }
}

/// A folder qualifies as an Android image if it contains at least boot.img or system.img
fn detect_image_folder(path: &Path) -> ImageFormat {
    let markers = ["boot.img", "system.img", "vendor.img", "userdata.img"];
    let has_marker = markers
        .iter()
        .any(|name| path.join(name).exists());

    if has_marker {
        ImageFormat::Folder
    } else {
        ImageFormat::Unknown
    }
}
