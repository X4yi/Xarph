use crate::position::DesktopObjectPosition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ObjectData {
    File {
        id: String,
        name: String,
        path: String,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
    Folder {
        id: String,
        name: String,
        path: String,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
    Application {
        id: String,
        name: String,
        desktop_file: String,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
    Project {
        id: String,
        name: String,
        path: String,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
    Shortcut {
        id: String,
        name: String,
        target: String,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
    Widget {
        id: String,
        name: String,
        wtype: String,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
    WorkspaceObject {
        id: String,
        name: String,
        workspace_id: u8,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
    VolumeObject {
        id: String,
        name: String,
        mount_path: String,
        filesystem: String,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
    RecentObject {
        id: String,
        name: String,
        path: String,
        accessed_at: i64,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
    TrashObject {
        id: String,
        name: String,
        item_count: u32,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
    NetworkObject {
        id: String,
        name: String,
        uri: String,
        protocol: String,
        position: DesktopObjectPosition,
        #[serde(default)]
        metadata: HashMap<String, String>,
    },
}

impl ObjectData {
    pub fn id(&self) -> &str {
        match self {
            ObjectData::File { id, .. }
            | ObjectData::Folder { id, .. }
            | ObjectData::Application { id, .. }
            | ObjectData::Project { id, .. }
            | ObjectData::Shortcut { id, .. }
            | ObjectData::Widget { id, .. }
            | ObjectData::WorkspaceObject { id, .. }
            | ObjectData::VolumeObject { id, .. }
            | ObjectData::RecentObject { id, .. }
            | ObjectData::TrashObject { id, .. }
            | ObjectData::NetworkObject { id, .. } => id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ObjectData::File { name, .. }
            | ObjectData::Folder { name, .. }
            | ObjectData::Application { name, .. }
            | ObjectData::Project { name, .. }
            | ObjectData::Shortcut { name, .. }
            | ObjectData::Widget { name, .. }
            | ObjectData::WorkspaceObject { name, .. }
            | ObjectData::VolumeObject { name, .. }
            | ObjectData::RecentObject { name, .. }
            | ObjectData::TrashObject { name, .. }
            | ObjectData::NetworkObject { name, .. } => name,
        }
    }

    pub fn object_type(&self) -> &str {
        match self {
            ObjectData::File { .. } => "file",
            ObjectData::Folder { .. } => "folder",
            ObjectData::Application { .. } => "application",
            ObjectData::Project { .. } => "project",
            ObjectData::Shortcut { .. } => "shortcut",
            ObjectData::Widget { .. } => "widget",
            ObjectData::WorkspaceObject { .. } => "workspace_object",
            ObjectData::VolumeObject { .. } => "volume",
            ObjectData::RecentObject { .. } => "recent",
            ObjectData::TrashObject { .. } => "trash",
            ObjectData::NetworkObject { .. } => "network",
        }
    }

    pub fn position(&self) -> &DesktopObjectPosition {
        match self {
            ObjectData::File { position, .. }
            | ObjectData::Folder { position, .. }
            | ObjectData::Application { position, .. }
            | ObjectData::Project { position, .. }
            | ObjectData::Shortcut { position, .. }
            | ObjectData::Widget { position, .. }
            | ObjectData::WorkspaceObject { position, .. }
            | ObjectData::VolumeObject { position, .. }
            | ObjectData::RecentObject { position, .. }
            | ObjectData::TrashObject { position, .. }
            | ObjectData::NetworkObject { position, .. } => position,
        }
    }

    pub fn position_mut(&mut self) -> &mut DesktopObjectPosition {
        match self {
            ObjectData::File { position, .. }
            | ObjectData::Folder { position, .. }
            | ObjectData::Application { position, .. }
            | ObjectData::Project { position, .. }
            | ObjectData::Shortcut { position, .. }
            | ObjectData::Widget { position, .. }
            | ObjectData::WorkspaceObject { position, .. }
            | ObjectData::VolumeObject { position, .. }
            | ObjectData::RecentObject { position, .. }
            | ObjectData::TrashObject { position, .. }
            | ObjectData::NetworkObject { position, .. } => position,
        }
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        match self {
            ObjectData::File { metadata, .. }
            | ObjectData::Folder { metadata, .. }
            | ObjectData::Application { metadata, .. }
            | ObjectData::Project { metadata, .. }
            | ObjectData::Shortcut { metadata, .. }
            | ObjectData::Widget { metadata, .. }
            | ObjectData::WorkspaceObject { metadata, .. }
            | ObjectData::VolumeObject { metadata, .. }
            | ObjectData::RecentObject { metadata, .. }
            | ObjectData::TrashObject { metadata, .. }
            | ObjectData::NetworkObject { metadata, .. } => metadata,
        }
    }

    pub fn metadata_mut(&mut self) -> &mut HashMap<String, String> {
        match self {
            ObjectData::File { metadata, .. }
            | ObjectData::Folder { metadata, .. }
            | ObjectData::Application { metadata, .. }
            | ObjectData::Project { metadata, .. }
            | ObjectData::Shortcut { metadata, .. }
            | ObjectData::Widget { metadata, .. }
            | ObjectData::WorkspaceObject { metadata, .. }
            | ObjectData::VolumeObject { metadata, .. }
            | ObjectData::RecentObject { metadata, .. }
            | ObjectData::TrashObject { metadata, .. }
            | ObjectData::NetworkObject { metadata, .. } => metadata,
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            ObjectData::VolumeObject { name, mount_path, .. } => {
                format!("{} ({})", name, mount_path)
            }
            ObjectData::TrashObject { name, item_count, .. } => {
                if *item_count == 0 {
                    name.clone()
                } else {
                    format!("{} ({})", name, item_count)
                }
            }
            ObjectData::NetworkObject { name, protocol, .. } => {
                format!("{} [{}]", name, protocol)
            }
            _ => self.name().to_string(),
        }
    }

    pub fn icon_name(&self) -> String {
        match self {
            ObjectData::File { name, .. } => {
                let ext = name.rsplit('.').next().unwrap_or("");
                match ext.to_lowercase().as_str() {
                    "png" | "jpg" | "jpeg" | "webp" | "avif" | "gif" => {
                        "image-x-generic-symbolic".to_string()
                    }
                    "mp4" | "webm" | "mkv" | "mov" => "video-x-generic-symbolic".to_string(),
                    "mp3" | "flac" | "ogg" | "wav" => "audio-x-generic-symbolic".to_string(),
                    "pdf" => "application-pdf-symbolic".to_string(),
                    "zip" | "tar" | "gz" | "xz" | "7z" => "application-x-archive-symbolic".to_string(),
                    "rs" | "py" | "js" | "ts" | "go" | "c" | "cpp" => {
                        "text-x-script-symbolic".to_string()
                    }
                    _ => "text-x-generic-symbolic".to_string(),
                }
            }
            ObjectData::Folder { .. } => "folder-symbolic".to_string(),
            ObjectData::Application { .. } => "application-x-executable-symbolic".to_string(),
            ObjectData::Project { .. } => "folder-git-symbolic".to_string(),
            ObjectData::Shortcut { .. } => "emblem-symbolic-link-symbolic".to_string(),
            ObjectData::Widget { wtype, .. } => match wtype.as_str() {
                "MiniClock" => "alarm-symbolic".to_string(),
                "Calendar" => "calendar-symbolic".to_string(),
                "SystemMonitor" => "utilities-system-monitor-symbolic".to_string(),
                _ => "x-office-spreadsheet-symbolic".to_string(),
            },
            ObjectData::WorkspaceObject { .. } => "view-paged-symbolic".to_string(),
            ObjectData::VolumeObject { filesystem, .. } => {
                if filesystem.starts_with("fuse") || filesystem == "tmpfs" {
                    "drive-removable-media-symbolic".to_string()
                } else {
                    "drive-harddisk-symbolic".to_string()
                }
            }
            ObjectData::RecentObject { .. } => "document-open-recent-symbolic".to_string(),
            ObjectData::TrashObject { .. } => "user-trash-symbolic".to_string(),
            ObjectData::NetworkObject { protocol, .. } => match protocol.as_str() {
                "smb" | "sftp" | "ssh" => "network-server-symbolic".to_string(),
                _ => "network-workgroup-symbolic".to_string(),
            },
        }
    }

    pub fn set_position(&mut self, new_pos: DesktopObjectPosition) {
        match self {
            ObjectData::File { position, .. }
            | ObjectData::Folder { position, .. }
            | ObjectData::Application { position, .. }
            | ObjectData::Project { position, .. }
            | ObjectData::Shortcut { position, .. }
            | ObjectData::Widget { position, .. }
            | ObjectData::WorkspaceObject { position, .. }
            | ObjectData::VolumeObject { position, .. }
            | ObjectData::RecentObject { position, .. }
            | ObjectData::TrashObject { position, .. }
            | ObjectData::NetworkObject { position, .. } => *position = new_pos,
        }
    }

    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        match self {
            ObjectData::File { metadata, .. }
            | ObjectData::Folder { metadata, .. }
            | ObjectData::Application { metadata, .. }
            | ObjectData::Project { metadata, .. }
            | ObjectData::Shortcut { metadata, .. }
            | ObjectData::Widget { metadata, .. }
            | ObjectData::WorkspaceObject { metadata, .. }
            | ObjectData::VolumeObject { metadata, .. }
            | ObjectData::RecentObject { metadata, .. }
            | ObjectData::TrashObject { metadata, .. }
            | ObjectData::NetworkObject { metadata, .. } => {
                metadata.insert(key.into(), value.into());
            }
        }
    }
}

// ── Builder helpers ──────────────────────────────────────────────────

pub fn new_file(name: impl Into<String>, path: impl AsRef<Path>, x: f64, y: f64) -> ObjectData {
    ObjectData::File {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        path: path.as_ref().to_string_lossy().to_string(),
        position: DesktopObjectPosition::desktop(x, y),
        metadata: HashMap::new(),
    }
}

pub fn new_folder(name: impl Into<String>, path: impl AsRef<Path>, x: f64, y: f64) -> ObjectData {
    ObjectData::Folder {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        path: path.as_ref().to_string_lossy().to_string(),
        position: DesktopObjectPosition::desktop(x, y),
        metadata: HashMap::new(),
    }
}

pub fn new_application(
    name: impl Into<String>,
    desktop_file: impl Into<String>,
    x: f64,
    y: f64,
) -> ObjectData {
    ObjectData::Application {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        desktop_file: desktop_file.into(),
        position: DesktopObjectPosition::desktop(x, y),
        metadata: HashMap::new(),
    }
}

pub fn new_project(name: impl Into<String>, path: impl AsRef<Path>, x: f64, y: f64) -> ObjectData {
    ObjectData::Project {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        path: path.as_ref().to_string_lossy().to_string(),
        position: DesktopObjectPosition::desktop(x, y),
        metadata: HashMap::new(),
    }
}

pub fn new_shortcut(
    name: impl Into<String>,
    target: impl AsRef<Path>,
    x: f64,
    y: f64,
) -> ObjectData {
    ObjectData::Shortcut {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        target: target.as_ref().to_string_lossy().to_string(),
        position: DesktopObjectPosition::desktop(x, y),
        metadata: HashMap::new(),
    }
}

pub fn new_widget(
    name: impl Into<String>,
    wtype: impl Into<String>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> ObjectData {
    ObjectData::Widget {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        wtype: wtype.into(),
        position: DesktopObjectPosition::desktop(x, y).with_size(width, height),
        metadata: HashMap::new(),
    }
}

pub fn new_workspace_object(
    name: impl Into<String>,
    workspace_id: u8,
    x: f64,
    y: f64,
) -> ObjectData {
    ObjectData::WorkspaceObject {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        workspace_id,
        position: DesktopObjectPosition::desktop(x, y).on_workspace(workspace_id),
        metadata: HashMap::new(),
    }
}

pub fn new_volume(
    name: impl Into<String>,
    mount_path: impl AsRef<Path>,
    filesystem: impl Into<String>,
    x: f64,
    y: f64,
) -> ObjectData {
    ObjectData::VolumeObject {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        mount_path: mount_path.as_ref().to_string_lossy().to_string(),
        filesystem: filesystem.into(),
        position: DesktopObjectPosition::desktop(x, y),
        metadata: HashMap::new(),
    }
}

pub fn new_recent(
    name: impl Into<String>,
    path: impl Into<String>,
    accessed_at: i64,
    x: f64,
    y: f64,
) -> ObjectData {
    ObjectData::RecentObject {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        path: path.into(),
        accessed_at,
        position: DesktopObjectPosition::desktop(x, y),
        metadata: HashMap::new(),
    }
}

pub fn new_trash(name: impl Into<String>, item_count: u32, x: f64, y: f64) -> ObjectData {
    ObjectData::TrashObject {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        item_count,
        position: DesktopObjectPosition::desktop(x, y),
        metadata: HashMap::new(),
    }
}

pub fn new_network(
    name: impl Into<String>,
    uri: impl Into<String>,
    protocol: impl Into<String>,
    x: f64,
    y: f64,
) -> ObjectData {
    ObjectData::NetworkObject {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        uri: uri.into(),
        protocol: protocol.into(),
        position: DesktopObjectPosition::desktop(x, y),
        metadata: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_data_type() {
        let f = new_file("test.rs", "/tmp/test.rs", 10.0, 20.0);
        assert_eq!(f.object_type(), "file");
        assert_eq!(f.name(), "test.rs");
    }

    #[test]
    fn test_folder_icon() {
        let f = new_folder("My Folder", "/tmp/my-folder", 10.0, 20.0);
        assert_eq!(f.icon_name(), "folder-symbolic");
    }

    #[test]
    fn test_position_mutation() {
        let mut f = new_file("test.rs", "/tmp/test.rs", 0.0, 0.0);
        f.set_position(DesktopObjectPosition::desktop(100.0, 200.0));
        assert_eq!(f.position().x, 100.0);
        assert_eq!(f.position().y, 200.0);
    }
}
