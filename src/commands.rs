use super::DateValue;
use reqwest::{header::COOKIE, Client, Response};

fn prefix_session(cookie: &mut String) {
    if !cookie.starts_with("session=") {
        cookie.insert_str(0, "session=");
    }
}

fn get_aoc_url(date: DateValue, postfix: &str) -> String {
    format!(
        "https://adventofcode.com/{}/day/{}/{}",
        date.year, date.day, postfix
    )
}

pub(crate) fn fetch<T: Into<String>>(
    date: DateValue,
    cookie: T,
) -> Result<reqwest::Response, reqwest::Error> {
    let mut cookie = cookie.into();
    prefix_session(&mut cookie);
    Client::new()
        .get(&get_aoc_url(date, "input"))
        .header(COOKIE, cookie)
        .send()
        .and_then(Response::error_for_status)
}

pub(crate) fn submit<T: Into<String>>(
    date: DateValue,
    level: u8,
    answer: String,
    cookie: T,
) -> Result<reqwest::Response, reqwest::Error> {
    use maplit::hashmap;

    let mut cookie = cookie.into();
    prefix_session(&mut cookie);
    Client::new()
        .post(&get_aoc_url(date, "answer"))
        .form(&hashmap!{
             "answer" => answer,
             "level" => level.to_string(),
        })
        .header(COOKIE, cookie)
        .send()
        .and_then(Response::error_for_status)
}
