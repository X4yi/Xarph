use std::fs;
use std::path::PathBuf;

use super::{
    ApplicationObject, DesktopObject, FileObject, FolderObject, ObjectData, ProjectObject,
    ShortcutObject, WidgetObject,
};

#[derive(Clone)]
pub struct ObjectPersistence {
    path: PathBuf,
}

impl ObjectPersistence {
    pub fn new() -> Self {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
        path.push("xarph");
        path.push("desktop");
        path.push("objects.json");
        Self { path }
    }

    pub fn load(&self) -> Vec<ObjectData> {
        if let Ok(contents) = fs::read_to_string(&self.path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    pub fn save(&self, objects: &[ObjectData]) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = serde_json::to_string_pretty(objects)?;
        fs::write(&self.path, contents)
    }

    pub fn deserialize_all(data: Vec<ObjectData>) -> Vec<Box<dyn DesktopObject>> {
        data.into_iter()
            .filter_map(|d| -> Option<Box<dyn DesktopObject>> {
                match d {
                    ObjectData::Folder { .. } => {
                        FolderObject::from_data(&d).map(|o| Box::new(o) as Box<dyn DesktopObject>)
                    }
                    ObjectData::File { .. } => {
                        FileObject::from_data(&d).map(|o| Box::new(o) as Box<dyn DesktopObject>)
                    }
                    ObjectData::Application { .. } => ApplicationObject::from_data(&d)
                        .map(|o| Box::new(o) as Box<dyn DesktopObject>),
                    ObjectData::Project { .. } => {
                        ProjectObject::from_data(&d).map(|o| Box::new(o) as Box<dyn DesktopObject>)
                    }
                    ObjectData::Shortcut { .. } => {
                        ShortcutObject::from_data(&d).map(|o| Box::new(o) as Box<dyn DesktopObject>)
                    }
                    ObjectData::Widget { .. } => {
                        WidgetObject::from_data(&d).map(|o| Box::new(o) as Box<dyn DesktopObject>)
                    }
                }
            })
            .collect()
    }
}

impl Default for ObjectPersistence {
    fn default() -> Self {
        Self::new()
    }
}
