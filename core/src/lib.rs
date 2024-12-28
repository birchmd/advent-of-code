use std::fmt::Debug;

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
