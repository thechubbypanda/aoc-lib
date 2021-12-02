const SESSION_COOKIE_FILE: &str = ".aoc.session";

fn read_session_cookie() -> String {
    std::fs::read_to_string(
        home::home_dir()
            .expect("Failed to find home directory")
            .join(SESSION_COOKIE_FILE),
    )
    .expect("Failed to read from session file")
}

pub fn get_input(file: String) -> Vec<String> {
    std::fs::read_to_string(file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[macro_export]
macro_rules! run {
    ($day:expr, $part: expr) => {
        paste::item! {
            let input = aoc::get_input(format!("{}.in", $day));
            let timer = Instant::now();
            println!("Part {} output: {}", $part, [<day$day>]::[<part$part>](&input));
            println!("Part {} time: {:?}", $part, timer.elapsed());
        }
    };
}
