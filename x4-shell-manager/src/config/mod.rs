use crate::filesystem::{atomic, paths};
use serde_json::json;

const DEFAULT_HYPRLAND_CONF: &str = r#"
monitor=,preferred,auto,1
exec-once=dbus-launch --sh-syntax

input {
    kb_layout=us
    follow_mouse=1
}

windowrulev2=float,class:^(pavucontrol)$
windowrulev2=noop,class:^(steam)$

bind=MOD, Q, exit
bind=MOD, C, exec, kitty
bind=MOD, V, togglefloating
bind=MOD, 1, workspace, 1
bind=MOD, 2, workspace, 2
bind=MOD, 3, workspace, 3
bind=MOD, 4, workspace, 4
bind=MOD, 5, workspace, 5
"#;

pub fn generate_default_configs(dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    generate_hypr_config(dry_run)?;
    generate_shell_config(dry_run)?;
    Ok(())
}

pub fn generate_hypr_config(dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let hypr_conf_path = paths::hypr_config_dir().join("hyprland.conf");
    atomic::atomic_write(&hypr_conf_path, DEFAULT_HYPRLAND_CONF, dry_run)?;
    tracing::info!("Generated default Hyprland config at {}", hypr_conf_path.display());
    Ok(())
}

pub fn generate_shell_config(dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let shell_conf_path = paths::shell_config_dir().join("settings.json");
    let settings = json!({
        "daemon": {
            "ipc_version": "1",
            "log_level": "info"
        },
        "ui": {
            "qml_path": "qrc:/qml/main.qml"
        }
    });
    let settings_str = serde_json::to_string_pretty(&settings)?;
    atomic::atomic_write(&shell_conf_path, &settings_str, dry_run)?;
    tracing::info!("Generated default shell config at {}", shell_conf_path.display());
    Ok(())
}
