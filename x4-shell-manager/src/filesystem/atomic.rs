use std::path::Path;
use super::ensure_safe_path;

pub fn atomic_write(path: &Path, content: &str, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    ensure_safe_path(path)?;

    if dry_run {
        tracing::info!("Dry run: would write to {}", path.display());
        return Ok(());
    }

    let dir = path.parent().ok_or("Path has no parent")?;
    let temp_name = format!("{}.tmp", path.file_name().unwrap().to_string_lossy());
    let temp_path = dir.join(temp_name);

    if path.exists() {
        let backup_path = path.with_extension("bak");
        fs_err::rename(path, &backup_path)?;
        tracing::debug!("Backed up {} to {}", path.display(), backup_path.display());
    }

    fs_err::write(&temp_path, content)?;

    fs_err::rename(temp_path, path)?;
    tracing::debug!("Atomically wrote to {}", path.display());

    Ok(())
}
