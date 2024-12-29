use {self::grid::Grid, std::fmt::Debug};

pub mod grid;
pub mod linked_list;

pub trait Solution {
    type Input: Clone;
    type Output1: Debug;
    type Output2: Debug;

    fn parse_input(data: &str) -> Self::Input;
    fn part_1(input: Self::Input) -> Self::Output1;
    fn part_2(input: Self::Input) -> Self::Output2;

    fn run(data: &str) {
        let input = Self::parse_input(data);

        let part1 = Self::part_1(input.clone());
        println!("{part1:?}");

        let part2 = Self::part_2(input);
        println!("{part2:?}");
    }
}

pub fn fold_lines<T, F: Fn(&mut T, &str)>(data: &str, init: T, acc: F) -> T {
    let mut result = init;
    for line in data.lines() {
        acc(&mut result, line);
    }
    result
}

pub fn digit_value(digit: u8) -> Option<u8> {
    const ZERO: u8 = b'0';
    const NINE: u8 = b'9';
    if (ZERO..=NINE).contains(&digit) {
        Some(digit - ZERO)
    } else {
        None
    }
}

/// Note: digits must be in order such that least significant is first.
pub fn construct_base_10<I>(digits: I) -> u64
where
    I: IntoIterator<Item = u8>,
{
    digits
        .into_iter()
        .fold((0_u64, 1_u64), |(acc, power), d| {
            (acc + power * (d as u64), power * 10)
        })
        .0
}

pub fn basic_grid(data: &str) -> Grid<u8> {
    let rows = data.lines().map(|row| row.bytes().collect()).collect();
    Grid { rows }
}

pub fn create_grid<T, F>(data: &str, element_constructor: F) -> Grid<T>
where
    F: Fn(u8) -> T,
{
    let rows = data
        .lines()
        .map(|row| row.bytes().map(&element_constructor).collect())
        .collect();
    Grid { rows }
}
