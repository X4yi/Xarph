use crate::filesystem::{self, atomic, paths};
use std::process::Command;

const SERVICE_FILE: &str = r#"[Unit]
Description=X4 Shell Daemon
After=graphical-session.target
PartOf=graphical-session.target

[Service]
ExecStart=/usr/local/bin/x4shell-daemon
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=graphical-session.target
"#;

pub fn install_service(_system_wide: bool, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let service_dir = paths::systemd_service_dir();
    let service_path = service_dir.join("x4-shell-daemon.service");
    filesystem::ensure_safe_path(&service_path)?;

    atomic::atomic_write(&service_path, SERVICE_FILE, dry_run)?;
    tracing::info!("Installed systemd service to {}", service_path.display());
    Ok(())
}

pub fn enable_service(dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    if dry_run {
        tracing::info!("Dry run: would enable x4-shell-daemon.service");
        return Ok(());
    }

    let status = Command::new("systemctl")
        .arg("--user")
        .arg("enable")
        .arg("x4-shell-daemon.service")
        .status()?;

    if !status.success() {
        return Err("Failed to enable systemd service".into());
    }

    tracing::info!("Enabled x4-shell-daemon.service");
    Ok(())
}

pub fn start_service(dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    if dry_run {
        tracing::info!("Dry run: would start x4-shell-daemon.service");
        return Ok(());
    }

    let status = Command::new("systemctl")
        .arg("--user")
        .arg("start")
        .arg("x4-shell-daemon.service")
        .status()?;

    if !status.success() {
        return Err("Failed to start systemd service".into());
    }

    tracing::info!("Started x4-shell-daemon.service");
    Ok(())
}

pub fn stop_service(dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    if dry_run {
        tracing::info!("Dry run: would stop x4-shell-daemon.service");
        return Ok(());
    }

    let status = Command::new("systemctl")
        .arg("--user")
        .arg("stop")
        .arg("x4-shell-daemon.service")
        .status();

    match status {
        Ok(s) if s.success() => {
            tracing::info!("Stopped x4-shell-daemon.service");
        }
        _ => {
            tracing::warn!("Failed to stop systemd service (may not be running)");
        }
    }
    Ok(())
}

pub fn disable_service(dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    if dry_run {
        tracing::info!("Dry run: would disable x4-shell-daemon.service");
        return Ok(());
    }

    let status = Command::new("systemctl")
        .arg("--user")
        .arg("disable")
        .arg("x4-shell-daemon.service")
        .status();

    match status {
        Ok(s) if s.success() => {
            tracing::info!("Disabled x4-shell-daemon.service");
        }
        _ => {
            tracing::warn!("Failed to disable systemd service (may not be enabled)");
        }
    }
    Ok(())
}
