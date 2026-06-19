use serde::{Deserialize, Serialize};

/// Zone on the desktop where an object can be placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DesktopZone {
    Main,
    Dock,
    Panel,
    Notification,
}

/// Position of a desktop object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesktopObjectPosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub workspace_id: Option<u8>,
    pub container_id: Option<String>,
    pub zone: DesktopZone,
    pub z_index: i32,
}

impl DesktopObjectPosition {
    /// Create a position on the main desktop zone.
    pub fn desktop(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            width: 80.0,
            height: 80.0,
            workspace_id: None,
            container_id: None,
            zone: DesktopZone::Main,
            z_index: 0,
        }
    }

    /// Set the size of the object.
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Place the object on a specific workspace.
    pub fn on_workspace(mut self, workspace_id: u8) -> Self {
        self.workspace_id = Some(workspace_id);
        self
    }

    /// Set the zone.
    pub fn in_zone(mut self, zone: DesktopZone) -> Self {
        self.zone = zone;
        self
    }

    /// Set the z-index.
    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = z_index;
        self
    }
}

impl Default for DesktopObjectPosition {
    fn default() -> Self {
        Self::desktop(0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desktop_position() {
        let pos = DesktopObjectPosition::desktop(100.0, 200.0);
        assert_eq!(pos.x, 100.0);
        assert_eq!(pos.y, 200.0);
        assert_eq!(pos.zone, DesktopZone::Main);
        assert_eq!(pos.workspace_id, None);
    }

    #[test]
    fn test_with_size() {
        let pos = DesktopObjectPosition::desktop(0.0, 0.0).with_size(200.0, 100.0);
        assert_eq!(pos.width, 200.0);
        assert_eq!(pos.height, 100.0);
    }

    #[test]
    fn test_on_workspace() {
        let pos = DesktopObjectPosition::desktop(0.0, 0.0).on_workspace(3);
        assert_eq!(pos.workspace_id, Some(3));
    }
}
