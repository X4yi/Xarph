use crate::arch;
use crate::filesystem;
use crate::config;
use crate::session;
use crate::systemd;
use std::path::Path;

pub fn run(cli: &crate::cli::Cli) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting x4-shell installation");

    if !arch::is_arch_based() {
        return Err("This installer only supports Arch-based distributions".into());
    }
    tracing::debug!("Arch-based system detected");

    arch::check_dependencies()?;
    tracing::debug!("All dependencies satisfied");

    filesystem::create_x4_dirs(cli.dry_run)?;

    config::generate_default_configs(cli.dry_run)?;

    install_ui_files(cli.dry_run)?;

    session::install_session_script(cli.system_wide, cli.dry_run)?;

    systemd::install_service(cli.system_wide, cli.dry_run)?;

    systemd::enable_service(cli.dry_run)?;
    systemd::start_service(cli.dry_run)?;

    session::install_session_file(cli.system_wide, cli.dry_run)?;

    if !cli.dry_run {
        tracing::info!("Note: Ensure x4-shell-daemon is installed in PATH or ~/.local/bin/");
        tracing::info!("Note: Ensure quickshell is installed for UI");
    }

    tracing::info!("Installation complete!");
    Ok(())
}

fn install_ui_files(dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let ui_source = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("x4-shell-ui");
    let ui_dest = dirs::data_dir().ok_or("Could not find data directory")?.join("x4-shell/ui");

    if !ui_source.exists() {
        tracing::warn!("UI source directory not found: {}", ui_source.display());
        return Ok(());
    }

    if dry_run {
        tracing::info!("Dry run: would install UI files from {} to {}",
            ui_source.display(), ui_dest.display());
        return Ok(());
    }

    std::fs::create_dir_all(&ui_dest)?;

    copy_dir_all(&ui_source, &ui_dest)?;
    tracing::info!("Installed UI files to {}", ui_dest.display());
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else if ty.is_file() {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
