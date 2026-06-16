use crate::Window;

/// Extension methods for [`Window`].
pub trait WindowExt {
    /// Returns a human-readable display name for the window.
    ///
    /// Prefers `app_id`, then `title`, then a fallback.
    fn display_name(&self) -> &str;

    /// Returns the title or app_id for display purposes.
    fn label(&self) -> &str;

    /// Returns whether the window belongs to the given output.
    fn is_on_output(&self, output_name: &str) -> bool;
}

impl WindowExt for Window {
    fn display_name(&self) -> &str {
        self.app_id
            .as_deref()
            .or(self.title.as_deref())
            .unwrap_or("(unnamed)")
    }

    fn label(&self) -> &str {
        self.title
            .as_deref()
            .or(self.app_id.as_deref())
            .unwrap_or("(unnamed)")
    }

    fn is_on_output(&self, _output_name: &str) -> bool {
        // Window doesn't directly carry output info;
        // workspace_id can be cross-referenced with workspace output.
        false
    }
}
