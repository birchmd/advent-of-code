use aoc_core::{digit_value, Solution};

pub struct Day1;

impl Solution<'_> for Day1 {
    type Input = String;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &str) -> Self::Input {
        data.to_string()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        input
            .lines()
            .map(|line| {
                let mut digits = line.bytes().filter_map(digit_value);
                let first = digits.next().expect("Must have at least one digit");
                let last = digits.last().unwrap_or(first);
                (first, last)
            })
            .map(|(a, b)| (a * 10 + b) as u64)
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        const VALID_DIGITS: [(&str, u8); 19] = [
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        fn get_digits(line: &str) -> Vec<u8> {
            let mut current = line;
            let mut result = Vec::new();
            'outer: while !current.is_empty() {
                for (digit, value) in VALID_DIGITS {
                    if current.strip_prefix(digit).is_some() {
                        current = &current[1..];
                        result.push(value);
                        continue 'outer;
                    }
                }
                current = &current[1..];
            }
            result
        }

        input
            .lines()
            .map(|line| {
                let digits = get_digits(line);
                let first = digits
                    .first()
                    .copied()
                    .expect("Must have at least one digit");
                let last = digits.last().copied().unwrap_or(first);
                (first, last)
            })
            .map(|(a, b)| (a * 10 + b) as u64)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = include_str!("res/day1_example_part1.txt");
    const EXAMPLE_INPUT_2: &str = include_str!("res/day1_example_part2.txt");

    #[test]
    fn test_part1() {
        let input = Day1::parse_input(EXAMPLE_INPUT_1);
        let output = Day1::part_1(input);
        assert_eq!(output, 142);
    }

    #[test]
    fn test_part2() {
        let input = Day1::parse_input(EXAMPLE_INPUT_2);
        let output = Day1::part_2(input);
        assert_eq!(output, 281);
    }
}
