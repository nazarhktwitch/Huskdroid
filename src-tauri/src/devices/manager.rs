use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::devices::config::DeviceConfig;
use crate::storage;

/// In-memory device list, loaded from disk on init and written on every mutation
pub struct DeviceManager {
    devices: Vec<DeviceConfig>,
}

impl DeviceManager {
    pub fn load() -> Result<Self> {
        let devices = storage::load_devices()?;
        Ok(Self { devices })
    }

    pub fn empty() -> Self {
        Self { devices: Vec::new() }
    }

    pub fn list(&self) -> &[DeviceConfig] {
        &self.devices
    }

    pub fn get(&self, id: Uuid) -> Option<&DeviceConfig> {
        self.devices.iter().find(|d| d.id == id)
    }

    pub fn create(&mut self, config: DeviceConfig) -> Result<&DeviceConfig> {
        self.devices.push(config);
        storage::save_devices(&self.devices)?;
        Ok(self.devices.last().unwrap())
    }

    pub fn delete(&mut self, id: Uuid) -> Result<()> {
        let before = self.devices.len();
        self.devices.retain(|d| d.id != id);
        if self.devices.len() == before {
            return Err(anyhow!("device not found: {id}"));
        }
        storage::save_devices(&self.devices)?;
        Ok(())
    }

    pub fn update(&mut self, updated: DeviceConfig) -> Result<()> {
        let device = self
            .devices
            .iter_mut()
            .find(|d| d.id == updated.id)
            .ok_or_else(|| anyhow!("device not found: {}", updated.id))?;
        *device = updated;
        storage::save_devices(&self.devices)?;
        Ok(())
    }
}
