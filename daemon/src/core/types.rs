use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub focused: bool,
    pub monitor: String,
    pub windows: Vec<u32>, // Window IDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub class: String,
    pub workspace_id: u32,
    pub focused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub focused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerState {
    pub battery_percent: f32,
    pub charging: bool,
    pub on_battery: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: u64,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub urgency: u8,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioState {
    pub volume: f32,
    pub muted: bool,
}
