/// Settings bridge: exposes settings to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, wallpaper_path)]
        #[qproperty(QString, wallpaper_mode)]
        #[qproperty(QString, wallpaper_color)]
        #[qproperty(QString, theme_name)]
        #[qproperty(i32, panel_count)]
        #[qproperty(QString, wallpaper_list)]
        #[namespace = "xarph"]
        type SettingsBridge = super::SettingsBridgeRust;

        #[qinvokable]
        fn load_settings(self: Pin<&mut Self>);

        #[qinvokable]
        fn save_settings(self: Pin<&mut Self>);

        #[qinvokable]
        fn choose_wallpaper(self: Pin<&mut Self>, path: &QString);

        #[qinvokable]
        fn choose_color(self: Pin<&mut Self>, color: &QString);

        #[qinvokable]
        fn apply_theme(self: Pin<&mut Self>, name: &QString);

        #[qinvokable]
        fn get_wallpaper_dirs(&self) -> QString;

        #[qinvokable]
        fn get_themes(&self) -> QString;

        #[qinvokable]
        fn collect_wallpapers(self: Pin<&mut Self>);

        #[qinvokable]
        fn change_wallpaper_mode(self: Pin<&mut Self>, mode: &QString);

        #[qinvokable]
        fn set_panel_visible(self: Pin<&mut Self>, widget_kind: &QString, visible: bool);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct SettingsBridgeRust {
    wallpaper_path: QString,
    wallpaper_mode: QString,
    wallpaper_color: QString,
    theme_name: QString,
    panel_count: i32,
    wallpaper_list: QString,
}

impl qobject::SettingsBridge {
    pub fn load_settings(mut self: Pin<&mut Self>) {
        let config = xarph_sdk::config::XarphConfig::load();

        let ws = 0u8;
        let wallpaper = config.get_wallpaper_for_workspace(ws);
        match &wallpaper {
            xarph_sdk::config::WallpaperConfig::Image { path, mode } => {
                self.as_mut().set_wallpaper_path(QString::from(path));
                self.as_mut().set_wallpaper_mode(QString::from(
                    xarph_sdk::wallpaper_engine::WallpaperEngine::content_fit(mode),
                ));
            }
            xarph_sdk::config::WallpaperConfig::Color { hex } => {
                self.as_mut().set_wallpaper_color(QString::from(hex));
            }
            xarph_sdk::config::WallpaperConfig::Video { path, mode, loop_play: _ } => {
                self.as_mut().set_wallpaper_path(QString::from(path));
                self.as_mut().set_wallpaper_mode(QString::from(
                    xarph_sdk::wallpaper_engine::WallpaperEngine::content_fit(mode),
                ));
            }
        }

        if let Some(name) = &config.theme {
            self.as_mut().set_theme_name(QString::from(name));
        }

        self.as_mut()
            .set_panel_count(config.shell.panels.len() as i32);
    }

    pub fn save_settings(mut self: Pin<&mut Self>) {
        let mut config = xarph_sdk::config::XarphConfig::load();
        let ws = 0u8;
        let path = self.wallpaper_path().to_string();
        let mode_str = self.wallpaper_mode().to_string();
        let color = self.wallpaper_color().to_string();

        let mode = match mode_str.as_str() {
            "cover" => xarph_sdk::config::WallpaperMode::Fill,
            "contain" => xarph_sdk::config::WallpaperMode::Fit,
            "stretch" => xarph_sdk::config::WallpaperMode::Stretch,
            "center" => xarph_sdk::config::WallpaperMode::Center,
            "repeat" => xarph_sdk::config::WallpaperMode::Tile,
            _ => xarph_sdk::config::WallpaperMode::Fill,
        };

        let wallpaper = if !color.is_empty() && path.is_empty() {
            xarph_sdk::config::WallpaperConfig::Color { hex: color }
        } else if !path.is_empty() && xarph_sdk::wallpaper_engine::WallpaperEngine::is_video_file(
            std::path::Path::new(&path),
        ) {
            xarph_sdk::config::WallpaperConfig::Video {
                path,
                mode,
                loop_play: true,
            }
        } else {
            xarph_sdk::config::WallpaperConfig::Image { path, mode }
        };

        config.set_workspace_wallpaper(ws, wallpaper);

        let theme = self.theme_name().to_string();
        if !theme.is_empty() && theme != "Default" {
            config.theme = Some(theme);
        }

        let _ = config.save();
    }

    pub fn choose_wallpaper(mut self: Pin<&mut Self>, path: &QString) {
        self.as_mut().set_wallpaper_path(path.clone());
        self.as_mut().set_wallpaper_color(QString::from(""));
        self.save_settings();
    }

    pub fn choose_color(mut self: Pin<&mut Self>, color: &QString) {
        self.as_mut().set_wallpaper_color(color.clone());
        self.as_mut().set_wallpaper_path(QString::from(""));
        self.save_settings();
    }

    pub fn apply_theme(mut self: Pin<&mut Self>, name: &QString) {
        self.as_mut().set_theme_name(name.clone());
        self.save_settings();
    }

    pub fn get_wallpaper_dirs(&self) -> QString {
        let dirs = xarph_sdk::wallpaper_engine::WallpaperEngine::get_wallpaper_dirs();
        let paths: Vec<&str> = dirs.iter().map(|p| p.to_str().unwrap_or("")).collect();
        QString::from(&paths.join(":"))
    }

    pub fn get_themes(&self) -> QString {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.config"))
            .join("xarph/themes");
        let mut themes = vec!["Default".to_string()];
        if let Ok(entries) = std::fs::read_dir(&config_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        themes.push(name.to_string());
                    }
                }
            }
        }
        themes.sort();
        themes.dedup();
        QString::from(&themes.join(","))
    }

    pub fn collect_wallpapers(mut self: Pin<&mut Self>) {
        let wallpapers =
            xarph_sdk::wallpaper_engine::WallpaperEngine::collect_wallpapers(None, false);
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

    pub fn change_wallpaper_mode(mut self: Pin<&mut Self>, mode: &QString) {
        self.as_mut().set_wallpaper_mode(mode.clone());
    }

    pub fn set_panel_visible(mut self: Pin<&mut Self>, widget_kind: &QString, visible: bool) {
        let kind_str = widget_kind.to_string();
        let widget_kind = match kind_str.as_str() {
            "clock" => xarph_sdk::config::WidgetKind::Clock,
            "workspaces" => xarph_sdk::config::WidgetKind::Workspaces,
            "tray" => xarph_sdk::config::WidgetKind::Tray,
            "start_button" => xarph_sdk::config::WidgetKind::StartButton,
            _ => return,
        };
        let model = xarph_sdk::settings_model::SettingsModel;
        let _ = model.set_panel_visible(&widget_kind, visible);
    }
}
