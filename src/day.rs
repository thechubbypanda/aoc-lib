use std::time::Instant;

pub enum DayPart{
	One,
	Two,
}

pub trait Day {
	fn part1(input: &[u32]) -> u32;

	fn part2(input: &[u32]) -> u32;
}
