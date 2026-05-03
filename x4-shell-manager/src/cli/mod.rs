use clap::Parser;
use crate::commands;

#[derive(Parser, Debug)]
#[command(version, about = "Installer/manager for x4-shell (Hyprland-based Wayland shell)", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: commands::Commands,

    #[arg(short, long)]
    pub yes: bool,

    #[arg(short, long)]
    pub dry_run: bool,

    #[arg(short, long)]
    pub verbose: bool,

    #[arg(long)]
    pub system_wide: bool,
}
