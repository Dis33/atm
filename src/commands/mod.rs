use clap::{ArgMatches, Command};

mod sync;

pub trait CommandDef {
    fn name(&self) -> &'static str;
    fn command(&self) -> Command;
    fn run(&self, sub_matches: &ArgMatches);
}

pub use sync::SyncCommand;
