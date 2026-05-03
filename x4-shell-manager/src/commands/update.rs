use crate::systemd;

pub fn run(cli: &crate::cli::Cli) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Starting x4-shell update");

    systemd::stop_service(cli.dry_run)?;

    if !cli.dry_run {
        tracing::info!("Binary update not implemented; skipping.");
    }

    systemd::start_service(cli.dry_run)?;

    tracing::info!("Update complete!");
    Ok(())
}
