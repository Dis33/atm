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

use crate::commands::CommandDef;
use crate::types::{Package, PackageMap};
use clap::{Arg, ArgAction, ArgMatches, Command};
use tracing::{error, info};
use url::Url;

pub struct SyncCommand;

impl SyncCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandDef for SyncCommand {
    fn name(&self) -> &'static str {
        "sync"
    }

    fn command(&self) -> Command {
        Command::new(self.name())
            .short_flag('S')
            .about("Synchronize package from repository")
            .arg(
                Arg::new("name")
                    .long("name")
                    .short('n')
                    .help("Name of package (default value is repository name)")
                    .num_args(1)
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("refresh")
                    .long("refresh")
                    .short('y')
                    .help(
                        "If package is already synchronized, Update it; Otherwise, option is ignored",
                    )
                    .action(ArgAction::Set),
            )
            .arg(Arg::new("url").help("URL of package to synchronize").required(true))
    }

    fn run(&self, sub_matches: &ArgMatches) {
        let url = sub_matches.get_one::<String>("url").unwrap();
        let Ok(url) = Url::parse(url) else {
            error!("Unable to parse URL");
            return;
        };

        let default_name = url
            .path_segments()
            .map_or_default(|c| c.last())
            .map(|v| v.to_string());

        let Some(name) = sub_matches
            .get_one::<String>("name")
            .cloned()
            .or(default_name)
        else {
            error!("Couldn't get name from given URL");
            return;
        };

        let mut map = match PackageMap::from_global() {
            Ok(map) => map,
            Err(e) => {
                error!("unable to get global: {}", e);
                return;
            }
        };
        if map.contains(&name) {
            error!("Package already installed");
            return;
        }

        let (pkg, dir) = match Package::fetch(name, url) {
            Ok(t) => t,
            Err(e) => {
                error!("unable to fetch package: {}", e);
                return;
            }
        };

        let backend = match pkg.config().backend().to_backend() {
            Ok(backend) => backend,
            Err(e) => {
                error!("Error initializing of backend: {}", e);
                return;
            }
        };

        if let Err(e) = backend.install(&pkg, dir) {
            error!("{}", e);
            return;
        }

        map.add(pkg);
        info!("Package installed");
    }
}
