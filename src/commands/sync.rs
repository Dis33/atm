use std::error::Error;
use std::fs;
use clap::{Arg, ArgAction, ArgMatches, Command};
use tracing::{error, info};
use url::Url;
use crate::commands::CommandDef;
use crate::types::{Package, PackageMap};

pub struct SyncCommand;

impl SyncCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl CommandDef for SyncCommand {
    fn name(&self) -> &'static str {
        "-S"
    }

    fn command(&self) -> Command {
        Command::new(self.name())
            .long_flag("sync")
            .arg(
                Arg::new("name")
                    .long("name")
                    .short('n')
                    .help("Name of package (default value is repository name)")
                    .action(ArgAction::Set),
            )
            .arg(Arg::new("url").required(true))
    }

    fn run(&self, sub_matches: &ArgMatches) {
        let url = sub_matches.get_one::<String>("url").unwrap();
        let Ok(url) = Url::parse(url) else {
            error!("Unable to parse URL");
            return
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
            return
        };

        let mut map = match PackageMap::from_global() {
            Ok(map) => map,
            Err(e) => {
                error!("unable to get global: {}", e);
                return
            }
        };
        if map.contains(&name) {
            error!("Package already installed");
            return
        }

        let (pkg, dir) = match Package::fetch(name, url) {
            Ok(t) => t,
            Err(e) => {
                error!("unable to fetch package: {}", e);
                return
            }
        };

        let dir_dtor = || {
            info!("Cleaning build environment...");
            if let Err(e) = fs::remove_dir_all(dir) {
                error!("Error cleaning build environment: {}", e)
            }
        };

        let backend = match pkg.config().backend().to_backend() {
            Ok(backend) => backend,
            Err(e) => {
                error!("Error initializing of backend: {}", e);
                dir_dtor();
                return
            }
        };

        backend.install(&pkg);

        map.add(pkg);
        info!("Package installed");
        dir_dtor();
    }
}
