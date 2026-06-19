use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub body: String,
    pub icon: Option<String>,
    pub urgency: NotificationUrgency,
    pub timestamp: i64,
    pub app_name: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

impl Default for NotificationUrgency {
    fn default() -> Self {
        NotificationUrgency::Normal
    }
}

pub struct NotificationService;

impl NotificationService {
    pub fn send_notification(
        title: &str,
        body: &str,
        urgency: NotificationUrgency,
    ) -> Result<(), String> {
        let notification = Notification {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            body: body.to_string(),
            icon: None,
            urgency,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
            app_name: Some("Xarph".to_string()),
        };
        Self::send_via_notify_send(&notification)
    }

    pub fn send_with_icon(
        title: &str,
        body: &str,
        icon: &str,
        urgency: NotificationUrgency,
    ) -> Result<(), String> {
        let notification = Notification {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            body: body.to_string(),
            icon: Some(icon.to_string()),
            urgency,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
            app_name: Some("Xarph".to_string()),
        };
        Self::send_via_notify_send(&notification)
    }

    fn send_via_notify_send(notification: &Notification) -> Result<(), String> {
        let urgency_str = match notification.urgency {
            NotificationUrgency::Low => "low",
            NotificationUrgency::Normal => "normal",
            NotificationUrgency::Critical => "critical",
        };

        let mut args = vec![
            "notify-send".to_string(),
            "-u".to_string(),
            urgency_str.to_string(),
            "-a".to_string(),
            notification.app_name.clone().unwrap_or_default(),
        ];

        if let Some(ref icon) = notification.icon {
            args.push("-i".to_string());
            args.push(icon.clone());
        }

        args.push(notification.title.clone());
        args.push(notification.body.clone());

        std::process::Command::new("notify-send")
            .args(&args[1..])
            .output()
            .map_err(|e| format!("Failed to run notify-send: {}", e))?;

        Ok(())
    }

    pub fn get_desktop_notifications() -> Vec<Notification> {
        // Read from XDG notification directory
        let dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("xarph")
            .join("notifications");

        let mut notifications = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".json") {
                        if let Ok(content) = std::fs::read_to_string(entry.path()) {
                            if let Ok(notification) = serde_json::from_str::<Notification>(&content) {
                                notifications.push(notification);
                            }
                        }
                    }
                }
            }
        }

        notifications.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        notifications
    }

    pub fn clear_notification(id: &str) -> Result<(), String> {
        let dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("xarph")
            .join("notifications");

        let path = dir.join(format!("{}.json", id));
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| format!("Failed to remove notification: {}", e))?;
        }
        Ok(())
    }

    pub fn clear_all_notifications() -> Result<(), String> {
        let dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/share"))
            .join("xarph")
            .join("notifications");

        if dir.exists() {
            std::fs::remove_dir_all(&dir)
                .map_err(|e| format!("Failed to clear notifications: {}", e))?;
        }
        Ok(())
    }
}

use std::path::PathBuf;
