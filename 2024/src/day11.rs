use {
    aoc_core::{n_digits, Solution},
    std::collections::HashMap,
};

pub struct Day11;

impl Solution<'_> for Day11 {
    type Input = Vec<u64>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &str) -> Self::Input {
        data.trim().split(' ').map(|x| x.parse().unwrap()).collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let mut memoize = HashMap::new();
        input
            .into_iter()
            .map(|number| stones_length(number, 25, &mut memoize))
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let mut memoize = HashMap::new();
        input
            .into_iter()
            .map(|number| stones_length(number, 75, &mut memoize))
            .sum()
    }
}

fn stones_length(number: u64, steps: u8, memoize: &mut HashMap<(u64, u8), u64>) -> u64 {
    if steps == 0 {
        return 1;
    }

    if let Some(answer) = memoize.get(&(number, steps)) {
        return *answer;
    }

    let n_digits = n_digits(number);
    let answer = if number == 0 {
        stones_length(1, steps - 1, memoize)
    } else if n_digits % 2 == 0 {
        let p10 = 10_u64.pow(n_digits / 2);
        let left = number / p10;
        let right = number % p10;
        stones_length(left, steps - 1, memoize) + stones_length(right, steps - 1, memoize)
    } else {
        stones_length(number * 2024, steps - 1, memoize)
    };
    memoize.insert((number, steps), answer);
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        let input = Day11::parse_input(EXAMPLE_INPUT);
        let output = Day11::part_1(input);
        assert_eq!(output, 55312);
    }

    #[test]
    fn test_part2() {
        let input = Day11::parse_input(EXAMPLE_INPUT);
        let output = Day11::part_2(input);
        assert_eq!(output, 65601038650482);
    }
}
