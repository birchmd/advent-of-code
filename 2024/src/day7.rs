use aoc_core::{iter::AtMost, Solution};

pub struct Day7;

impl Solution<'_> for Day7 {
    type Input = Vec<Data>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &str) -> Self::Input {
        data.lines()
            .map(|line| {
                let (value, input) = line.split_once(':').expect("Has colon");
                let value = value.parse().unwrap();
                let input = input
                    .trim()
                    .split(' ')
                    .map(|x| x.parse().expect("Is number"))
                    .collect();
                Data {
                    value,
                    inputs: input,
                }
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        input
            .into_iter()
            .filter_map(|data| {
                if check_data(data.value, data.inputs) {
                    Some(data.value)
                } else {
                    None
                }
            })
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        input
            .into_iter()
            .filter_map(|data| {
                if check_data_with_concat(data.value, data.inputs) {
                    Some(data.value)
                } else {
                    None
                }
            })
            .sum()
    }
}

fn check_data(value: u64, mut inputs: Vec<u64>) -> bool {
    let n = inputs.len();

    if n == 1 {
        return value == inputs[0];
    }

    let rightmost = inputs.pop().unwrap();
    if !value.is_multiple_of(rightmost) {
        check_data(value - rightmost, inputs)
    } else {
        check_data(value / rightmost, inputs.clone()) || check_data(value - rightmost, inputs)
    }
}

fn check_data_with_concat(value: u64, mut inputs: Vec<u64>) -> bool {
    let n = inputs.len();

    if n == 1 {
        return value == inputs[0];
    }

    let rightmost = inputs.pop().unwrap();
    let mut possibilities: AtMost<(u64, Vec<u64>), 3> =
        AtMost::one((value - rightmost, inputs.clone()));
    if value.is_multiple_of(rightmost) {
        possibilities.push((value / rightmost, inputs.clone()));
    }
    let stv = value.to_string();
    let srm = rightmost.to_string();
    if value != rightmost && stv.ends_with(&srm) {
        let m = stv.len() - srm.len();
        possibilities.push((stv[0..m].parse().unwrap(), inputs));
    }
    possibilities
        .into_iter()
        .any(|(x, y)| check_data_with_concat(x, y))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    value: u64,
    inputs: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day7_example.txt");

    #[test]
    fn test_part1() {
        let input = Day7::parse_input(EXAMPLE_INPUT);
        let output = Day7::part_1(input);
        assert_eq!(output, 3749);
    }

    #[test]
    fn test_part2() {
        let input = Day7::parse_input(EXAMPLE_INPUT);
        let output = Day7::part_2(input);
        assert_eq!(output, 11387);
    }
}
