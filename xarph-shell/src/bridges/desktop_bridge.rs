/// Desktop bridge: exposes desktop objects to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, object_id)]
        #[qproperty(QString, object_type)]
        #[qproperty(QString, display_name)]
        #[qproperty(QString, icon_name)]
        #[qproperty(f64, x)]
        #[qproperty(f64, y)]
        #[qproperty(f64, width)]
        #[qproperty(f64, height)]
        #[qproperty(i32, z_index)]
        #[qproperty(bool, selected)]
        #[namespace = "xarph"]
        type DesktopObjectBridge = super::DesktopObjectBridgeRust;

        #[qinvokable]
        fn set_position(self: Pin<&mut Self>, x: f64, y: f64);

        #[qinvokable]
        fn get_metadata(&self, key: &QString) -> QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, object_count)]
        #[namespace = "xarph"]
        type DesktopModelBridge = super::DesktopModelBridgeRust;

        #[qinvokable]
        fn load_objects(self: Pin<&mut Self>);

        #[qinvokable]
        fn save_objects(&self);

        #[qinvokable]
        fn add_object(
            self: Pin<&mut Self>,
            name: &QString,
            obj_type: &QString,
            path: &QString,
            x: f64,
            y: f64,
        );

        #[qinvokable]
        fn remove_object(self: Pin<&mut Self>, id: &QString);

        #[qinvokable]
        fn get_object_count(&self) -> i32;
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;
use std::collections::HashMap;

#[derive(Default)]
pub struct DesktopObjectBridgeRust {
    object_id: QString,
    object_type: QString,
    display_name: QString,
    icon_name: QString,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    z_index: i32,
    selected: bool,
    metadata: HashMap<String, String>,
}

#[derive(Default)]
pub struct DesktopModelBridgeRust {
    object_count: i32,
}

impl qobject::DesktopObjectBridge {
    pub fn set_position(mut self: Pin<&mut Self>, x: f64, y: f64) {
        self.as_mut().set_x(x);
        self.as_mut().set_y(y);
    }

    pub fn get_metadata(&self, key: &QString) -> QString {
        let key_str = key.to_string();
        let value = self
            .metadata
            .get(&key_str)
            .map(|s| s.as_str())
            .unwrap_or("");
        QString::from(value)
    }
}

impl qobject::DesktopModelBridge {
    pub fn load_objects(self: Pin<&mut Self>) {
        let mut registry = xarph_sdk::desktop_registry::DesktopRegistry::with_default_path();
        let _ = registry.load();
        let count = registry.len() as i32;
        self.set_object_count(count);
    }

    pub fn save_objects(&self) {
    }

    pub fn add_object(
        mut self: Pin<&mut Self>,
        name: &QString,
        obj_type: &QString,
        path: &QString,
        x: f64,
        y: f64,
    ) {
        let name_str = name.to_string();
        let path_str = path.to_string();
        let obj_type_str = obj_type.to_string();
        let _obj = match obj_type_str.as_str() {
            "file" => xarph_sdk::desktop_object::new_file(&name_str, &path_str, x, y),
            "folder" => xarph_sdk::desktop_object::new_folder(&name_str, &path_str, x, y),
            "application" => xarph_sdk::desktop_object::new_application(&name_str, &path_str, x, y),
            "project" => xarph_sdk::desktop_object::new_project(&name_str, &path_str, x, y),
            _ => return,
        };
        let count = *self.object_count() + 1;
        self.as_mut().set_object_count(count);
    }

    pub fn remove_object(mut self: Pin<&mut Self>, _id: &QString) {
        let count = *self.object_count() - 1;
        self.as_mut().set_object_count(count.max(0));
    }

    pub fn get_object_count(&self) -> i32 {
        *self.object_count()
    }
}
