use aoc_core::{self, Solution};

pub struct Day2;

impl Solution<'_> for Day2 {
    type Input = Vec<(u64, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &'_ str) -> Self::Input {
        data.split(',')
            .map(|range| {
                let (lower, upper) = range
                    .trim()
                    .split_once('-')
                    .expect("Range separated by dash");
                let lower: u64 = lower.parse().expect("Lower bound of range is a number");
                let upper: u64 = upper.parse().expect("Upper bound of range is a number");
                debug_assert!(
                    lower <= upper,
                    "Lower bound should be less than upper bound"
                );
                (lower, upper)
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        fn is_invalid_id(id: &u64) -> bool {
            let id = *id;

            let n_digits = aoc_core::n_digits(id);

            // Must be an even number of digits to have a duplicated substring
            if !n_digits.is_multiple_of(2) {
                return false;
            }

            let prefix_length = n_digits / 2;
            let power = 10_u64.pow(prefix_length);

            // Check if first half of digits equal last half.
            id / power == id % power
        }

        input
            .into_iter()
            .flat_map(|(lower, upper)| (lower..=upper).filter(is_invalid_id))
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        fn is_invalid_id(id: &u64) -> bool {
            let id = *id;

            let n_digits = aoc_core::n_digits(id);
            for substring_len in 1..=(n_digits / 2) {
                // The substring is repeated some number of times to get the whole number.
                if !n_digits.is_multiple_of(substring_len) {
                    continue;
                }

                let m = n_digits / substring_len;
                let sub_string = sub_digits(id, substring_len, 1);
                let is_repeated =
                    (2..=m).all(|index| sub_digits(id, substring_len, index) == sub_string);
                if is_repeated {
                    return true;
                }
            }

            false
        }

        input
            .into_iter()
            .flat_map(|(lower, upper)| (lower..=upper).filter(is_invalid_id))
            .sum()
    }
}

fn sub_digits(id: u64, substring_len: u32, index: u32) -> u64 {
    let upper_power = 10_u64.pow(index * substring_len);
    let lower_power = 10_u64.pow((index - 1) * substring_len);
    (id % upper_power) / lower_power
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day2_example.txt");

    #[test]
    fn test_part1() {
        let input = Day2::parse_input(EXAMPLE_INPUT);
        let output = Day2::part_1(input);
        assert_eq!(output, 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = Day2::parse_input(EXAMPLE_INPUT);
        let output = Day2::part_2(input);
        assert_eq!(output, 4174379265);
    }

    #[test]
    fn test_sub_digits() {
        assert_eq!(sub_digits(654321, 1, 1), 1);
        assert_eq!(sub_digits(654321, 1, 3), 3);
        assert_eq!(sub_digits(654321, 1, 6), 6);

        assert_eq!(sub_digits(654321, 2, 1), 21);
        assert_eq!(sub_digits(654321, 2, 2), 43);
        assert_eq!(sub_digits(654321, 2, 3), 65);

        assert_eq!(sub_digits(654321, 3, 1), 321);
        assert_eq!(sub_digits(654321, 3, 2), 654);
    }
}
