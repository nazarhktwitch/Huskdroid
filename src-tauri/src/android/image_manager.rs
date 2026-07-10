use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::android::image::{detect_image, ImageFormat};

/// A registered Android image entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageEntry {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub format: ImageFormat,
    pub added_at: DateTime<Utc>,
}

pub struct ImageManager {
    images: Vec<ImageEntry>,
}

impl ImageManager {
    pub fn load() -> Result<Self> {
        let images = crate::storage::load_images()?;
        Ok(Self { images })
    }

    pub fn empty() -> Self {
        Self { images: Vec::new() }
    }

    pub fn list(&self) -> &[ImageEntry] {
        &self.images
    }

    pub fn import(&mut self, name: String, path: String) -> Result<&ImageEntry> {
        let p = std::path::Path::new(&path);
        let format = detect_image(p);
        let entry = ImageEntry {
            id: Uuid::new_v4(),
            name,
            path,
            format,
            added_at: Utc::now(),
        };
        self.images.push(entry);
        crate::storage::save_images(&self.images)?;
        Ok(self.images.last().unwrap())
    }

    pub fn delete(&mut self, id: Uuid) -> Result<()> {
        let before = self.images.len();
        self.images.retain(|i| i.id != id);
        if self.images.len() == before {
            return Err(anyhow::anyhow!("image not found: {id}"));
        }
        crate::storage::save_images(&self.images)?;
        Ok(())
    }
}
