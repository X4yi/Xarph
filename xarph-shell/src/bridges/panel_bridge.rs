/// Panel bridge: exposes panel widgets to QML
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, widget_id)]
        #[qproperty(QString, widget_type)]
        #[qproperty(QString, label)]
        #[qproperty(bool, visible)]
        #[qproperty(QString, section)]
        #[namespace = "xarph"]
        type PanelWidgetBridge = super::PanelWidgetBridgeRust;

        #[qinvokable]
        fn toggle_visibility(self: Pin<&mut Self>);

        #[qinvokable]
        fn get_label(&self) -> QString;
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, panel_id)]
        #[qproperty(QString, position)]
        #[qproperty(i32, widget_count)]
        #[namespace = "xarph"]
        type PanelBridge = super::PanelBridgeRust;

        #[qinvokable]
        fn load_panel_config(self: Pin<&mut Self>);

        #[qinvokable]
        fn get_widgets(&self) -> QString;
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct PanelWidgetBridgeRust {
    widget_id: QString,
    widget_type: QString,
    label: QString,
    visible: bool,
    section: QString,
}

#[derive(Default)]
pub struct PanelBridgeRust {
    panel_id: QString,
    position: QString,
    widget_count: i32,
}

impl qobject::PanelWidgetBridge {
    pub fn toggle_visibility(self: Pin<&mut Self>) {
        let current = *self.visible();
        self.set_visible(!current);
    }

    pub fn get_label(&self) -> QString {
        self.label().clone()
    }
}

impl qobject::PanelBridge {
    pub fn load_panel_config(mut self: Pin<&mut Self>) {
        let config = xarph_sdk::config::XarphConfig::load();
        if let Some(panel) = config.shell.panels.first() {
            self.as_mut().set_panel_id(QString::from(&panel.id));
            let pos = QString::from(match panel.position {
                xarph_sdk::config::PanelPosition::Top => "top",
                xarph_sdk::config::PanelPosition::Bottom => "bottom",
                xarph_sdk::config::PanelPosition::Left => "left",
                xarph_sdk::config::PanelPosition::Right => "right",
            });
            self.as_mut().set_position(pos);
            self.as_mut().set_widget_count(panel.widgets.len() as i32);
        }
    }

    pub fn get_widgets(&self) -> QString {
        QString::from("[]")
    }
}
