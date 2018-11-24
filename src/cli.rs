use chrono::Datelike;
use failure::format_err;
use prettytable::{cell, row, table};
use std::{
    fmt,
    io::{self, Read},
};
use structopt::StructOpt;

mod commands;

use self::commands::{Progress, StarProgress};

#[derive(StructOpt, Clone, Copy)]
pub struct DayArg {
    /// The day of AoC to work with. Will attempt to default to the current day if not present.
    #[structopt(short = "d", long = "day")]
    day: Option<u8>,
}

impl DayArg {
    fn or_today(self) -> u8 {
        let today = chrono::offset::Local::now();
        self.day.unwrap_or(today.day() as u8)
    }
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "fetch")]
    Fetch {
        #[structopt(flatten)]
        day: DayArg,
    },
    #[structopt(name = "submit")]
    Submit {
        #[structopt(flatten)]
        day: DayArg,

        /// The level to submit the solution for.
        #[structopt(short = "l", long = "level")]
        level: u8,
    },
    #[structopt(name = "leaderboard")]
    Leaderboard,
}

impl Command {
    pub fn execute(&self, config: super::Config) -> Result<(), failure::Error> {
        match self {
            Command::Fetch { day } => {
                let day = day.or_today();
                let mut res_body = commands::fetch(config, day)
                    .map_err(|e| format_err!("Failed to fetch input data -- {}", e))?;
                io::copy(&mut res_body, &mut io::stdout().lock())?;
            }
            Command::Submit { day, level } => {
                let day = day.or_today();
                let mut answer = String::new();
                // TODO: consider using generic `Read`
                io::stdin().lock().read_to_string(&mut answer)?;
                let msg = commands::submit(config, day, *level, answer)
                    .map_err(|e| format_err!("Failed to submit solution -- {}", e))?;

                println!("{}", msg);
            }
            Command::Leaderboard => {
                for &lb in &config.leaderboards {
                    let mut table = table!([bFg => "Name", "Score", "Stars"]);
                    let mut data = commands::fetch_leaderboard(&config, lb)?;
                    data.sort_unstable_by_key(|d| -(d.local_score as i16));
                    for user in &data {
                        table.add_row(row![Fb->user.name, Fc->user.local_score, FY->user.progress]);
                    }

                    table.printstd();
                }
            }
        };

        Ok(())
    }
}

impl fmt::Display for Progress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token = match self {
            Progress::NotStarted => " ",
            Progress::LevelOne => "☆",
            Progress::LevelTwo => "★",
        };
        write!(f, "{}", token)
    }
}

impl fmt::Display for StarProgress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.0 {
            write!(f, "{}", p)?;
        }

        Ok(())
    }
}
