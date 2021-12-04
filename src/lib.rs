use std::fmt::Display;
use std::time::Instant;
use client::Client;

mod client;
mod util;

pub use util::*;

pub fn run<F, R>(year: u16, day: u8, code: F)
where
    F: Fn(String) -> R,
    R: Display,
{
    let client = Client::new(year);
    let input = client.get_input(day);
    let timer = Instant::now();
    println!("Output: {}", code(input));
    println!("Time: {:?}", timer.elapsed());
}

pub fn test<F, R>(_: u16, _: u8, code: F)
where
    F: Fn(String) -> R,
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
