use std::fs;
use std::path::PathBuf;

const MAX_RECENT: usize = 10;

fn path() -> PathBuf {
    let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("~/.config"));
    p.push("xarph");
    p.push("recent_apps.json");
    p
}

pub fn load() -> Vec<String> {
    let p = path();
    fs::read_to_string(&p)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save(apps: &[String]) {
    let p = path();
    if let Some(parent) = p.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&p, serde_json::to_string_pretty(apps).unwrap_or_default());
}

pub fn track_launch(app_id: &str) {
    let mut recent = load();
    recent.retain(|id| id != app_id);
    recent.insert(0, app_id.to_string());
    recent.truncate(MAX_RECENT);
    save(&recent);
}
