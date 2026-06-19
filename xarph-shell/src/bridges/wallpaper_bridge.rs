/// Wallpaper bridge: exposes wallpaper engine to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, wallpaper_type)]
        #[qproperty(QString, wallpaper_path)]
        #[qproperty(QString, wallpaper_mode)]
        #[qproperty(QString, wallpaper_color)]
        #[qproperty(i32, current_workspace)]
        #[qproperty(QString, wallpaper_list)]
        #[qproperty(QString, favorites_list)]
        #[namespace = "xarph"]
        type WallpaperBridge = super::WallpaperBridgeRust;

        #[qinvokable]
        fn load_config(self: Pin<&mut Self>);

        #[qinvokable]
        fn set_wallpaper(self: Pin<&mut Self>, path: &QString, mode: &QString);

        #[qinvokable]
        fn set_color(self: Pin<&mut Self>, color: &QString);

        #[qinvokable]
        fn set_workspace(self: Pin<&mut Self>, workspace_id: i32);

        #[qinvokable]
        fn get_content_fit(&self) -> QString;

        #[qinvokable]
        fn is_video(&self) -> bool;

        #[qinvokable]
        fn collect_wallpapers(self: Pin<&mut Self>, search: &QString);

        #[qinvokable]
        fn toggle_favorite(self: Pin<&mut Self>, path: &QString);

        #[qinvokable]
        fn load_favorites(self: Pin<&mut Self>);

        #[qinvokable]
        fn is_favorite(&self, path: &QString) -> bool;

        #[qinvokable]
        fn set_mode_for_workspace(self: Pin<&mut Self>, ws_id: i32, path: &QString, mode: &QString);

        #[qinvokable]
        fn save(self: Pin<&mut Self>);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct WallpaperBridgeRust {
    wallpaper_type: QString,
    wallpaper_path: QString,
    wallpaper_mode: QString,
    wallpaper_color: QString,
    current_workspace: i32,
    wallpaper_list: QString,
    favorites_list: QString,
}

impl qobject::WallpaperBridge {
    pub fn load_config(mut self: Pin<&mut Self>) {
        let config = xarph_sdk::config::XarphConfig::load();
        let ws = *self.current_workspace() as u8;
        let wallpaper = config.get_wallpaper_for_workspace(ws);

        match &wallpaper {
            xarph_sdk::config::WallpaperConfig::Image { path, mode } => {
                self.as_mut().set_wallpaper_type(QString::from("image"));
                self.as_mut().set_wallpaper_path(QString::from(path));
                self.as_mut().set_wallpaper_mode(QString::from(
                    xarph_sdk::wallpaper_engine::WallpaperEngine::content_fit(mode),
                ));
            }
            xarph_sdk::config::WallpaperConfig::Color { hex } => {
                self.as_mut().set_wallpaper_type(QString::from("color"));
                self.as_mut().set_wallpaper_color(QString::from(hex));
            }
            xarph_sdk::config::WallpaperConfig::Video { path, mode, loop_play: _ } => {
                self.as_mut().set_wallpaper_type(QString::from("video"));
                self.as_mut().set_wallpaper_path(QString::from(path));
                self.as_mut().set_wallpaper_mode(QString::from(
                    xarph_sdk::wallpaper_engine::WallpaperEngine::content_fit(mode),
                ));
            }
        }
    }

    pub fn set_wallpaper(mut self: Pin<&mut Self>, path: &QString, mode: &QString) {
        self.as_mut().set_wallpaper_path(path.clone());
        self.as_mut().set_wallpaper_mode(mode.clone());
        self.as_mut().set_wallpaper_type(QString::from("image"));
    }

    pub fn set_color(mut self: Pin<&mut Self>, color: &QString) {
        self.as_mut().set_wallpaper_color(color.clone());
        self.as_mut().set_wallpaper_type(QString::from("color"));
    }

    pub fn set_workspace(mut self: Pin<&mut Self>, workspace_id: i32) {
        self.as_mut().set_current_workspace(workspace_id);
        self.load_config();
    }

    pub fn get_content_fit(&self) -> QString {
        self.wallpaper_mode().clone()
    }

    pub fn is_video(&self) -> bool {
        self.wallpaper_type().to_string() == "video"
    }

    /// Returns wallpaper list as pipe-delimited string: "path|name|isVideo\n..."
    pub fn collect_wallpapers(mut self: Pin<&mut Self>, search: &QString) {
        let search_str = search.to_string();
        let search_opt = if search_str.is_empty() {
            None
        } else {
            Some(search_str.as_str())
        };
        let wallpapers =
            xarph_sdk::wallpaper_engine::WallpaperEngine::collect_wallpapers(search_opt, false);

        let lines: Vec<String> = wallpapers
            .iter()
            .map(|w| {
                format!(
                    "{}|{}|{}",
                    w.path.display(),
                    w.name,
                    if w.is_video { "1" } else { "0" }
                )
            })
            .collect();
        self.as_mut()
            .set_wallpaper_list(QString::from(&lines.join("\n")));
    }

    pub fn toggle_favorite(mut self: Pin<&mut Self>, path: &QString) {
        let path_str = path.to_string();
        if xarph_sdk::wallpaper_engine::WallpaperEngine::is_favorite(&path_str) {
            let _ = xarph_sdk::wallpaper_engine::WallpaperEngine::remove_favorite(&path_str);
        } else {
            let _ = xarph_sdk::wallpaper_engine::WallpaperEngine::add_favorite(&path_str);
        }
        self.load_favorites();
    }

    pub fn load_favorites(mut self: Pin<&mut Self>) {
        let favorites = xarph_sdk::wallpaper_engine::WallpaperEngine::load_favorites();
        self.as_mut()
            .set_favorites_list(QString::from(&favorites.join("\n")));
    }

    pub fn is_favorite(&self, path: &QString) -> bool {
        xarph_sdk::wallpaper_engine::WallpaperEngine::is_favorite(&path.to_string())
    }

    pub fn set_mode_for_workspace(
        mut self: Pin<&mut Self>,
        ws_id: i32,
        path: &QString,
        mode: &QString,
    ) {
        let path_str = path.to_string();
        let mode_str = mode.to_string();
        let wallpaper_mode = match mode_str.as_str() {
            "cover" => xarph_sdk::config::WallpaperMode::Fill,
            "contain" => xarph_sdk::config::WallpaperMode::Fit,
            "stretch" => xarph_sdk::config::WallpaperMode::Stretch,
            "center" => xarph_sdk::config::WallpaperMode::Center,
            "repeat" => xarph_sdk::config::WallpaperMode::Tile,
            _ => xarph_sdk::config::WallpaperMode::Fill,
        };

        let mut config = xarph_sdk::config::XarphConfig::load();
        if path_str.starts_with("#") || path_str.starts_with("rgb") {
            config.set_workspace_wallpaper(
                ws_id as u8,
                xarph_sdk::config::WallpaperConfig::Color {
                    hex: path_str.clone(),
                },
            );
        } else if xarph_sdk::wallpaper_engine::WallpaperEngine::is_video_file(
            std::path::Path::new(&path_str),
        ) {
            config.set_workspace_wallpaper(
                ws_id as u8,
            xarph_sdk::config::WallpaperConfig::Video {
                path: path_str.clone(),
                mode: wallpaper_mode.clone(),
                loop_play: true,
            },
            );
        } else {
            config.set_workspace_wallpaper(
                ws_id as u8,
                xarph_sdk::config::WallpaperConfig::Image {
                    path: path_str,
                    mode: wallpaper_mode,
                },
            );
        }

        let _ = config.save();
        self.as_mut().set_current_workspace(ws_id);
        self.load_config();
    }

    pub fn save(mut self: Pin<&mut Self>) {
        let mut config = xarph_sdk::config::XarphConfig::load();
        let ws = *self.current_workspace() as u8;
        let wallpaper_type = self.wallpaper_type().to_string();
        let path = self.wallpaper_path().to_string();
        let color = self.wallpaper_color().to_string();

        let wallpaper = match wallpaper_type.as_str() {
            "color" => xarph_sdk::config::WallpaperConfig::Color { hex: color },
            "video" => xarph_sdk::config::WallpaperConfig::Video {
                path,
                mode: xarph_sdk::config::WallpaperMode::Fill,
                loop_play: true,
            },
            _ => xarph_sdk::config::WallpaperConfig::Image {
                path,
                mode: xarph_sdk::config::WallpaperMode::Fill,
            },
        };

        config.set_workspace_wallpaper(ws, wallpaper);
        let _ = config.save();
    }
}
