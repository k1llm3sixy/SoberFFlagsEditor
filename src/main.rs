use std::{env::var, path::PathBuf};

use clap::Parser;

use crate::{cli::Cli, errors::SFFResult};

pub mod cli;
pub mod config;
pub mod errors;
pub mod flags;
fn main() -> SFFResult<()> {
    let cli = Cli::parse();

    let config_path = PathBuf::from(var("HOME")?)
        .join(".var")
        .join("app")
        .join("org.vinegarhq.Sober")
        .join("config")
        .join("sober")
        .join("config.json");

    if let Err(e) = cli.command.execute(&config_path) {
        eprintln!("{e}");
        std::process::exit(1);
    }

    Ok(())
}
