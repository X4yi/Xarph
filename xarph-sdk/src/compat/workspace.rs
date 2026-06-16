use crate::Workspace;

/// Extension methods for [`Workspace`].
pub trait WorkspaceExt {
    /// Returns a display label for the workspace.
    ///
    /// Uses `name` if set, otherwise formats the index.
    fn label(&self) -> String;

    /// Returns whether the workspace is visible (active) on any output.
    fn is_visible(&self) -> bool;
}

impl WorkspaceExt for Workspace {
    fn label(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| format!("{}", self.idx))
    }

    fn is_visible(&self) -> bool {
        self.is_active
    }
}
