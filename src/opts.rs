pub mod common;
mod generate;

/// Temporary workaround for allowing main.rs to access the [`VersionOpts`] enum.
pub use generate::VersionOpts;
use crate::execute::Execute;

/// ruuhid is a command line tool for generating and parsing UUIDs.
#[derive(clap::Parser)]
#[clap(about, author, version)]
pub enum Opts {
    /// Generate a UUID.
    #[clap(aliases= &["g", "gen"], version)]
    Generate {
        #[clap(subcommand)]
        version: generate::VersionOpts,
    },
    /// Parse a UUID.
    #[clap(aliases = &["p"])]
    Parse,
}

impl Opts {
    pub fn parse() -> Self {
        clap::Parser::parse()
    }
}

impl Execute for Opts {
    fn execute(&self) {
        match self {
            Opts::Generate { version } => {}
            Opts::Parse => {}
        }
    }
}