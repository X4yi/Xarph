use crate::filesystem;
use crate::config;
use crate::session;
use crate::systemd;
use crate::filesystem::paths;

pub fn run(cli: &crate::cli::Cli) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting x4-shell repair");

    let dirs = [
        paths::config_dir(),
        paths::hypr_config_dir(),
        paths::shell_config_dir(),
        paths::data_dir(),
        paths::cache_dir(),
        paths::systemd_service_dir(),
    ];
    for dir in dirs {
        if !dir.exists() {
            tracing::warn!("Missing directory: {}", dir.display());
            filesystem::create_dir(&dir, cli.dry_run)?;
        }
    }

    let hypr_conf = paths::hypr_config_dir().join("hyprland.conf");
    if !hypr_conf.exists() {
        tracing::warn!("Missing hyprland.conf, regenerating");
        config::generate_hypr_config(cli.dry_run)?;
    }
    let shell_conf = paths::shell_config_dir().join("settings.json");
    if !shell_conf.exists() {
        tracing::warn!("Missing shell settings, regenerating");
        config::generate_shell_config(cli.dry_run)?;
    }

    let session_script = if cli.system_wide {
        paths::bin_dir_system().join("x4-shell-session")
    } else {
        paths::bin_dir_user().join("x4-shell-session")
    };
    if !session_script.exists() {
        tracing::warn!("Missing session script, reinstalling");
        session::install_session_script(cli.system_wide, cli.dry_run, None)?;
    }

    let service_path = paths::systemd_service_dir().join("x4-shell-daemon.service");
    if !service_path.exists() {
        tracing::warn!("Missing systemd service, reinstalling");
        systemd::install_service(cli.system_wide, cli.dry_run)?;
    }

    let session_file = if cli.system_wide {
        paths::session_dir_system().join("x4-shell.desktop")
    } else {
        paths::session_dir_user().join("x4-shell.desktop")
    };
    if !session_file.exists() {
        tracing::warn!("Missing session file, reinstalling");
        session::install_session_file(cli.system_wide, cli.dry_run, None)?;
    }

    tracing::info!("Repair complete!");
    Ok(())
}
