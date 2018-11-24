#![feature(try_from)]

use failure::format_err;
use serde_derive::{Deserialize, Serialize};
use std::{fs, io};
use structopt::StructOpt;

mod cli;

static CONFIG: &str = "aoc.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
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
    let command = cli::Command::from_args();
    if let cli::Command::Init = command {
        return command.execute(Config::default());
    }

    let config_file = match fs::read_to_string(CONFIG) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                let err_msg = format_err!(
                    "{config} could not be found in the current directory. \
                     You can use 'aoc init' to create an example {config}",
                    config = CONFIG
                );
                Err(err_msg)?
            }
            _ => Err(e)?,
        },
    };

    let mut config = toml::from_str::<Config>(&config_file)
        .map_err(|e| format_err!("Invalid configuration: {}", e))?;
    config.prefix_key();

    command.execute(config)
}
