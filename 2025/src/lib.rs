pub mod day1;
pub mod day2;
pub mod day3;

pub fn run(day: usize, data: &str) {
    use aoc_core::Solution;
    match day {
        1 => day1::Day1::run(data),
        2 => day2::Day2::run(data),
        3 => day3::Day3::run(data),
        other => panic!("Unknown Day 2025 {other}"),
    }
}
