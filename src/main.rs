use chrono::Datelike;
use failure::format_err;
use std::{io, io::Read};
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

        if self.year.and(self.day).is_none() {
            self.year = Some(today.year() as u16);
            self.day = Some(today.day() as u8);
        }

        DateValue {
            year: self.year.unwrap(),
            day: self.day.unwrap(),
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
    fn execute(&self) -> Result<(), failure::Error> {
        let session_cookie = dotenv::var("SESSION_COOKIE")?;

        match self {
            Command::Fetch { date } => {
                let date = date.or_today();
                let mut res_body = commands::fetch(date, session_cookie)
                    .map_err(|e| format_err!("Failed to fetch input data -- {}", e))?;
                io::copy(&mut res_body, &mut io::stdout().lock())?;
            }
            Command::Submit { date, level } => {
                let date = date.or_today();
                let mut answer = String::new();
                io::stdin().lock().read_to_string(&mut answer)?;
                let mut res_body = commands::submit(date, *level, answer, session_cookie)
                    .map_err(|e| format_err!("Failed to submit solution -- {}", e))?;
                let res_text = res_body
                    .text()
                    .map_err(|e| format_err!("Malformed response -- {}", e))?;

                let msg = extract_main(&res_text);
                println!("{}", msg);
            }
        };

        Ok(())
    }
}

fn main() {
    let command = Command::from_args();
    if let Err(e) = command.execute() {
        eprintln!("Error: {}", e);
    }
}

fn extract_main(html_body: &str) -> String {
    use scraper::{Html, Selector};

    let html = Html::parse_document(html_body);
    let selector = Selector::parse("main").unwrap();
    let main = html.select(&selector).next().unwrap();
    main.text().map(str::trim).collect::<Vec<_>>().join(" ")
}
