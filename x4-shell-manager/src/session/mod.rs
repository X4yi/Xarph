use crate::filesystem::{self, atomic, paths};
use std::os::unix::fs::PermissionsExt;

const SESSION_SCRIPT: &str = r#"#!/bin/sh
# x4-shell-session: Launcher for x4-shell
# Starts the daemon, UI, and Hyprland with custom config

# Start the daemon
x4-shell-daemon &

# Wait for daemon to initialize
sleep 1

# Start Quickshell UI (QML shell)
quickshell -c "$XDG_DATA_HOME/x4-shell/ui/main.qml" &

# Launch Hyprland with isolated config
exec Hyprland --config "$XDG_CONFIG_HOME/x4-shell/hypr/hyprland.conf"
"#;

const SESSION_DESKTOP: &str = r#"[Desktop Entry]
Name=X4 Shell
Comment=Custom Hyprland-based Wayland desktop shell
Exec=x4-shell-session
Type=Application
DesktopNames=X4Shell
X-GNOME-WaylandClientsSupported=true
X-GNOME-UsesWayland=true
Keywords=wayland;hyprland;
"#;

pub fn install_session_script(system_wide: bool, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let bin_dir = if system_wide {
        paths::bin_dir_system()
    } else {
        paths::bin_dir_user()
    };

    let script_path = bin_dir.join("x4-shell-session");
    filesystem::ensure_safe_path(&script_path)?;

    atomic::atomic_write(&script_path, SESSION_SCRIPT, dry_run)?;

    if !dry_run {
        let metadata = std::fs::metadata(&script_path)?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script_path, perms)?;
        tracing::debug!("Made {} executable", script_path.display());
    }

    tracing::info!("Installed x4-shell-session to {}", script_path.display());
    Ok(())
}

pub fn install_session_file(system_wide: bool, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let session_dir = if system_wide {
        paths::session_dir_system()
    } else {
        paths::session_dir_user()
    };

    let desktop_path = session_dir.join("x4-shell.desktop");
    filesystem::ensure_safe_path(&desktop_path)?;

    atomic::atomic_write(&desktop_path, SESSION_DESKTOP, dry_run)?;
    tracing::info!("Installed session file to {}", desktop_path.display());
    Ok(())
}
