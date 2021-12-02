use reqwest::header::COOKIE;
use std::{fmt::Display, time::Instant};

const SESSION_COOKIE_FILE: &str = ".aoc.cookie";

fn read_session_cookie() -> String {
    let filename = home::home_dir()
        .expect("Failed to find home directory")
        .join(SESSION_COOKIE_FILE);
    println!("Reading session cookie from file: {:?}", filename);
    std::fs::read_to_string(filename)
        .expect("Failed to read from session file")
        .trim()
        .to_string()
}

pub fn download_input(year: u16, day: u8) -> Vec<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let cookie = format!("session={}", read_session_cookie());
    reqwest::blocking::Client::new()
        .get(&url)
        .header(COOKIE, cookie)
        .send()
        .expect("Failed to send request")
        .error_for_status()
        .expect("Got error response from server")
        .text()
        .expect("Failed to parse text from response")
        .lines()
        .map(String::from)
        .collect()
}

pub fn run<F, R>(code: F, year: u16, day: u8, part: u8)
where
    F: Fn(Vec<String>) -> R,
    R: Display,
{
    let input = download_input(year, day);
    let timer = Instant::now();
    println!("Part {} output: {}", part, code(input));
    println!("Part {} time: {:?}", part, timer.elapsed());
}
