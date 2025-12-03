use aoc_core::Solution;

pub struct Day3;

impl Solution<'_> for Day3 {
    type Input = Vec<Vec<u8>>;
    type Output1 = u32;
    type Output2 = u64;

    fn parse_input(data: &'_ str) -> Self::Input {
        data.lines()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|&digit| aoc_core::digit_value(digit).expect("All joltages are digits"))
                    .collect()
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        input
            .into_iter()
            .map(|jolts| largest_two_digit_jolts(&jolts))
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        input
            .into_iter()
            .map(|jolts| largest_twelve_digit_jolts(&jolts))
            .sum()
    }
}

fn largest_two_digit_jolts(jolts: &[u8]) -> u32 {
    let max_value = *jolts.iter().max().expect("Input is non-empty");
    let max_index = jolts
        .iter()
        .position(|x| x == &max_value)
        .expect("Max value is present");

    // Edge case: if the largest digit is at the end of the input
    // then we need to pick the first digit from the other digits instead.
    if max_index + 1 == jolts.len() {
        let second_max = max_value;
        let max_value = *jolts[0..max_index]
            .iter()
            .max()
            .expect("Input is non-empty");
        return (max_value * 10 + second_max) as u32;
    }

    let second_max = *jolts[(max_index + 1)..]
        .iter()
        .max()
        .expect("Input is non-empty");
    (max_value * 10 + second_max) as u32
}

fn largest_twelve_digit_jolts(jolts: &[u8]) -> u64 {
    let mut acc: u64 = 0;
    let mut remaining_digits = 12;
    let mut remaining_jolts = jolts;

    while remaining_digits > 0 {
        // We can't search for a max value too far into the list because
        // otherwise there will not be enough digits left to get all the way to 12.
        let largest_allowed_index = remaining_jolts.len() - remaining_digits;
        let current_slice = &remaining_jolts[0..=largest_allowed_index];
        let max_value = *current_slice
            .iter()
            .max()
            .expect("The slice is non-empty by construction");
        let max_index = current_slice
            .iter()
            .position(|x| x == &max_value)
            .expect("Max value is present");
        acc = 10 * acc + (max_value as u64);
        remaining_jolts = &remaining_jolts[(max_index + 1)..];
        remaining_digits -= 1;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day3_example.txt");

    #[test]
    fn test_part1() {
        let input = Day3::parse_input(EXAMPLE_INPUT);
        let output = Day3::part_1(input);
        assert_eq!(output, 357);
    }

    #[test]
    fn test_part2() {
        let input = Day3::parse_input(EXAMPLE_INPUT);
        let output = Day3::part_2(input);
        assert_eq!(output, 3121910778619);
    }
}
