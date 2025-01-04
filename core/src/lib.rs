use {
    self::grid::Grid,
    std::{collections::HashMap, fmt::Debug, hash::Hash},
};

pub mod grid;
pub mod linked_list;

pub trait Solution<'a> {
    type Input: Clone + 'a;
    type Output1: Debug;
    type Output2: Debug;

    fn parse_input(data: &'a str) -> Self::Input;
    fn part_1(input: Self::Input) -> Self::Output1;
    fn part_2(input: Self::Input) -> Self::Output2;

    fn run(data: &'a str) {
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

// Splits data at each empty line
pub fn blocks(data: &str) -> impl Iterator<Item = &str> {
    data.split("\n\n")
}

pub fn strip_label(data: &str) -> &str {
    data.split_once(':').expect("Has colon").1.trim()
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

pub fn isqrt(x: u64) -> u64 {
    let mut lower = 0;
    let mut upper = x + 1;
    while lower != upper - 1 {
        let middle = (lower + upper) / 2;
        if middle * middle <= x {
            lower = middle;
        } else {
            upper = middle;
        }
    }
    lower
}

pub fn gcd(x: u64, y: u64) -> u64 {
    let (mut a, mut b) = if x < y { (y, x) } else { (x, y) };
    let mut c = a % b;
    while c != 0 {
        a = b;
        b = c;
        c = a % b;
    }
    b
}

pub fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

pub fn count_distinct<'a, T, I>(iter: I) -> HashMap<&'a T, usize>
where
    T: Eq + Hash,
    I: IntoIterator<Item = &'a T>,
{
    let mut result = HashMap::new();
    for t in iter {
        let entry = result.entry(t);
        let count = entry.or_default();
        *count += 1;
    }
    result
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

#[test]
fn test_gcd() {
    assert_eq!(gcd(462, 1071), 21);
}
