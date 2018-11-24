#![feature(try_from)]

use failure::format_err;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use structopt::StructOpt;

mod cli;

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    api_key: String,
    year: u16,
    #[serde(default)]
    leaderboards: Vec<u32>,
}

impl Config {
    fn prefix_key(&mut self) {
        if !self.api_key.starts_with("session=") {
            self.api_key.insert_str(0, "session=");
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), failure::Error> {
    let config_file = fs::read_to_string("aoc.toml")?;
    // TODO: generate config file
    let mut config = toml::from_str::<Config>(&config_file)
        .map_err(|e| format_err!("Invalid configuration: {}", e))?;
    config.prefix_key();

    let command = cli::Command::from_args();
    command.execute(config)
}
