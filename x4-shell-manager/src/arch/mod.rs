use std::process::Command;

pub fn is_arch_based() -> bool {
    let Ok(release) = std::fs::read_to_string("/etc/os-release") else {
        return false;
    };

    let mut id = String::new();
    let mut id_like = Vec::new();

    for line in release.lines() {
        if let Some(val) = line.strip_prefix("ID=") {
            id = val.trim_matches('"').to_string();
        } else if let Some(val) = line.strip_prefix("ID_LIKE=") {
            let like = val.trim_matches('"');
            id_like = like.split_whitespace().map(|s| s.to_string()).collect();
        }
    }

    id == "arch" || id_like.contains(&"arch".to_string())
}

pub fn check_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let deps = ["hyprland", "systemctl", "dbus-daemon"];
    for dep in deps {
        let status = Command::new("which").arg(dep).status()?;
        if !status.success() {
            return Err(format!("Missing dependency: {}", dep).into());
        }
    }

    let status = Command::new("systemctl")
        .arg("--user")
        .arg("status")
        .status();
    if status.is_err() || !status.unwrap().success() {
        return Err("systemd user session is not running".into());
    }

    Ok(())
}
