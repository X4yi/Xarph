/// Tray bridge: exposes system tray items to QML
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
        #[qproperty(QString, icon_name)]
        #[qproperty(QString, title)]
        #[qproperty(QString, status)]
        #[namespace = "xarph"]
        type TrayItemBridge = super::TrayItemBridgeRust;

        #[qinvokable]
        fn activate(&self);

        #[qinvokable]
        fn secondary_activate(&self);
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, item_count)]
        #[namespace = "xarph"]
        type TrayModelBridge = super::TrayModelBridgeRust;

        #[qinvokable]
        fn refresh(self: Pin<&mut Self>);

        #[qinvokable]
        fn get_item_count(&self) -> i32;
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct TrayItemBridgeRust {
    item_id: QString,
    icon_name: QString,
    title: QString,
    status: QString,
}

#[derive(Default)]
pub struct TrayModelBridgeRust {
    item_count: i32,
}

impl qobject::TrayItemBridge {
    pub fn activate(&self) {
        // TODO: Implement StatusNotifier activation
        println!("Tray item activated: {}", self.item_id().to_string());
    }

    pub fn secondary_activate(&self) {
        // TODO: Implement StatusNotifier secondary activation
        println!("Tray item secondary activated: {}", self.item_id().to_string());
    }
}

impl qobject::TrayModelBridge {
    pub fn refresh(mut self: Pin<&mut Self>) {
        self.as_mut().set_item_count(0);
    }

    pub fn get_item_count(&self) -> i32 {
        *self.item_count()
    }
}
