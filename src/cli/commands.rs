use failure::format_err;
use reqwest::{header::COOKIE, Client, Response};
use serde_json as json;
use std::convert::TryFrom;

use crate::Config;

static AOC_BASE_URL: &str = "https://adventofcode.com";

#[derive(Clone, Copy)]
pub enum Progress {
    NotStarted,
    LevelOne,
    LevelTwo,
}

impl Default for Progress {
    fn default() -> Self {
        Progress::NotStarted
    }
}

pub struct StarProgress(pub [Progress; 25]);

pub struct User {
    pub name: String,
    pub local_score: u16,
    pub progress: StarProgress,
}

pub fn fetch_leaderboard(config: &Config, id: u32) -> Result<Vec<User>, failure::Error> {
    let url = format!(
        "{base}/{year}/leaderboard/private/view/{id}.json",
        base = AOC_BASE_URL,
        year = config.year,
        id = id
    );

    let mut res = Client::new()
        .get(&url)
        .header(COOKIE, config.api_key.as_str())
        .send()
        .and_then(Response::error_for_status)?;


    let json: json::Value = res.json()?;
    json.as_object()
        .and_then(|map| map.get("members"))
        .and_then(json::Value::as_object)
        .ok_or_else(|| format_err!("Unexpected JSON format"))?
        .values()
        .map(User::try_from)
        .collect()
}

pub fn fetch(config: Config, day: u8) -> Result<reqwest::Response, reqwest::Error> {
    Client::new()
        .get(&get_aoc_url(&config, day, "input"))
        .header(COOKIE, config.api_key)
        .send()
        .and_then(Response::error_for_status)
}

pub fn submit(
    config: Config,
    day: u8,
    level: u8,
    answer: String,
) -> Result<String, reqwest::Error> {
    use maplit::hashmap;

    let mut res_body = Client::new()
        .post(&get_aoc_url(&config, day, "answer"))
        .form(&hashmap!{
             "answer" => answer,
             "level" => level.to_string(),
        })
        .header(COOKIE, config.api_key)
        .send()
        .and_then(Response::error_for_status)?;
    let res_text = res_body.text()?;

    let msg = extract_main(&res_text);
    Ok(msg)
}

fn get_aoc_url(config: &Config, day: u8, postfix: &str) -> String {
    format!(
        "{base}/{year}/day/{day}/{tail}",
        base = AOC_BASE_URL,
        year = config.year,
        day = day,
        tail = postfix
    )
}

fn extract_main(html_body: &str) -> String {
    use scraper::{Html, Selector};

    let html = Html::parse_document(html_body);
    let selector = Selector::parse("main").unwrap();
    let main = html.select(&selector).next().unwrap();
    main.text().map(str::trim).collect::<Vec<_>>().join(" ")
}

impl TryFrom<&json::Value> for User {
    type Error = failure::Error;

    fn try_from(value: &json::Value) -> Result<Self, Self::Error> {
        let name = value
            .get("name")
            .and_then(json::Value::as_str)
            .unwrap()
            .to_string();
        let local_score = value
            .get("local_score")
            .and_then(json::Value::as_u64)
            .unwrap() as u16;
        let mut problems = [Progress::default(); 25];
        let progress_map = value
            .get("completion_day_level")
            .and_then(json::Value::as_object)
            .unwrap();

        for (i, val) in progress_map.into_iter() {
            let i = i.parse::<usize>()? - 1;
            problems[i] = if val
                .as_object()
                .unwrap()
                .contains_key("2")
            {
                Progress::LevelTwo
            } else {
                Progress::LevelOne
            };
        }

        let result = User {
            name,
            local_score,
            progress: StarProgress(problems),
        };

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc_url() {
        let config = Config::default();
        let result = get_aoc_url(&config, 22, "test");
        let expected = "https://adventofcode.com/0/day/22/test";
        assert_eq!(&result, expected);
    }
}
