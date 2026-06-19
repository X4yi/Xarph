use crate::config::{WallpaperConfig, WallpaperMode};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct WallpaperEntry {
    pub path: PathBuf,
    pub is_video: bool,
    pub name: String,
}

impl WallpaperEntry {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        if !path.exists() {
            return None;
        }

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let is_video = matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("mp4" | "webm" | "mkv" | "mov" | "avi")
        );

        let is_image = matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("png" | "jpg" | "jpeg" | "webp" | "avif" | "gif")
        );

        if is_video || is_image {
            Some(Self {
                path,
                is_video,
                name,
            })
        } else {
            None
        }
    }
}

pub struct WallpaperEngine;

impl WallpaperEngine {
    pub fn get_wallpaper_dirs() -> Vec<PathBuf> {
        let home = dirs::home_dir().unwrap_or_default();
        let data = dirs::data_dir().unwrap_or_default();
        let config = dirs::config_dir().unwrap_or_default();

        vec![
            PathBuf::from("/usr/share/backgrounds"),
            PathBuf::from("/usr/share/wallpapers"),
            home.join("Pictures/Wallpapers"),
            home.join("Pictures/Backgrounds"),
            data.join("wallpapers"),
            config.join("xarph/wallpapers"),
            home.join(".local/share/backgrounds"),
        ]
    }

    pub fn collect_wallpapers(search: Option<&str>, favorites_only: bool) -> Vec<WallpaperEntry> {
        let favorites = Self::load_favorites();
        let search_lower = search.map(|s| s.to_lowercase());
        let mut results = Vec::new();

        for dir in Self::get_wallpaper_dirs() {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(wallpaper) = WallpaperEntry::from_path(path) {
                        if favorites_only
                            && !favorites.contains(&wallpaper.path.to_string_lossy().to_string())
                        {
                            continue;
                        }

                        if let Some(ref search) = search_lower {
                            if !wallpaper.name.to_lowercase().contains(search) {
                                continue;
                            }
                        }

                        results.push(wallpaper);
                    }
                }
            }
        }

        results.sort_by(|a, b| a.name.cmp(&b.name));
        results.dedup_by(|a, b| a.path == b.path);
        results
    }

    pub fn load_favorites() -> Vec<String> {
        let path = Self::favorites_path();
        if let Ok(contents) = std::fs::read_to_string(&path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    pub fn save_favorites(favorites: &[String]) -> Result<(), String> {
        let path = Self::favorites_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create dir: {}", e))?;
        }
        let content = serde_json::to_string_pretty(favorites)
            .map_err(|e| format!("Failed to serialize: {}", e))?;
        std::fs::write(&path, content).map_err(|e| format!("Failed to write: {}", e))
    }

    pub fn add_favorite(path: &str) -> Result<(), String> {
        let mut favorites = Self::load_favorites();
        if !favorites.contains(&path.to_string()) {
            favorites.push(path.to_string());
            Self::save_favorites(&favorites)?;
        }
        Ok(())
    }

    pub fn remove_favorite(path: &str) -> Result<(), String> {
        let mut favorites = Self::load_favorites();
        favorites.retain(|f| f != path);
        Self::save_favorites(&favorites)
    }

    pub fn is_favorite(path: &str) -> bool {
        Self::load_favorites().contains(&path.to_string())
    }

    pub fn favorites_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("xarph")
            .join("desktop")
            .join("wallpaper_favorites.json")
    }

    pub fn resolve_wallpaper_for_workspace(
        global: &WallpaperConfig,
        workspace_wallpapers: &[(u8, WallpaperConfig)],
        workspace_id: u8,
    ) -> WallpaperConfig {
        // Try workspace-specific wallpaper first
        for (ws_id, config) in workspace_wallpapers {
            if *ws_id == workspace_id {
                return config.clone();
            }
        }
        // Fall back to global wallpaper
        global.clone()
    }

    pub fn content_fit(mode: &WallpaperMode) -> &'static str {
        match mode {
            WallpaperMode::Fill => "cover",
            WallpaperMode::Fit => "contain",
            WallpaperMode::Stretch => "stretch",
            WallpaperMode::Center => "center",
            WallpaperMode::Tile => "repeat",
        }
    }

    pub fn is_video_file(path: &Path) -> bool {
        matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("mp4" | "webm" | "mkv" | "mov" | "avi")
        )
    }

    pub fn is_image_file(path: &Path) -> bool {
        matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("png" | "jpg" | "jpeg" | "webp" | "avif" | "gif")
        )
    }
}
