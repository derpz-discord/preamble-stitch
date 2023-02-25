use std::path::{PathBuf};
use clap::{ArgAction, Parser, ValueEnum};
use serde::Serialize;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum, Serialize)]
pub enum Environment {
    /// Builds a preamble for use with Overleaf
    Overleaf,
    /// Builds a preamble for use with TeXit
    Texit,
}

// add to string for env
impl From<Environment> for String {
    fn from(val: Environment) -> Self {
        match val {
            Environment::Overleaf => "overleaf".to_string(),
            Environment::Texit => "texit".to_string(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    /// Increase the level of tracing/logging
    #[arg(short, long, action = ArgAction::Count)]
    pub verbosity: u8,

    /// Which environment we are building the preamble for.
    #[arg(value_enum, default_value = "texit")]
    pub environment: Environment,

    /// The config file to read out of
    #[arg(short, long, default_value = "stitchconfig.yml", value_parser = clap::value_parser!(PathBuf))]
    pub config_path: PathBuf,

}
