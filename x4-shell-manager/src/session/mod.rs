use crate::filesystem::{self, atomic, paths};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};
use std::io::Write;

const SESSION_SCRIPT: &str = r#"#!/bin/sh
XDG_DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"
XDG_CONFIG_HOME="${XDG_CONFIG_HOME:-$HOME/.config}"

DAEMON_PATH="/usr/local/bin/x4shell-daemon"
if [ ! -x "$DAEMON_PATH" ]; then
    DAEMON_PATH="$HOME/.local/bin/x4shell-daemon"
fi
if [ ! -x "$DAEMON_PATH" ]; then
    echo "x4shell-daemon not found" >&2
    exit 1
fi
$DAEMON_PATH &

sleep 1

quickshell -c "$XDG_DATA_HOME/x4-shell/ui/main.qml" &

exec Hyprland --config "$XDG_CONFIG_HOME/x4-shell/hypr/hyprland.conf"
"#;

const SESSION_DESKTOP: &str = r#"[Desktop Entry]
Name=X4 Shell
Comment=Custom Hyprland-based Wayland desktop shell
Exec=/usr/local/bin/x4-shell-session
Type=Application
DesktopNames=X4Shell
Keywords=wayland;hyprland;
"#;

pub fn install_session_script(_system_wide: bool, dry_run: bool, sudo_password: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let bin_dir = paths::bin_dir_system();
    let script_path = bin_dir.join("x4-shell-session");
    filesystem::ensure_safe_path(&script_path)?;

    if let Some(password) = sudo_password {
        if !dry_run {
            let mut child = Command::new("sudo")
                .arg("-S")
                .arg("tee")
                .arg(&script_path)
                .stdin(Stdio::piped())
                .spawn()?;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(SESSION_SCRIPT.as_bytes())?;
                writeln!(stdin, "{}", password)?;
                drop(stdin);
            }
            child.wait()?;
            let mut child = Command::new("sudo")
                .arg("-S")
                .arg("chmod")
                .arg("755")
                .arg(&script_path)
                .stdin(Stdio::piped())
                .spawn()?;
            if let Some(mut stdin) = child.stdin.take() {
                writeln!(stdin, "{}", password)?;
                drop(stdin);
            }
            child.wait()?;
        }
    } else {
        atomic::atomic_write(&script_path, SESSION_SCRIPT, dry_run)?;
        if !dry_run {
            let metadata = std::fs::metadata(&script_path)?;
            let mut perms = metadata.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&script_path, perms)?;
        }
    }

    tracing::info!("Installed x4-shell-session to {}", script_path.display());
    Ok(())
}

pub fn install_session_file(_system_wide: bool, dry_run: bool, sudo_password: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let session_dir = paths::session_dir_system();
    let desktop_path = session_dir.join("x4-shell.desktop");
    filesystem::ensure_safe_path(&desktop_path)?;

    if let Some(password) = sudo_password {
        if !dry_run {
            let mut child = Command::new("sudo")
                .arg("-S")
                .arg("tee")
                .arg(&desktop_path)
                .stdin(Stdio::piped())
                .spawn()?;
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(SESSION_DESKTOP.as_bytes())?;
                writeln!(stdin, "{}", password)?;
                drop(stdin);
            }
            child.wait()?;
        }
    } else {
        atomic::atomic_write(&desktop_path, SESSION_DESKTOP, dry_run)?;
    }

    tracing::info!("Installed session file to {}", desktop_path.display());
    Ok(())
}
