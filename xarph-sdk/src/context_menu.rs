use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuItem {
    Action {
        id: String,
        label: String,
        icon: Option<String>,
        shortcut: Option<String>,
        enabled: bool,
    },
    Separator,
    Submenu {
        label: String,
        icon: Option<String>,
        items: Vec<MenuItem>,
    },
    Info {
        label: String,
    },
}

impl MenuItem {
    pub fn action(id: impl Into<String>, label: impl Into<String>) -> Self {
        MenuItem::Action {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            enabled: true,
        }
    }

    pub fn action_with_icon(
        id: impl Into<String>,
        label: impl Into<String>,
        icon: impl Into<String>,
    ) -> Self {
        MenuItem::Action {
            id: id.into(),
            label: label.into(),
            icon: Some(icon.into()),
            shortcut: None,
            enabled: true,
        }
    }

    pub fn action_disabled(id: impl Into<String>, label: impl Into<String>) -> Self {
        MenuItem::Action {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            enabled: false,
        }
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        if let MenuItem::Action {
            shortcut: ref mut s,
            ..
        } = self
        {
            *s = Some(shortcut.into());
        }
        self
    }

    pub fn submenu(label: impl Into<String>, items: Vec<MenuItem>) -> Self {
        MenuItem::Submenu {
            label: label.into(),
            icon: None,
            items,
        }
    }

    pub fn info(label: impl Into<String>) -> Self {
        MenuItem::Info { label: label.into() }
    }
}

pub fn build_file_menu(object_name: &str) -> Vec<MenuItem> {
    vec![
        MenuItem::info(object_name),
        MenuItem::Separator,
        MenuItem::action_with_icon("open", "Open", "document-open-symbolic"),
        MenuItem::action_with_icon("open_with", "Open With...", "document-open-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("copy", "Copy", "edit-copy-symbolic"),
        MenuItem::action_with_icon("cut", "Cut", "edit-cut-symbolic"),
        MenuItem::action_with_icon("paste", "Paste", "edit-paste-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("rename", "Rename", "document-edit-symbolic"),
        MenuItem::action_with_icon("delete", "Delete", "edit-delete-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("properties", "Properties", "document-properties-symbolic"),
    ]
}

pub fn build_folder_menu(object_name: &str) -> Vec<MenuItem> {
    vec![
        MenuItem::info(object_name),
        MenuItem::Separator,
        MenuItem::action_with_icon("open", "Open", "folder-open-symbolic"),
        MenuItem::action_with_icon(
            "open_in_terminal",
            "Open in Terminal",
            "utilities-terminal-symbolic",
        ),
        MenuItem::Separator,
        MenuItem::action_with_icon("copy", "Copy", "edit-copy-symbolic"),
        MenuItem::action_with_icon("cut", "Cut", "edit-cut-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("rename", "Rename", "document-edit-symbolic"),
        MenuItem::action_with_icon("delete", "Delete", "edit-delete-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("properties", "Properties", "document-properties-symbolic"),
    ]
}

pub fn build_application_menu(object_name: &str) -> Vec<MenuItem> {
    vec![
        MenuItem::info(object_name),
        MenuItem::Separator,
        MenuItem::action_with_icon("launch", "Launch", "system-run-symbolic"),
        MenuItem::action_with_icon(
            "launch_terminal",
            "Launch in Terminal",
            "utilities-terminal-symbolic",
        ),
        MenuItem::Separator,
        MenuItem::action_with_icon("pin_to_desktop", "Pin to Desktop", "view-pin-symbolic"),
        MenuItem::action_with_icon(
            "add_to_favorites",
            "Add to Favorites",
            "starred-symbolic",
        ),
        MenuItem::Separator,
        MenuItem::action_with_icon("properties", "Properties", "document-properties-symbolic"),
    ]
}

pub fn build_project_menu(object_name: &str) -> Vec<MenuItem> {
    vec![
        MenuItem::info(object_name),
        MenuItem::Separator,
        MenuItem::action_with_icon("open", "Open", "folder-open-symbolic"),
        MenuItem::action_with_icon(
            "open_in_terminal",
            "Open in Terminal",
            "utilities-terminal-symbolic",
        ),
        MenuItem::Separator,
        MenuItem::action_with_icon(
            "open_in_editor",
            "Open in Editor",
            "text-editor-symbolic",
        ),
        MenuItem::Separator,
        MenuItem::action_with_icon("rename", "Rename", "document-edit-symbolic"),
        MenuItem::action_with_icon("delete", "Delete", "edit-delete-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("properties", "Properties", "document-properties-symbolic"),
    ]
}

pub fn build_shortcut_menu(object_name: &str) -> Vec<MenuItem> {
    vec![
        MenuItem::info(object_name),
        MenuItem::Separator,
        MenuItem::action_with_icon("open_target", "Open Target", "go-jump-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("rename", "Rename", "document-edit-symbolic"),
        MenuItem::action_with_icon("delete", "Delete", "edit-delete-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("properties", "Properties", "document-properties-symbolic"),
    ]
}

pub fn build_widget_menu(widget_name: &str) -> Vec<MenuItem> {
    vec![
        MenuItem::info(widget_name),
        MenuItem::Separator,
        MenuItem::action_with_icon("configure", "Configure", "preferences-system-symbolic"),
        MenuItem::action_with_icon("move", "Move", "view-dual-symbolic"),
        MenuItem::action_with_icon("resize", "Resize", "view-fullscreen-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("duplicate", "Duplicate", "edit-copy-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("remove", "Remove from Desktop", "edit-delete-symbolic"),
    ]
}

pub fn build_desktop_context_menu() -> Vec<MenuItem> {
    vec![
        MenuItem::action_with_icon("new_folder", "New Folder", "folder-new-symbolic"),
        MenuItem::action_with_icon("new_file", "New File", "document-new-symbolic"),
        MenuItem::Separator,
        MenuItem::submenu(
            "Add Widget",
            vec![
                MenuItem::action("add_clock", "Clock"),
                MenuItem::action("add_calendar", "Calendar"),
                MenuItem::action("add_system_monitor", "System Monitor"),
                MenuItem::action("add_notes", "Notes"),
                MenuItem::action("add_weather", "Weather"),
            ],
        ),
        MenuItem::Separator,
        MenuItem::action_with_icon(
            "change_wallpaper",
            "Change Wallpaper",
            "preferences-desktop-wallpaper-symbolic",
        ),
        MenuItem::action_with_icon("display_settings", "Display Settings", "video-display-symbolic"),
        MenuItem::action_with_icon(
            "desktop_settings",
            "Desktop Settings",
            "preferences-desktop-symbolic",
        ),
        MenuItem::Separator,
        MenuItem::action_with_icon(
            "open_terminal",
            "Open Terminal",
            "utilities-terminal-symbolic",
        ),
    ]
}

pub fn build_volume_menu(volume_name: &str) -> Vec<MenuItem> {
    vec![
        MenuItem::info(volume_name),
        MenuItem::Separator,
        MenuItem::action_with_icon("open", "Open", "folder-open-symbolic"),
        MenuItem::action_with_icon("eject", "Eject", "media-eject-symbolic"),
        MenuItem::action_with_icon("unmount", "Unmount", "media-eject-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("properties", "Properties", "document-properties-symbolic"),
    ]
}

pub fn build_trash_menu() -> Vec<MenuItem> {
    vec![
        MenuItem::action_with_icon("open", "Open Trash", "user-trash-symbolic"),
        MenuItem::action_with_icon("empty", "Empty Trash", "edit-clear-all-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("properties", "Properties", "document-properties-symbolic"),
    ]
}

pub fn build_network_menu(object_name: &str) -> Vec<MenuItem> {
    vec![
        MenuItem::info(object_name),
        MenuItem::Separator,
        MenuItem::action_with_icon("connect", "Connect", "network-connect-symbolic"),
        MenuItem::action_with_icon("disconnect", "Disconnect", "network-disconnect-symbolic"),
        MenuItem::Separator,
        MenuItem::action_with_icon("properties", "Properties", "document-properties-symbolic"),
    ]
}

pub fn menu_for_object(obj: &crate::desktop_object::ObjectData) -> Vec<MenuItem> {
    match obj {
        crate::desktop_object::ObjectData::File { name, .. } => build_file_menu(name),
        crate::desktop_object::ObjectData::Folder { name, .. } => build_folder_menu(name),
        crate::desktop_object::ObjectData::Application { name, .. } => build_application_menu(name),
        crate::desktop_object::ObjectData::Project { name, .. } => build_project_menu(name),
        crate::desktop_object::ObjectData::Shortcut { name, .. } => build_shortcut_menu(name),
        crate::desktop_object::ObjectData::Widget { name, .. } => build_widget_menu(name),
        crate::desktop_object::ObjectData::VolumeObject { name, .. } => build_volume_menu(name),
        crate::desktop_object::ObjectData::TrashObject { .. } => build_trash_menu(),
        crate::desktop_object::ObjectData::NetworkObject { name, .. } => {
            build_network_menu(name)
        }
        crate::desktop_object::ObjectData::WorkspaceObject { name, .. } => {
            vec![
                MenuItem::info(name),
                MenuItem::Separator,
                MenuItem::action_with_icon("switch_to", "Switch to Workspace", "go-jump-symbolic"),
                MenuItem::Separator,
                MenuItem::action_with_icon("configure", "Configure", "preferences-system-symbolic"),
            ]
        }
        crate::desktop_object::ObjectData::RecentObject { name, .. } => {
            build_file_menu(name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_menu_has_open() {
        let menu = build_file_menu("test.txt");
        assert!(menu.iter().any(|m| matches!(m, MenuItem::Action { id, .. } if id == "open")));
    }

    #[test]
    fn test_desktop_menu_has_new_folder() {
        let menu = build_desktop_context_menu();
        assert!(menu
            .iter()
            .any(|m| matches!(m, MenuItem::Action { id, .. } if id == "new_folder")));
    }
}
