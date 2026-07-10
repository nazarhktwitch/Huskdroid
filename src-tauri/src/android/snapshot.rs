use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: Uuid,
    pub device_id: Uuid,
    pub name: String,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub size_bytes: u64,
}

pub struct SnapshotManager {
    snapshots: Vec<Snapshot>,
}

impl SnapshotManager {
    pub fn load() -> Result<Self> {
        let snapshots = crate::storage::load_snapshots()?;
        Ok(Self { snapshots })
    }

    pub fn empty() -> Self {
        Self { snapshots: Vec::new() }
    }

    pub fn list_for_device(&self, device_id: Uuid) -> Vec<&Snapshot> {
        self.snapshots
            .iter()
            .filter(|s| s.device_id == device_id)
            .collect()
    }

    pub fn create(&mut self, device_id: Uuid, name: String, image_path: &str) -> Result<&Snapshot> {
        let snap_dir = snapshot_dir(device_id);
        std::fs::create_dir_all(&snap_dir)?;

        let snap_id = Uuid::new_v4();
        let dest = snap_dir.join(format!("{snap_id}.img"));

        // Copy image to snapshot location
        std::fs::copy(image_path, &dest)
            .with_context(|| format!("failed to copy image to snapshot: {dest:?}"))?;

        let size_bytes = std::fs::metadata(&dest)?.len();

        let snapshot = Snapshot {
            id: snap_id,
            device_id,
            name,
            path: dest.to_string_lossy().to_string(),
            created_at: Utc::now(),
            size_bytes,
        };

        self.snapshots.push(snapshot);
        crate::storage::save_snapshots(&self.snapshots)?;
        Ok(self.snapshots.last().unwrap())
    }

    pub fn restore(&self, snapshot_id: Uuid, image_path: &str) -> Result<()> {
        let snap = self
            .snapshots
            .iter()
            .find(|s| s.id == snapshot_id)
            .ok_or_else(|| anyhow::anyhow!("snapshot not found: {snapshot_id}"))?;

        std::fs::copy(&snap.path, image_path)
            .with_context(|| format!("failed to restore snapshot {snapshot_id}"))?;
        Ok(())
    }

    pub fn delete(&mut self, snapshot_id: Uuid) -> Result<()> {
        let pos = self
            .snapshots
            .iter()
            .position(|s| s.id == snapshot_id)
            .ok_or_else(|| anyhow::anyhow!("snapshot not found: {snapshot_id}"))?;

        let snap = self.snapshots.remove(pos);
        // Remove the snapshot file; ignore error if already gone
        let _ = std::fs::remove_file(&snap.path);
        crate::storage::save_snapshots(&self.snapshots)?;
        Ok(())
    }
}

fn snapshot_dir(device_id: Uuid) -> std::path::PathBuf {
    crate::storage::data_dir().join("snapshots").join(device_id.to_string())
}
