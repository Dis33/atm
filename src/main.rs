#![feature(result_option_map_or_default)]

mod backends;
mod commands;
mod types;

use crate::commands::{CommandDef, SyncCommand};
use clap::Command;
use constcat::concat;
use tracing_subscriber::prelude::*;

const ATM_RESOURCE_DIR: &str = ".";
const ATM_PACKAGES_FILE: &str = concat!(ATM_RESOURCE_DIR, "/packages.toml");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let subcommands: Vec<Box<dyn CommandDef>> = vec![Box::new(SyncCommand::new())];

    let mut command = Command::new("atm")
        .about("agent tooling manager utility")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true);

    for subcommand in &subcommands {
        command = command.subcommand(subcommand.command());
    }

    let matches = command.get_matches();
    let (name, sub_matches) = matches.subcommand().unwrap();

    for subcommand in &subcommands {
        if name == subcommand.name() {
            subcommand.run(sub_matches);
        }
    }

    Ok(())
}
