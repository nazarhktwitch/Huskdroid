use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::process::Child;
use uuid::Uuid;

/// Tracks running device processes by device ID
pub struct Runtime {
    processes: HashMap<Uuid, Child>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: Uuid, child: Child) {
        self.processes.insert(id, child);
    }

    pub fn remove(&mut self, id: Uuid) -> Result<()> {
        match self.processes.remove(&id) {
            Some(mut child) => {
                let _ = child.kill();
                Ok(())
            }
            None => Err(anyhow!("device {id} is not running")),
        }
    }

    pub fn is_running(&mut self, id: Uuid) -> bool {
        match self.processes.get_mut(&id) {
            Some(child) => child.try_wait().map(|s| s.is_none()).unwrap_or(false),
            None => false,
        }
    }

    pub fn status(&mut self, id: Uuid) -> &'static str {
        if self.is_running(id) { "running" } else { "stopped" }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}
