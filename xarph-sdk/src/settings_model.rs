use crate::config::{WallpaperConfig, XarphConfig};
use std::path::PathBuf;

pub struct SettingsModel;

impl SettingsModel {
    pub fn load_config() -> XarphConfig {
        XarphConfig::load()
    }

    pub fn save_config(config: &XarphConfig) -> Result<(), String> {
        config.save().map_err(|e| format!("Failed to save config: {}", e))
    }

    pub fn get_wallpaper(&self) -> WallpaperConfig {
        let config = XarphConfig::load();
        config.wallpaper_global.clone()
    }

    pub fn set_wallpaper(&self, wallpaper: WallpaperConfig) -> Result<(), String> {
        let mut config = XarphConfig::load();
        config.wallpaper_global = wallpaper;
        config.save().map_err(|e| format!("Failed to save wallpaper: {}", e))
    }

    pub fn get_workspace_wallpaper(&self, workspace_id: u8) -> WallpaperConfig {
        let config = XarphConfig::load();
        config.get_wallpaper_for_workspace(workspace_id).clone()
    }

    pub fn set_workspace_wallpaper(
        &self,
        workspace_id: u8,
        wallpaper: WallpaperConfig,
    ) -> Result<(), String> {
        let mut config = XarphConfig::load();
        config.set_workspace_wallpaper(workspace_id, wallpaper);
        config.save().map_err(|e| format!("Failed to save workspace wallpaper: {}", e))
    }

    pub fn get_theme(&self) -> Option<String> {
        let config = XarphConfig::load();
        config.theme.clone()
    }

    pub fn set_theme(&self, theme: &str) -> Result<(), String> {
        let mut config = XarphConfig::load();
        config.theme = Some(theme.to_string());
        config.save().map_err(|e| format!("Failed to save theme: {}", e))
    }

    pub fn get_available_themes() -> Vec<String> {
        let mut themes = Vec::new();
        let dirs = vec![
            PathBuf::from("/usr/share/themes"),
            dirs::home_dir().unwrap_or_default().join(".themes"),
            dirs::data_dir().unwrap_or_default().join("themes"),
        ];

        for dir in dirs {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            let has_qt = entry.path().join("qt6").exists()
                                || entry.path().join("QtProject").exists();
                            if has_qt {
                                if let Some(name) = entry.file_name().to_str() {
                                    themes.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        themes.sort();
        themes.dedup();
        themes
    }

    pub fn get_panel_config(&self) -> crate::config::ShellConfig {
        let config = XarphConfig::load();
        config.shell
    }

    pub fn set_panel_visible(
        &self,
        widget_kind: &crate::config::WidgetKind,
        visible: bool,
    ) -> Result<(), String> {
        let mut config = XarphConfig::load();
        if let Some(panel) = config.shell.panels.first_mut() {
            if let Some(widget) = panel
                .widgets
                .iter_mut()
                .find(|w| std::mem::discriminant(&w.kind) == std::mem::discriminant(widget_kind))
            {
                widget.visible = visible;
            }
        }
        config.save().map_err(|e| format!("Failed to save panel config: {}", e))
    }

    pub fn get_keybinds(&self) -> crate::config::KeybindConfig {
        let config = XarphConfig::load();
        config.keybind_config
    }
}
