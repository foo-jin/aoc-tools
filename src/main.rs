#![feature(try_from)]

use failure::format_err;
use serde_derive::{Deserialize, Serialize};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};
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

    let current_dir = env::current_dir()?;
    let config_loc = find(current_dir, CONFIG)?;
    let config_str = match fs::read_to_string(config_loc) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                let err_msg = format_err!(
                    "{config} could not be found in the current directory \
                     or any of its parent directories. Use 'aoc init' \
                     to create an example {config}",
                    config = CONFIG
                );
                Err(err_msg)?
            }
            _ => Err(e)?,
        },
    };

    let mut config = toml::from_str::<Config>(&config_str)
        .map_err(|e| format_err!("Invalid configuration: {}", e))?;
    config.prefix_key();

    command.execute(config)
}

/// Searches for `filename` in `directory` and parent directories until found or root is reached.
pub fn find<S, T>(directory: S, filename: T) -> io::Result<PathBuf>
where
    S: AsRef<Path>,
    T: AsRef<Path>,
{
    let filename = filename.as_ref();
    let directory = directory.as_ref();
    let candidate = directory.join(filename);

    match fs::metadata(&candidate) {
        Ok(metadata) => {
            if metadata.is_file() {
                return Ok(candidate);
            }
        }
        Err(e) => {
            if e.kind() != io::ErrorKind::NotFound {
                return Err(e);
            }
        }
    }

    if let Some(parent) = directory.parent() {
        find(parent, filename)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "path not found"))
    }
}
