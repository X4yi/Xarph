pub mod install;
pub mod uninstall;
pub mod update;
pub mod repair;
pub mod status;

use clap::Subcommand;
use crate::cli::Cli;
use crate::commands::install::run as install_run;
use crate::commands::uninstall::run as uninstall_run;
use crate::commands::update::run as update_run;
use crate::commands::repair::run as repair_run;
use crate::commands::status::run as status_run;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Install,
    Uninstall,
    Update,
    Repair,
    Status(status::StatusArgs),
}

pub fn run(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        Commands::Install => install_run(&cli),
        Commands::Uninstall => uninstall_run(&cli),
        Commands::Update => update_run(&cli),
        Commands::Repair => repair_run(&cli),
        Commands::Status(args) => status_run(&cli, args),
    }
}
