pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub fn run(day: usize, data: &str) {
    use aoc_core::Solution;
    match day {
        1 => day1::Day1::run(data),
        2 => day2::Day2::run(data),
        3 => day3::Day3::run(data),
        4 => day4::Day4::run(data),
        5 => day5::Day5::run(data),
        6 => day6::Day6::run(data),
        7 => day7::Day7::run(data),
        8 => day8::Day8::run(data),
        other => panic!("Unknown Day 2024 {other}"),
    }
}
