//! Command-line interface support for Dell Controller.

pub mod args;
pub mod ops;

use clap::Parser;

pub fn run() -> anyhow::Result<()> {
    let cli = args::Cli::parse();
    ops::execute(cli)
}
