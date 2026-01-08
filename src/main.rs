mod cli;
mod app;
mod util;
mod git;
mod gh;
mod config;
mod error;
mod commands;

use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    app::run(cli)
}
