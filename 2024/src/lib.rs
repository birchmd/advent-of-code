pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

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
        9 => day9::Day9::run(data),
        10 => day10::Day10::run(data),
        11 => day11::Day11::run(data),
        12 => day12::Day12::run(data),
        13 => day13::Day13::run(data),
        14 => day14::Day14::run(data),
        15 => day15::Day15::run(data),
        16 => day16::Day16::run(data),
        17 => day17::Day17::run(data),
        18 => day18::Day18::run(data),
        19 => day19::Day19::run(data),
        20 => day20::Day20::run(data),
        21 => day21::Day21::run(data),
        22 => day22::Day22::run(data),
        23 => day23::Day23::run(data),
        24 => day24::Day24::run(data),
        other => panic!("Unknown Day 2024 {other}"),
    }
}
