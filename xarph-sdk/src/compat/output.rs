use crate::{Mode, Output};

/// Extension methods for [`Output`].
pub trait OutputExt {
    /// Returns the current mode, if any.
    fn current_mode(&self) -> Option<Mode>;

    /// Returns a resolution string like "1920x1080@60Hz".
    fn resolution_string(&self) -> String;

    /// Returns a human-readable display string like "DP-1 (Dell U2720Q)".
    fn display_string(&self) -> String;
}

impl OutputExt for Output {
    fn current_mode(&self) -> Option<Mode> {
        let idx = self.current_mode?;
        self.modes.get(idx).copied()
    }

    fn resolution_string(&self) -> String {
        let Some(mode) = self.current_mode() else {
            return "disabled".to_string();
        };
        let hz = mode.refresh_rate as f64 / 1000.0;
        format!("{}x{}@{:.0}Hz", mode.width, mode.height, hz)
    }

    fn display_string(&self) -> String {
        let res = self.resolution_string();
        if self.make.is_empty() && self.model.is_empty() {
            format!("{} ({})", self.name, res)
        } else {
            format!("{} {} {} ({})", self.name, self.make, self.model, res)
        }
    }
}
