use crate::toggle_service;

/// Start menu bridge: exposes app registry to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, app_name)]
        #[qproperty(QString, app_icon)]
        #[qproperty(QString, desktop_file)]
        #[qproperty(QString, categories)]
        #[qproperty(bool, is_recent)]
        #[namespace = "xarph"]
        type AppEntryBridge = super::AppEntryBridgeRust;

        #[qinvokable]
        fn launch(&self);

        #[qinvokable]
        fn get_display_name(&self) -> QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, app_count)]
        #[qproperty(QString, search_query)]
        #[namespace = "xarph"]
        type StartMenuBridge = super::StartMenuBridgeRust;

        #[qinvokable]
        fn load_apps(self: Pin<&mut Self>);

        #[qinvokable]
        fn search(&self, query: &QString) -> QString;

        #[qinvokable]
        fn get_all_apps(&self) -> QString;

        #[qinvokable]
        fn launch_app(self: Pin<&mut Self>, desktop_file: &QString);

        #[qinvokable]
        fn launch_binary(self: Pin<&mut Self>, name: &QString);

        #[qinvokable]
        fn get_app_count(&self) -> i32;

        #[qinvokable]
        fn check_toggle(&self) -> bool;
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct AppEntryBridgeRust {
    app_name: QString,
    app_icon: QString,
    desktop_file: QString,
    categories: QString,
    is_recent: bool,
}

#[derive(Default)]
pub struct StartMenuBridgeRust {
    app_count: i32,
    search_query: QString,
}

impl qobject::AppEntryBridge {
    pub fn launch(&self) {
        let desktop_file = self.desktop_file().to_string();
        std::thread::spawn(move || {
            let _ = std::process::Command::new("xdg-open")
                .arg(&desktop_file)
                .spawn();
        });
    }

    pub fn get_display_name(&self) -> QString {
        self.app_name().clone()
    }
}

impl qobject::StartMenuBridge {
    pub fn load_apps(mut self: Pin<&mut Self>) {
        let registry = xarph_sdk::app_registry::AppRegistry::load();
        self.as_mut().set_app_count(registry.len() as i32);
    }

    pub fn get_all_apps(&self) -> QString {
        let registry = xarph_sdk::app_registry::AppRegistry::load();
        let lines: Vec<String> = registry
            .all()
            .iter()
            .map(|app| {
                format!(
                    "{}|{}|{}|{}",
                    app.id,
                    app.name,
                    app.exec.as_deref().unwrap_or(""),
                    app.categories.join(",")
                )
            })
            .collect();
        QString::from(&lines.join("\n"))
    }

    pub fn search(&self, query: &QString) -> QString {
        let registry = xarph_sdk::app_registry::AppRegistry::load();
        let query_str = query.to_string();
        let results = registry.search(&query_str);
        let lines: Vec<String> = results
            .iter()
            .map(|app| {
                format!(
                    "{}|{}|{}|{}",
                    app.id,
                    app.name,
                    app.exec.as_deref().unwrap_or(""),
                    app.categories.join(",")
                )
            })
            .collect();
        QString::from(&lines.join("\n"))
    }

    pub fn launch_app(self: Pin<&mut Self>, desktop_file: &QString) {
        let path = desktop_file.to_string();
        std::thread::spawn(move || {
            let _ = std::process::Command::new("xdg-open")
                .arg(&path)
                .spawn();
        });
    }

    pub fn launch_binary(self: Pin<&mut Self>, name: &QString) {
        let name_str = name.to_string();
        std::thread::spawn(move || {
            let _ = std::process::Command::new(&name_str)
                .spawn();
        });
    }

    pub fn get_app_count(&self) -> i32 {
        *self.app_count()
    }

    pub fn check_toggle(&self) -> bool {
        toggle_service::check_toggle()
    }
}
