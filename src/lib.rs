use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use reqwest::header::COOKIE;

fn read_session_cookie() -> String {
    let filename = home::home_dir()
        .expect("Failed to find home directory")
        .join(".aoc.cookie");
    println!("Reading session cookie from file: {:?}", filename);
    std::fs::read_to_string(filename)
        .expect("Failed to read from session file")
        .trim()
        .to_string()
}

pub fn run<F, R>(year: u16, day: u8, code: F)
where
    F: Fn(Vec<String>) -> R,
    R: Display,
{
    let client = Client::new(year);
    let input = client.get_input(day).lines().map(String::from).collect();
    let timer = Instant::now();
    println!("Output: {}", code(input));
    println!("Time: {:?}", timer.elapsed());
}

pub fn test<F, R>(_: u16, _: u8, code: F)
where
    F: Fn(Vec<String>) -> R,
    R: Display,
{
    let input = std::fs::read_to_string("input/test.in")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    let timer = Instant::now();
    println!("Output: {}", code(input));
    println!("Time: {:?}", timer.elapsed());
}

struct Client {
    year: u16,
    session_token: String,
    client: reqwest::blocking::Client,
    input_dir: PathBuf,
}

impl Client {
    pub fn new(year: u16) -> Self {
        let input_dir = Path::new("input").to_path_buf();
        if let Err(e) = std::fs::create_dir(input_dir.clone()) {
            println!("Didn't create input dir: {}", e.to_string());
        }
        Self {
            input_dir,
            year,
            session_token: read_session_cookie(),
            client: reqwest::blocking::Client::new(),
        }
    }

    fn get_day_path(&self, day: u8) -> PathBuf {
        self.input_dir.join(format!("{}.in", day))
    }

    pub fn get_input(&self, day: u8) -> String {
        if let Ok(input) = std::fs::read_to_string(self.get_day_path(day)) {
            return input;
        }

        let input = self.download_input(day);
        self.cache_input(day, &input);

        input
    }

    fn cache_input(&self, day: u8, input: &str) {
        File::create(self.get_day_path(day))
            .expect("Failed to create input file")
            .write_all(input.as_bytes())
            .expect("Failed to write input file");
    }

    fn download_input(&self, day: u8) -> String {
        self.client
            .get(&format!(
                "https://adventofcode.com/{}/day/{}/input",
                self.year, day
            ))
            .header(COOKIE, format!("session={}", self.session_token))
            .send()
            .expect("Failed to sent input request to server")
            .error_for_status()
            .expect("Got error from server")
            .text()
            .expect("Failed to parse response from server")
    }
}
