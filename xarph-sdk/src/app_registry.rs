use crate::desktop_object::ObjectData;
use freedesktop_desktop_entry::{self, DesktopEntry, Iter};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub exec: Option<String>,
    pub desktop_file_path: PathBuf,
    pub categories: Vec<String>,
    pub comment: Option<String>,
    pub terminal: bool,
}

impl AppEntry {
    pub fn from_desktop_entry(entry: &DesktopEntry) -> Self {
        let locales = freedesktop_desktop_entry::get_languages_from_env();
        Self {
            id: entry.id().to_string(),
            name: entry
                .name(&locales)
                .map(|s| s.to_string())
                .unwrap_or_else(|| entry.id().to_string()),
            icon: entry.icon().map(|s| s.to_string()),
            exec: entry.exec().map(|s| s.to_string()),
            desktop_file_path: PathBuf::new(),
            categories: entry
                .categories()
                .map(|cats| cats.iter().map(|c| c.to_string()).collect())
                .unwrap_or_default(),
            comment: entry
                .comment(&locales)
                .map(|s| s.to_string()),
            terminal: entry.try_exec().is_some(),
        }
    }

    pub fn to_object_data(&self, x: f64, y: f64) -> ObjectData {
        ObjectData::Application {
            id: uuid::Uuid::new_v4().to_string(),
            name: self.name.clone(),
            desktop_file: self.desktop_file_path.to_string_lossy().to_string(),
            position: crate::position::DesktopObjectPosition::desktop(x, y),
            metadata: std::collections::HashMap::new(),
        }
    }
}

pub struct AppRegistry {
    apps: Vec<AppEntry>,
}

impl AppRegistry {
    pub fn new() -> Self {
        Self { apps: Vec::new() }
    }

    pub fn load() -> Self {
        let mut registry = Self::new();
        registry.refresh();
        registry
    }

    pub fn refresh(&mut self) {
        self.apps.clear();

        let locales = freedesktop_desktop_entry::get_languages_from_env();
        let entries = Iter::new(freedesktop_desktop_entry::default_paths())
            .entries(Some(&locales));

        for entry in entries {
            if entry.no_display() {
                continue;
            }
            self.apps.push(AppEntry::from_desktop_entry(&entry));
        }

        self.apps.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn all(&self) -> &[AppEntry] {
        &self.apps
    }

    pub fn search(&self, query: &str) -> Vec<&AppEntry> {
        let query_lower = query.to_lowercase();
        self.apps
            .iter()
            .filter(|app| {
                app.name.to_lowercase().contains(&query_lower)
                    || app
                        .categories
                        .iter()
                        .any(|c| c.to_lowercase().contains(&query_lower))
                    || app
                        .comment
                        .as_ref()
                        .map(|c| c.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            })
            .collect()
    }

    pub fn by_category(&self, category: &str) -> Vec<&AppEntry> {
        self.apps
            .iter()
            .filter(|app| {
                app.categories
                    .iter()
                    .any(|c| c.eq_ignore_ascii_case(category))
            })
            .collect()
    }

    pub fn find_by_desktop_id(&self, desktop_id: &str) -> Option<&AppEntry> {
        self.apps.iter().find(|app| app.id == desktop_id)
    }

    pub fn find_by_name(&self, name: &str) -> Option<&AppEntry> {
        self.apps.iter().find(|app| app.name == name)
    }

    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<String> = self
            .apps
            .iter()
            .flat_map(|app| app.categories.clone())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }

    pub fn len(&self) -> usize {
        self.apps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.apps.is_empty()
    }
}

impl Default for AppRegistry {
    fn default() -> Self {
        Self::load()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_registry_load() {
        let registry = AppRegistry::load();
        assert!(registry.len() > 0, "Should find at least some apps");
    }

    #[test]
    fn test_search() {
        let registry = AppRegistry::load();
        let results = registry.search("terminal");
        assert!(!results.is_empty(), "Should find terminal-related apps");
    }
}
