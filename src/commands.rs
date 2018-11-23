use super::{Config, DateValue};
use reqwest::{header::COOKIE, Client, Response};

pub(crate) fn fetch(config: Config, date: DateValue) -> Result<reqwest::Response, reqwest::Error> {
    Client::new()
        .get(&get_aoc_url(date, "input"))
        .header(COOKIE, config.api_key)
        .send()
        .and_then(Response::error_for_status)
}

pub(crate) fn submit(
    config: Config,
    date: DateValue,
    level: u8,
    answer: String,
) -> Result<String, reqwest::Error> {
    use maplit::hashmap;

    let mut res_body = Client::new()
        .post(&get_aoc_url(date, "answer"))
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

// pub(crate) fn fetch_leaderboard(&str) -> Vec<UserProgress> {
//     unimplemented!();
// }

fn get_aoc_url(date: DateValue, postfix: &str) -> String {
    format!(
        "https://adventofcode.com/{}/day/{}/{}",
        date.year, date.day, postfix
    )
}

fn extract_main(html_body: &str) -> String {
    use scraper::{Html, Selector};

    let html = Html::parse_document(html_body);
    let selector = Selector::parse("main").unwrap();
    let main = html.select(&selector).next().unwrap();
    main.text().map(str::trim).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc_url() {
        let date = DateValue {
            year: 2018,
            day: 22,
        };
        let result = get_aoc_url(date, "test");
        let expected = "https://adventofcode.com/2018/day/22/test";
        assert_eq!(&result, expected);
    }
}
