use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "xarph-shell")]
#[command(about = "Xarph Desktop Shell - Qt6/QML")]
pub struct Cli {
    #[arg(short, long, default_value = "full")]
    pub mode: Mode,

    #[arg(short, long)]
    pub config: Option<String>,

    #[arg(long)]
    pub no_tray: bool,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Windowed,
    Nested,
    Full,
}
