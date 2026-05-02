use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dellctl")]
#[command(about = "Control Dell monitors over MCCS/DDC-CI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Monitors,
    Caps {
        monitor: String,
    },
    Get {
        monitor: String,
        control: String,
    },
    Set {
        monitor: String,
        control: String,
        value: String,
        #[arg(long)]
        advanced: bool,
    },
    Probe {
        monitor: String,
        #[arg(long = "write", action = ArgAction::SetFalse, default_value_t = true)]
        read_only: bool,
    },
    Snapshot {
        monitor: String,
    },
    Diff {
        before: String,
        after: String,
    },
}
