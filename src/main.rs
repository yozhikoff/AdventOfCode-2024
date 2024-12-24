use reqwest::{Url, header};
use std::fs;

mod day_5;
use day_5::solution;

fn main() {
    // Read config file
    let config_content = fs::read_to_string("config.toml").expect("Failed to read config file");
    let config: toml::Table = toml::from_str(&config_content).expect("Failed to parse config file");
    let cookie_key = config["cookie_key"]
        .as_str()
        .expect("Cookie key not found in config");

    let url = "https://adventofcode.com/2024/day/5/input"
        .parse::<Url>()
        .unwrap();
    let cookie_name = "session";
    let cookie = format!("{cookie_name}={cookie_key}").to_string();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&cookie).unwrap(),
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let resp = client.get(url).send().unwrap().text().unwrap();

    solution::solve(resp)
}
