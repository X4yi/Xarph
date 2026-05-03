use crate::filesystem::paths;
use crate::systemd;
use std::fs;

pub fn run(cli: &crate::cli::Cli) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting x4-shell uninstallation");

    systemd::stop_service(cli.dry_run)?;
    systemd::disable_service(cli.dry_run)?;

    let service_path = paths::systemd_service_dir().join("x4-shell-daemon.service");
    if service_path.exists() {
        if cli.dry_run {
            tracing::info!("Dry run: would remove {}", service_path.display());
        } else {
            fs::remove_file(&service_path)?;
            tracing::info!("Removed service file {}", service_path.display());
        }
    }

    let session_user = paths::session_dir_user().join("x4-shell.desktop");
    let session_system = paths::session_dir_system().join("x4-shell.desktop");
    for path in &[session_user, session_system] {
        if path.exists() {
            if cli.dry_run {
                tracing::info!("Dry run: would remove {}", path.display());
            } else {
                fs::remove_file(path)?;
                tracing::info!("Removed session file {}", path.display());
            }
        }
    }

    let bin_dirs = if cli.system_wide {
        vec![paths::bin_dir_system()]
    } else {
        vec![paths::bin_dir_user(), paths::bin_dir_system()]
    };
    for bin_dir in bin_dirs {
        let script_path = bin_dir.join("x4-shell-session");
        if script_path.exists() {
            if cli.dry_run {
                tracing::info!("Dry run: would remove {}", script_path.display());
            } else {
                fs::remove_file(&script_path)?;
                tracing::info!("Removed session script {}", script_path.display());
            }
        }
    }

    if !cli.dry_run {
        tracing::info!("Preserving config directories. Use --purge to remove them (not implemented).");
    }

    tracing::info!("Uninstallation complete!");
    Ok(())
}
