use crate::filesystem::paths;
use clap::Args;
use serde_json::json;
use std::process::Command;

#[derive(Args, Debug)]
pub struct StatusArgs {
    #[arg(long)]
    pub json: bool,
}

pub fn run(_cli: &crate::cli::Cli, args: &StatusArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut status = json!({});

    let daemon_status = Command::new("systemctl")
        .arg("--user")
        .arg("is-active")
        .arg("x4-shell-daemon.service")
        .output();
    let daemon_active = match daemon_status {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim() == "active",
        Err(_) => false,
    };
    status["daemon_running"] = json!(daemon_active);

    let service_path = paths::systemd_service_dir().join("x4-shell-daemon.service");
    status["service_file_exists"] = json!(service_path.exists());

    let session_user = paths::session_dir_user().join("x4-shell.desktop");
    let session_system = paths::session_dir_system().join("x4-shell.desktop");
    status["session_file_exists"] = json!(session_user.exists() || session_system.exists());

    let hypr_conf = paths::hypr_config_dir().join("hyprland.conf");
    status["hypr_config_exists"] = json!(hypr_conf.exists());
    let shell_conf = paths::shell_config_dir().join("settings.json");
    status["shell_config_exists"] = json!(shell_conf.exists());
    status["shell_config_valid"] = json!(shell_conf.exists() && std::fs::read_to_string(&shell_conf).is_ok_and(|s| serde_json::from_str::<serde_json::Value>(&s).is_ok()));

    status["config_dir_exists"] = json!(paths::config_dir().exists());
    status["data_dir_exists"] = json!(paths::data_dir().exists());
    status["cache_dir_exists"] = json!(paths::cache_dir().exists());

    if args.json {
        println!("{}", serde_json::to_string_pretty(&status)?);
    } else {
        println!("x4-shell Status Report");
        println!("=========================");
        println!("Daemon running: {}", status["daemon_running"]);
        println!("Service file exists: {}", status["service_file_exists"]);
        println!("Session file exists: {}", status["session_file_exists"]);
        println!("Hyprland config exists: {}", status["hypr_config_exists"]);
        println!("Shell config exists: {}", status["shell_config_exists"]);
        println!("Shell config valid: {}", status["shell_config_valid"]);
        println!("\nDirectories:");
        println!("  Config: {} ({})", paths::config_dir().display(), status["config_dir_exists"]);
        println!("  Data: {} ({})", paths::data_dir().display(), status["data_dir_exists"]);
        println!("  Cache: {} ({})", paths::cache_dir().display(), status["cache_dir_exists"]);
    }

    Ok(())
}
