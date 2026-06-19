use crate::desktop_object::ObjectData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopSnapshot {
    pub objects: Vec<ObjectData>,
    pub version: u32,
    pub timestamp: i64,
}

pub struct DesktopRegistry {
    objects: HashMap<String, ObjectData>,
    persistence_path: PathBuf,
}

impl DesktopRegistry {
    pub fn new(persistence_path: PathBuf) -> Self {
        Self {
            objects: HashMap::new(),
            persistence_path,
        }
    }

    pub fn with_default_path() -> Self {
        let path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("xarph")
            .join("desktop")
            .join("objects.json");
        Self::new(path)
    }

    pub fn register(&mut self, obj: ObjectData) -> bool {
        let id = obj.id().to_string();
        if self.objects.contains_key(&id) {
            return false;
        }
        self.objects.insert(id, obj);
        true
    }

    pub fn unregister(&mut self, id: &str) -> Option<ObjectData> {
        self.objects.remove(id)
    }

    pub fn get(&self, id: &str) -> Option<&ObjectData> {
        self.objects.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ObjectData> {
        self.objects.get_mut(id)
    }

    pub fn contains(&self, id: &str) -> bool {
        self.objects.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    pub fn all(&self) -> Vec<&ObjectData> {
        self.objects.values().collect()
    }

    pub fn all_ids(&self) -> Vec<&str> {
        self.objects.keys().map(|s| s.as_str()).collect()
    }

    pub fn find_by_type(&self, obj_type: &str) -> Vec<&ObjectData> {
        self.objects
            .values()
            .filter(|o| o.object_type() == obj_type)
            .collect()
    }

    pub fn find_by_name(&self, name: &str) -> Vec<&ObjectData> {
        self.objects
            .values()
            .filter(|o| o.name() == name)
            .collect()
    }

    pub fn find_by_metadata(&self, key: &str, value: &str) -> Vec<&ObjectData> {
        self.objects
            .values()
            .filter(|o| o.metadata().get(key).map(|s| s.as_str()) == Some(value))
            .collect()
    }

    pub fn on_workspace(&self, workspace_id: u8) -> Vec<&ObjectData> {
        self.objects
            .values()
            .filter(|o| o.position().workspace_id == Some(workspace_id))
            .collect()
    }

    pub fn in_container(&self, container_id: &str) -> Vec<&ObjectData> {
        self.objects
            .values()
            .filter(|o| o.position().container_id.as_deref() == Some(container_id))
            .collect()
    }

    pub fn in_zone(&self, zone: crate::position::DesktopZone) -> Vec<&ObjectData> {
        self.objects
            .values()
            .filter(|o| o.position().zone == zone)
            .collect()
    }

    pub fn update(&mut self, id: &str, data: ObjectData) -> bool {
        if self.objects.contains_key(id) {
            self.objects.insert(id.to_string(), data);
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    // ── Persistence ──────────────────────────────────────────────

    pub fn load(&mut self) -> Result<(), String> {
        let content = std::fs::read_to_string(&self.persistence_path)
            .map_err(|e| format!("Failed to read {}: {}", self.persistence_path.display(), e))?;
        let objects: Vec<ObjectData> = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse objects.json: {}", e))?;
        self.objects.clear();
        for obj in objects {
            self.objects.insert(obj.id().to_string(), obj);
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {
        if let Some(parent) = self.persistence_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create dir {}: {}", parent.display(), e))?;
        }
        let objects: Vec<&ObjectData> = self.objects.values().collect();
        let content = serde_json::to_string_pretty(&objects)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        std::fs::write(&self.persistence_path, content)
            .map_err(|e| format!("Failed to write {}: {}", self.persistence_path.display(), e))?;
        Ok(())
    }

    pub fn snapshot(&self) -> DesktopSnapshot {
        DesktopSnapshot {
            objects: self.objects.values().cloned().collect(),
            version: 1,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
        }
    }

    pub fn restore(&mut self, snapshot: &DesktopSnapshot) -> Result<(), String> {
        self.objects.clear();
        for obj in &snapshot.objects {
            self.objects.insert(obj.id().to_string(), obj.clone());
        }
        Ok(())
    }

    pub fn serialize(&self) -> Vec<ObjectData> {
        self.objects.values().cloned().collect()
    }

    pub fn deserialize(&mut self, data: Vec<ObjectData>) {
        self.objects.clear();
        for obj in data {
            self.objects.insert(obj.id().to_string(), obj);
        }
    }
}

impl std::fmt::Debug for DesktopRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DesktopRegistry")
            .field("objects_count", &self.objects.len())
            .field("persistence_path", &self.persistence_path)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop_object::new_file;
    use std::path::PathBuf;

    fn test_registry() -> DesktopRegistry {
        let dir = std::env::temp_dir().join("xarph_test_registry");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("objects.json");
        DesktopRegistry::new(path)
    }

    #[test]
    fn test_register_and_get() {
        let mut reg = test_registry();
        let file = new_file("test.rs", "/tmp/test.rs", 10.0, 20.0);
        let id = file.id().to_string();
        assert!(reg.register(file));
        assert!(reg.get(&id).is_some());
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn test_unregister() {
        let mut reg = test_registry();
        let file = new_file("test.rs", "/tmp/test.rs", 10.0, 20.0);
        let id = file.id().to_string();
        reg.register(file);
        let removed = reg.unregister(&id);
        assert!(removed.is_some());
        assert!(reg.is_empty());
    }

    #[test]
    fn test_find_by_type() {
        let mut reg = test_registry();
        reg.register(new_file("a.rs", "/a", 0.0, 0.0));
        reg.register(new_file("b.rs", "/b", 0.0, 0.0));
        assert_eq!(reg.find_by_type("file").len(), 2);
        assert_eq!(reg.find_by_type("folder").len(), 0);
    }

    #[test]
    fn test_persistence_roundtrip() {
        let mut reg = test_registry();
        reg.register(new_file("test.rs", "/tmp/test.rs", 10.0, 20.0));
        reg.save().unwrap();

        let mut reg2 = DesktopRegistry::new(reg.persistence_path.clone());
        reg2.load().unwrap();
        assert_eq!(reg2.len(), 1);
    }
}
