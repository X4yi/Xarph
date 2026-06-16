pub mod layer;
pub mod windowed;

use gtk4::ApplicationWindow;
use xarph_sdk::config::PanelConfig;

/// A generic trait for a shell backend to abstract away the windowing system.
pub trait ShellBackend {
    /// Configures the given window according to the backend's capabilities.
    fn setup_window(&self, window: &ApplicationWindow, panel: &PanelConfig);

    /// Whether this backend uses the Wayland Layer Shell protocol.
    fn uses_layer_shell(&self) -> bool;
}
