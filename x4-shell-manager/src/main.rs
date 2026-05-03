mod cli;
mod commands;
mod arch;
mod session;
mod systemd;
mod filesystem;
mod config;
mod diagnostics;
mod utils;

use clap::Parser;
use tracing_subscriber::fmt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    let log_level = if cli.verbose { "debug" } else { "info" };
    fmt().with_env_filter(tracing_subscriber::EnvFilter::new(log_level)).init();

    tracing::debug!("CLI args: {:?}", cli);

    commands::run(&cli)
}
