pub mod day1;

pub fn run(day: usize, data: &str) {
    use aoc_core::Solution;
    match day {
        1 => day1::Day1::run(data),
        other => panic!("Unknown Day 2025 {other}"),
    }
}
