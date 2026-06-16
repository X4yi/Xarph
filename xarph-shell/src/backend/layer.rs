use super::ShellBackend;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, gdk};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use xarph_sdk::config::{PanelAnchor, PanelConfig, PanelPosition};

pub struct LayerShellBackend;

impl ShellBackend for LayerShellBackend {
    fn setup_window(&self, window: &ApplicationWindow, panel: &PanelConfig) {
        window.init_layer_shell();
        window.set_layer(Layer::Top);

        for edge in [Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
            window.set_anchor(edge, false);
        }

        match panel.position {
            PanelPosition::Top => {
                window.set_anchor(Edge::Top, true);
                set_horizontal_anchor(window, panel.anchor);
            }
            PanelPosition::Bottom => {
                window.set_anchor(Edge::Bottom, true);
                set_horizontal_anchor(window, panel.anchor);
            }
            PanelPosition::Left => {
                window.set_anchor(Edge::Left, true);
                set_vertical_anchor(window, panel.anchor);
            }
            PanelPosition::Right => {
                window.set_anchor(Edge::Right, true);
                set_vertical_anchor(window, panel.anchor);
            }
        }

        if let Some(zone) = panel.exclusive_zone {
            window.set_exclusive_zone(zone);
        } else {
            window.auto_exclusive_zone_enable();
        }

        if let Some(output) = panel.output.as_deref().filter(|s| !s.is_empty()) {
            if let Some(monitor) = find_monitor(output) {
                window.set_monitor(Some(&monitor));
            }
        }
    }

    fn uses_layer_shell(&self) -> bool {
        true
    }
}

fn set_horizontal_anchor(window: &ApplicationWindow, anchor: PanelAnchor) {
    match anchor {
        PanelAnchor::Fill => {
            window.set_anchor(Edge::Left, true);
            window.set_anchor(Edge::Right, true);
        }
        PanelAnchor::Start => window.set_anchor(Edge::Left, true),
        PanelAnchor::End => window.set_anchor(Edge::Right, true),
    }
}

fn set_vertical_anchor(window: &ApplicationWindow, anchor: PanelAnchor) {
    match anchor {
        PanelAnchor::Fill => {
            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Bottom, true);
        }
        PanelAnchor::Start => window.set_anchor(Edge::Top, true),
        PanelAnchor::End => window.set_anchor(Edge::Bottom, true),
    }
}

fn find_monitor(name: &str) -> Option<gdk::Monitor> {
    let display = gdk::Display::default()?;
    let monitors = display.monitors();
    for i in 0..monitors.n_items() {
        let Some(obj) = monitors.item(i) else {
            continue;
        };
        let Ok(monitor) = obj.downcast::<gdk::Monitor>() else {
            continue;
        };
        let connector = monitor.connector().map(|s| s.to_string());
        let model = monitor.model().map(|s| s.to_string());
        if connector.as_deref() == Some(name) || model.as_deref() == Some(name) {
            return Some(monitor);
        }
    }
    None
}
