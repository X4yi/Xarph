use crate::arch;
use crate::filesystem;
use crate::config;
use crate::session;
use crate::systemd;
use crate::filesystem::paths;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};
use std::io::Write;

fn get_sudo_password() -> Result<String, Box<dyn std::error::Error>> {
    dialoguer::Password::new()
        .with_prompt("Contraseña sudo")
        .validate_with(|s: &String| if s.is_empty() { Err("La contraseña no puede estar vacía") } else { Ok(()) })
        .interact()
        .map_err(|e| e.into())
}

fn run_with_sudo(password: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("sudo")
        .arg("-S")
        .args(args)
        .stdin(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        writeln!(stdin, "{}", password)?;
        drop(stdin);
    }
    let status = child.wait()?;
    if !status.success() {
        return Err(format!("Comando falló: {}", args.join(" ")).into());
    }
    Ok(())
}

fn needs_sudo() -> bool {
    !Command::new("test")
        .arg("-w")
        .arg(paths::bin_dir_system())
        .status()
        .map(|s| s.success())
        .unwrap_or(true)
}

pub fn run(cli: &crate::cli::Cli) -> Result<(), Box<dyn std::error::Error>> {
    let sudo_password = if needs_sudo() {
        Some(get_sudo_password()?)
    } else {
        None
    };

    if !cli.dry_run {
        let proceed = dialoguer::Confirm::new()
            .with_prompt("¿Instalar x4-shell?")
            .default(true)
            .interact()?;
        if !proceed {
            return Ok(());
        }
    }

    tracing::info!("Starting x4-shell installation");

    if !arch::is_arch_based() {
        return Err("This installer only supports Arch-based distributions".into());
    }
    tracing::debug!("Arch-based system detected");

    arch::check_dependencies()?;
    tracing::debug!("All dependencies satisfied");

    filesystem::create_x4_dirs(cli.dry_run)?;
    config::generate_default_configs(cli.dry_run)?;
    install_ui_files(cli.dry_run, sudo_password.as_deref())?;
    install_daemon_binary(cli.dry_run, sudo_password.as_deref())?;
    session::install_session_script(cli.system_wide, cli.dry_run, sudo_password.as_deref())?;

    if which::which("systemctl").is_ok() {
        systemd::install_service(cli.system_wide, cli.dry_run)?;
        systemd::enable_service(cli.dry_run)?;
        systemd::start_service(cli.dry_run)?;
    } else {
        tracing::warn!("systemctl no encontrado, omitiendo servicios systemd");
    }

    session::install_session_file(cli.system_wide, cli.dry_run, sudo_password.as_deref())?;

    if !cli.dry_run {
        tracing::info!("Note: Ensure quickshell is installed for UI");
    }

    tracing::info!("Installation complete!");
    Ok(())
}

fn install_ui_files(dry_run: bool, _sudo_password: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
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

fn install_daemon_binary(dry_run: bool, sudo_password: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let src = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("x4shell-daemon/target/release/x4shell-daemon");
    let dst = paths::bin_dir_system().join("x4shell-daemon");

    if dry_run {
        tracing::info!("Dry run: would copy {:?} to {:?}", src, dst);
        return Ok(());
    }

    if !src.exists() {
        tracing::warn!("Daemon binary not found at {:?}, skipping", src);
        return Ok(());
    }

    if let Some(password) = sudo_password {
        run_with_sudo(password, &["cp", src.to_str().unwrap(), dst.to_str().unwrap()])?;
        run_with_sudo(password, &["chmod", "755", dst.to_str().unwrap()])?;
    } else {
        std::fs::copy(&src, &dst)?;
        let metadata = std::fs::metadata(&dst)?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&dst, perms)?;
    }
    tracing::info!("Installed daemon binary to {:?}", dst);
    Ok(())
}
