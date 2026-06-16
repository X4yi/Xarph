use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The execution mode of the shell.
    #[arg(short, long, value_enum, default_value_t = Mode::Full)]
    pub mode: Mode,

    /// Config directory or shell.conf path.
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Disable StatusNotifier tray hosting and tray widgets.
    #[arg(long)]
    pub no_tray: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Mode {
    /// Run as a normal floating window (useful for UI debugging).
    Windowed,
    /// Run as a nested session inside a compositor like cage or niri --nested.
    Nested,
    /// Run natively as the full session shell via Layer Shell.
    Full,
}
