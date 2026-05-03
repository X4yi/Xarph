pub mod paths;
pub mod atomic;

use std::path::Path;
use fs_err::create_dir_all;

pub fn ensure_safe_path(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let hypr_path = dirs::config_dir().map(|d| d.join("hypr"));
    if let Some(hypr) = hypr_path {
        if path.starts_with(hypr) {
            return Err(format!("Refusing to operate on Hyprland config path: {}", path.display()).into());
        }
    }
    Ok(())
}

pub fn create_dir(path: &Path, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    ensure_safe_path(path)?;
    if dry_run {
        tracing::info!("Dry run: would create directory {}", path.display());
        return Ok(());
    }
    if !path.exists() {
        create_dir_all(path)?;
        tracing::debug!("Created directory {}", path.display());
    }
    Ok(())
}

pub fn create_x4_dirs(dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let dirs = [
        paths::config_dir(),
        paths::hypr_config_dir(),
        paths::shell_config_dir(),
        paths::data_dir(),
        paths::cache_dir(),
        paths::systemd_service_dir(),
    ];

    for dir in dirs {
        create_dir(&dir, dry_run)?;
    }
    Ok(())
}
