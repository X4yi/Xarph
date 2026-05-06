use crate::core::types::*;

#[derive(Debug, Clone)]
pub enum HyprlandEvent {
    WorkspaceFocused { id: u32, name: String },
    WindowOpened { id: u32, class: String, workspace_id: u32 },
    WindowClosed { id: u32 },
    MonitorAdded { name: String, width: u32, height: u32 },
    MonitorRemoved { name: String },
}

#[derive(Debug, Clone)]
pub enum Event {
    Hyprland(HyprlandEvent),
    WorkspaceChanged(Workspace),
    FocusedWindowChanged(Option<Window>),
    PowerChanged(PowerState),
    VolumeChanged(AudioState),
    NotificationAdded(Notification),
    SettingsChanged,
}
