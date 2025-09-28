/*
 * Copyright (C) 2025  Yeong-won Seo
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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
        .about(format!(
            r#"
atm (ATM) {}
Copyright (C) Yeong-won Seo
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE."#,
            env!("CARGO_PKG_VERSION")
        ))
        .version(env!("CARGO_PKG_VERSION"))
        .disable_help_subcommand(true)
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
