#[cfg(test)]
mod tests {
    use crate::devices::config::{DeviceConfig, RootConfig, SandboxConfig};

    #[test]
    fn device_config_round_trip() {
        let original = DeviceConfig::new("test", "Android 14", 512);
        let json = serde_json::to_string(&original).expect("serialize failed");
        let decoded: DeviceConfig = serde_json::from_str(&json).expect("deserialize failed");
        assert_eq!(original.id, decoded.id);
        assert_eq!(original.name, decoded.name);
        assert_eq!(original.ram_mb, decoded.ram_mb);
        assert_eq!(original.android_version, decoded.android_version);
    }

    #[test]
    fn root_config_defaults() {
        let root = RootConfig::default();
        assert!(!root.enabled);
        assert_eq!(root.selinux, "enforcing");
        assert!(!root.magisk);
    }

    #[test]
    fn sandbox_config_defaults_all_off() {
        let sandbox = SandboxConfig::default();
        assert!(!sandbox.disable_network);
        assert!(!sandbox.reset_on_shutdown);
        assert!(!sandbox.fake_device_info);
        assert!(!sandbox.fake_gps);
        assert!(!sandbox.isolated_storage);
    }

    #[test]
    fn device_config_unique_ids() {
        let a = DeviceConfig::new("a", "Android 12", 256);
        let b = DeviceConfig::new("b", "Android 12", 256);
        assert_ne!(a.id, b.id);
    }
}
