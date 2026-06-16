pub mod clock;
pub mod network;
pub mod start_button;
pub mod system;
pub mod tray;
pub mod workspace;

use gtk4::Widget;

/// Generic contract that all Xarph Shell widgets must fulfil.
/// A widget is responsible for building its own GTK subtree and
/// managing any background threads or timers it needs internally.
pub trait ShellWidget {
    /// Construct and return the root GTK widget for this component.
    /// The returned widget can be placed anywhere in the panel layout.
    fn build(&self) -> Widget;
}
