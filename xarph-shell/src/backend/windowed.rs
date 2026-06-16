use super::ShellBackend;
use gtk4::ApplicationWindow;
use gtk4::prelude::*;
use xarph_sdk::config::PanelConfig;

pub struct WindowedBackend;

impl ShellBackend for WindowedBackend {
    fn setup_window(&self, window: &ApplicationWindow, panel: &PanelConfig) {
        // In Windowed mode we don't use LayerShell at all — the window
        // floats like any normal GTK app. Height is set by app::build_ui
        // to match the panel bar height (40 px).
        window.set_title(Some("Xarph Shell — Windowed Mode"));
        if panel.position.is_horizontal() {
            window.set_default_size(900, panel.size);
        } else {
            window.set_default_size(panel.size, 700);
        }
    }

    fn uses_layer_shell(&self) -> bool {
        false
    }
}
