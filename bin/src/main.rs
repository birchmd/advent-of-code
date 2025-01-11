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
        (2023, 5) => aoc_2023::day5::Day5::run(&data),
        (2023, 6) => aoc_2023::day6::Day6::run(&data),
        (2023, 7) => aoc_2023::day7::Day7::run(&data),
        (2023, 8) => aoc_2023::day8::Day8::run(&data),
        (2023, 9) => aoc_2023::day9::Day9::run(&data),
        (2023, 10) => aoc_2023::day10::Day10::run(&data),
        (2023, 11) => aoc_2023::day11::Day11::run(&data),
        (2023, 12) => aoc_2023::day12::Day12::run(&data),
        (2023, 13) => aoc_2023::day13::Day13::run(&data),
        (2023, 14) => aoc_2023::day14::Day14::run(&data),
        (2023, 15) => aoc_2023::day15::Day15::run(&data),
        (2023, 16) => aoc_2023::day16::Day16::run(&data),
        (2023, 17) => aoc_2023::day17::Day17::run(&data),
        (2023, 18) => aoc_2023::day18::Day18::run(&data),
        (2023, 19) => aoc_2023::day19::Day19::run(&data),
        (2024, 1) => aoc_2024::day1::Day1::run(&data),
        other => panic!("Unknown {other:?}"),
    }
}
