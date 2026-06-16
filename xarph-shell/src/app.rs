use gtk4::prelude::*;
use gtk4::{ApplicationWindow, CssProvider, gdk};
use xarph_sdk::config::{PanelConfig, XarphConfig};

use crate::panel;

const PANEL_CSS: &str = include_str!("style.css");
/// Loads the panel CSS into the GTK default display.
fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(PANEL_CSS);

    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

/// Builds and attaches the full shell UI to the given window.
/// Backend-agnostic: does not know or care whether the window is
/// floating (Windowed) or anchored via Wayland Layer Shell.
pub fn build_ui(
    window: &ApplicationWindow,
    config: &XarphConfig,
    panel_config: &PanelConfig,
    no_tray: bool,
) {
    load_css();

    // Apply GTK theme from config
    if let Some(settings) = gtk4::Settings::default() {
        if let Some(theme) = config.theme.as_deref() {
            settings.set_gtk_theme_name(Some(theme));
        }
    }

    let panel = panel::build_panel(config, panel_config, no_tray);

    if panel_config.position.is_horizontal() {
        panel.set_height_request(panel_config.size);
    } else {
        panel.set_width_request(panel_config.size);
    }
    panel.set_hexpand(true);

    window.set_child(Some(&panel));

    if panel_config.position.is_horizontal() {
        window.set_default_size(-1, panel_config.size);
    } else {
        window.set_default_size(panel_config.size, -1);
    }
    window.set_resizable(false);
}
