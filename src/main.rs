use chrono::Datelike;
use failure::format_err;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Read},
};
use structopt::StructOpt;

mod commands;

#[derive(Clone, Copy)]
struct DateValue {
    year: u16,
    day: u8,
}

#[derive(StructOpt, Clone, Copy)]
struct DateArgs {
    /// The edition of AoC to work with. Will default to the current year if not present.
    #[structopt(short = "y", long = "year")]
    year: Option<u16>,

    /// The day of AoC to work with. Will attempt to default to the current day if not present.
    #[structopt(short = "d", long = "day")]
    day: Option<u8>,
}

impl DateArgs {
    fn or_today(mut self) -> DateValue {
        let today = chrono::offset::Local::now();

        if self.year.is_none() && self.day.is_none() {
            self.year = Some(today.year() as u16);
            self.day = Some(today.day() as u8);
        }

        DateValue {
            year: self.year.unwrap_or_default(),
            day: self.day.unwrap_or_default(),
        }
    }
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "fetch")]
    Fetch {
        #[structopt(flatten)]
        date: DateArgs,
    },
    #[structopt(name = "submit")]
    Submit {
        #[structopt(flatten)]
        date: DateArgs,

        /// The level to submit the solution for.
        #[structopt(short = "l", long = "level")]
        level: u8,
    },
}

impl Command {
    fn execute(&self, config: Config) -> Result<(), failure::Error> {
        match self {
            Command::Fetch { date } => {
                let date = date.or_today();
                let mut res_body = commands::fetch(config, date)
                    .map_err(|e| format_err!("Failed to fetch input data -- {}", e))?;
                io::copy(&mut res_body, &mut io::stdout().lock())?;
            }
            Command::Submit { date, level } => {
                let date = date.or_today();
                let mut answer = String::new();
                // TODO: consider using generic `Read`
                io::stdin().lock().read_to_string(&mut answer)?;
                let msg = commands::submit(config, date, *level, answer)
                    .map_err(|e| format_err!("Failed to submit solution -- {}", e))?;

                println!("{}", msg);
            }
        };

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    api_key: String,
    #[serde(default)]
    leaderboards: Vec<String>,
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

    let command = Command::from_args();
    command.execute(config)
}

