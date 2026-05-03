use dirs;
use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    dirs::config_dir().map(|d| d.join("x4-shell")).unwrap()
}

pub fn hypr_config_dir() -> PathBuf {
    config_dir().join("hypr")
}

pub fn shell_config_dir() -> PathBuf {
    config_dir().join("shell")
}

pub fn data_dir() -> PathBuf {
    dirs::data_dir().map(|d| d.join("x4-shell")).unwrap()
}

pub fn cache_dir() -> PathBuf {
    dirs::cache_dir().map(|d| d.join("x4-shell")).unwrap()
}

pub fn systemd_service_dir() -> PathBuf {
    dirs::config_dir().map(|d| d.join("systemd/user")).unwrap()
}

pub fn session_dir_user() -> PathBuf {
    dirs::data_dir().map(|d| d.join("wayland-sessions")).unwrap()
}

pub fn session_dir_system() -> PathBuf {
    PathBuf::from("/usr/share/wayland-sessions")
}

pub fn bin_dir_user() -> PathBuf {
    dirs::executable_dir().unwrap_or_else(|| dirs::home_dir().unwrap().join(".local/bin"))
}

pub fn bin_dir_system() -> PathBuf {
    PathBuf::from("/usr/local/bin")
}
