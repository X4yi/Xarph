/// Context menu bridge: exposes menu items to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, item_id)]
        #[qproperty(QString, label)]
        #[qproperty(QString, icon)]
        #[qproperty(QString, shortcut)]
        #[qproperty(bool, enabled)]
        #[qproperty(QString, item_type)]
        #[namespace = "xarph"]
        type MenuItemBridge = super::MenuItemBridgeRust;

        #[qinvokable]
        fn trigger(&self);
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, menu_type)]
        #[qproperty(QString, object_name)]
        #[qproperty(i32, item_count)]
        #[namespace = "xarph"]
        type ContextMenuBridge = super::ContextMenuBridgeRust;

        #[qinvokable]
        fn build_menu(self: Pin<&mut Self>, object_type: &QString, object_name: &QString);

        #[qinvokable]
        fn get_item_count(&self) -> i32;

        #[qinvokable]
        fn get_item_id(&self, index: i32) -> QString;

        #[qinvokable]
        fn get_item_label(&self, index: i32) -> QString;

        #[qinvokable]
        fn get_item_icon(&self, index: i32) -> QString;

        #[qinvokable]
        fn get_item_shortcut(&self, index: i32) -> QString;

        #[qinvokable]
        fn is_item_enabled(&self, index: i32) -> bool;

        #[qinvokable]
        fn execute_action(self: Pin<&mut Self>, action_id: &QString);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;
use std::sync::Mutex;

static MENU_ITEMS: Mutex<Vec<xarph_sdk::context_menu::MenuItem>> = Mutex::new(Vec::new());

#[derive(Default)]
pub struct MenuItemBridgeRust {
    item_id: QString,
    label: QString,
    icon: QString,
    shortcut: QString,
    enabled: bool,
    item_type: QString,
}

#[derive(Default)]
pub struct ContextMenuBridgeRust {
    menu_type: QString,
    object_name: QString,
    item_count: i32,
}

impl qobject::MenuItemBridge {
    pub fn trigger(&self) {
        let id = self.item_id().to_string();
        println!("Menu item triggered: {}", id);
    }
}

impl qobject::ContextMenuBridge {
    pub fn build_menu(mut self: Pin<&mut Self>, object_type: &QString, object_name: &QString) {
        let obj_type_str = object_type.to_string();
        let obj_name_str = object_name.to_string();

        let items = match obj_type_str.as_str() {
            "file" => xarph_sdk::context_menu::build_file_menu(&obj_name_str),
            "folder" => xarph_sdk::context_menu::build_folder_menu(&obj_name_str),
            "application" => xarph_sdk::context_menu::build_application_menu(&obj_name_str),
            "project" => xarph_sdk::context_menu::build_project_menu(&obj_name_str),
            "shortcut" => xarph_sdk::context_menu::build_shortcut_menu(&obj_name_str),
            "widget" => xarph_sdk::context_menu::build_widget_menu(&obj_name_str),
            "desktop" => xarph_sdk::context_menu::build_desktop_context_menu(),
            _ => vec![],
        };

        let count = items.len() as i32;
        self.as_mut().set_menu_type(object_type.clone());
        self.as_mut().set_object_name(object_name.clone());
        self.as_mut().set_item_count(count);

        if let Ok(mut menu_items) = MENU_ITEMS.lock() {
            *menu_items = items;
        }
    }

    pub fn get_item_count(&self) -> i32 {
        *self.item_count()
    }

    fn get_item(&self, index: i32) -> Option<xarph_sdk::context_menu::MenuItem> {
        let idx = index as usize;
        MENU_ITEMS
            .lock()
            .ok()
            .and_then(|items| items.get(idx).cloned())
    }

    pub fn get_item_id(&self, index: i32) -> QString {
        let item = match self.get_item(index) {
            Some(item) => item,
            None => return QString::from(""),
        };
        let id = match &item {
            xarph_sdk::context_menu::MenuItem::Action { id, .. } => id.clone(),
            xarph_sdk::context_menu::MenuItem::Separator => "---".to_string(),
            xarph_sdk::context_menu::MenuItem::Submenu { label, .. } => {
                format!("submenu_{}", label)
            }
            xarph_sdk::context_menu::MenuItem::Info { label } => {
                format!("info_{}", label)
            }
        };
        QString::from(&id)
    }

    pub fn get_item_label(&self, index: i32) -> QString {
        let item = match self.get_item(index) {
            Some(item) => item,
            None => return QString::from(""),
        };
        let label = match &item {
            xarph_sdk::context_menu::MenuItem::Action { label, .. } => label.clone(),
            xarph_sdk::context_menu::MenuItem::Separator => "---".to_string(),
            xarph_sdk::context_menu::MenuItem::Submenu { label, .. } => label.clone(),
            xarph_sdk::context_menu::MenuItem::Info { label } => label.clone(),
        };
        QString::from(&label)
    }

    pub fn get_item_icon(&self, index: i32) -> QString {
        let item = match self.get_item(index) {
            Some(item) => item,
            None => return QString::from(""),
        };
        let icon = match &item {
            xarph_sdk::context_menu::MenuItem::Action { icon, .. } => {
                icon.clone().unwrap_or_default()
            }
            xarph_sdk::context_menu::MenuItem::Separator => String::new(),
            xarph_sdk::context_menu::MenuItem::Submenu { icon, .. } => {
                icon.clone().unwrap_or_default()
            }
            xarph_sdk::context_menu::MenuItem::Info { .. } => String::new(),
        };
        QString::from(&icon)
    }

    pub fn get_item_shortcut(&self, index: i32) -> QString {
        let item = match self.get_item(index) {
            Some(item) => item,
            None => return QString::from(""),
        };
        let shortcut = match &item {
            xarph_sdk::context_menu::MenuItem::Action { shortcut, .. } => {
                shortcut.clone().unwrap_or_default()
            }
            _ => String::new(),
        };
        QString::from(&shortcut)
    }

    pub fn is_item_enabled(&self, index: i32) -> bool {
        let item = match self.get_item(index) {
            Some(item) => item,
            None => return false,
        };
        match &item {
            xarph_sdk::context_menu::MenuItem::Action { enabled, .. } => *enabled,
            xarph_sdk::context_menu::MenuItem::Separator => false,
            _ => true,
        }
    }

    pub fn execute_action(self: Pin<&mut Self>, action_id: &QString) {
        let id = action_id.to_string();
        let obj_type = self.menu_type().to_string();
        let obj_name = self.object_name().to_string();

        match id.as_str() {
            "open" => {
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("xdg-open")
                        .arg(&obj_name)
                        .spawn();
                });
            }
            "open_in_terminal" => {
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("x-terminal-emulator").spawn();
                });
            }
            "copy" => {
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("wl-copy")
                        .arg(&obj_name)
                        .spawn();
                });
            }
            "cut" => {
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("wl-copy")
                        .arg("-t")
                        .arg("text/uri-list")
                        .spawn();
                });
            }
            "paste" => {
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("wl-paste").spawn();
                });
            }
            "delete" => {
                std::thread::spawn(move || {
                    if obj_type == "file" || obj_type == "folder" {
                        let _ = std::process::Command::new("gio")
                            .args(["trash", &obj_name])
                            .spawn();
                    }
                });
            }
            "rename" => {
                // Handled via dialog in QML
            }
            "properties" => {
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("xdg-open")
                        .arg(&obj_name)
                        .spawn();
                });
            }
            "new_folder" => {
                std::thread::spawn(move || {
                    let dir = dirs::home_dir()
                        .unwrap_or_else(|| std::path::PathBuf::from("~"))
                        .join("Desktop");
                    let _ = std::fs::create_dir(dir.join("New Folder"));
                });
            }
            "new_file" => {
                std::thread::spawn(move || {
                    let dir = dirs::home_dir()
                        .unwrap_or_else(|| std::path::PathBuf::from("~"))
                        .join("Desktop");
                    let _ = std::fs::write(dir.join("New File.txt"), "");
                });
            }
            "change_wallpaper" => {
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("xdg-open")
                        .arg("xarph-settings")
                        .spawn();
                });
            }
            "desktop_settings" | "display_settings" => {
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("xdg-open")
                        .arg("xarph-settings")
                        .spawn();
                });
            }
            "open_terminal" => {
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("x-terminal-emulator").spawn();
                });
            }
            "launch" => {
                std::thread::spawn(move || {
                    let _ = std::process::Command::new("xdg-open")
                        .arg(&obj_name)
                        .spawn();
                });
            }
            _ => {
                println!("Unknown action: {}", id);
            }
        }
    }
}
