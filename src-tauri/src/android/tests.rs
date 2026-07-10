#[cfg(test)]
mod tests {
    use std::fs;
    use crate::android::image::{detect_image, ImageFormat};

    fn tmp_dir() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("husk_test_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn detects_raw_img() {
        let dir = tmp_dir();
        let file = dir.join("system.img");
        fs::write(&file, b"").unwrap();
        assert_eq!(detect_image(&file), ImageFormat::Raw);
    }

    #[test]
    fn detects_raw_extension() {
        let dir = tmp_dir();
        let file = dir.join("disk.raw");
        fs::write(&file, b"").unwrap();
        assert_eq!(detect_image(&file), ImageFormat::Raw);
    }

    #[test]
    fn detects_qcow2() {
        let dir = tmp_dir();
        let file = dir.join("android.qcow2");
        fs::write(&file, b"").unwrap();
        assert_eq!(detect_image(&file), ImageFormat::Qcow2);
    }

    #[test]
    fn detects_iso() {
        let dir = tmp_dir();
        let file = dir.join("android-x86.iso");
        fs::write(&file, b"").unwrap();
        assert_eq!(detect_image(&file), ImageFormat::Iso);
    }

    #[test]
    fn detects_image_folder() {
        let dir = tmp_dir();
        fs::write(dir.join("boot.img"), b"").unwrap();
        fs::write(dir.join("system.img"), b"").unwrap();
        assert_eq!(detect_image(&dir), ImageFormat::Folder);
    }

    #[test]
    fn unknown_folder_without_markers() {
        let dir = tmp_dir();
        fs::write(dir.join("readme.txt"), b"").unwrap();
        assert_eq!(detect_image(&dir), ImageFormat::Unknown);
    }

    #[test]
    fn unknown_extension() {
        let dir = tmp_dir();
        let file = dir.join("data.bin");
        fs::write(&file, b"").unwrap();
        assert_eq!(detect_image(&file), ImageFormat::Unknown);
    }
}
