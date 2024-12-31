use {aoc_core::Solution, clap::Parser, std::fs};

mod cli;

const BASE_PATH: &str = env!("CARGO_MANIFEST_DIR");
const LATEST_DAY: usize = 1;
const LATEST_YEAR: usize = 2024;

fn main() {
    let args = cli::Cli::parse();
    let year = args.year.unwrap_or(LATEST_YEAR);
    let day = args.day.unwrap_or(LATEST_DAY);

    get_solution(year, day);
}

fn get_solution(year: usize, day: usize) {
    let filename = format!("{BASE_PATH}/res/{year}/day{day}.txt");
    let data = fs::read_to_string(&filename).unwrap_or_else(|e| {
        panic!("Failed to load {filename}: {e:?}");
    });
    match (year, day) {
        (2023, 1) => aoc_2023::day1::Day1::run(&data),
        (2023, 2) => aoc_2023::day2::Day2::run(&data),
        (2023, 3) => aoc_2023::day3::Day3::run(&data),
        (2023, 4) => aoc_2023::day4::Day4::run(&data),
        (2024, 1) => aoc_2024::day1::Day1::run(&data),
        other => panic!("Unknown {other:?}"),
    }
}
