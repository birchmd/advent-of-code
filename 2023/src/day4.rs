use {aoc_core::Solution, std::collections::HashSet};

pub struct Day4;

impl Solution for Day4 {
    type Input = Vec<(HashSet<u8>, Vec<u8>)>;

    type Output1 = u64;

    type Output2 = u64;

    fn parse_input(data: &str) -> Self::Input {
        fn to_numbers<T>(line: &str) -> T
        where
            T: FromIterator<u8>,
        {
            line.trim()
                .split_ascii_whitespace()
                .map(|x| x.parse().expect("Numbers are u8"))
                .collect()
        }

        data.lines()
            .map(|line| {
                let (_, remainder) = line.split_once(':').expect("Line has colon");
                let (winning, present) = remainder.split_once('|').expect("Line has pipe");
                (to_numbers(winning), to_numbers(present))
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        fn score_card(winning: HashSet<u8>, present: Vec<u8>) -> u64 {
            let count = present.into_iter().filter(|x| winning.contains(x)).count();
            if count == 0 {
                return 0;
            }
            1 << (count - 1)
        }

        input
            .into_iter()
            .map(|(winning, present)| score_card(winning, present))
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let n_originals = input.len();
        let mut n_copies = vec![1_u64; n_originals];
        for (i, (winning, present)) in input.into_iter().enumerate() {
            let score = present.into_iter().filter(|x| winning.contains(x)).count();
            let card_copies = n_copies[i];
            for n in n_copies[(i + 1)..].iter_mut().take(score) {
                *n += card_copies;
            }
        }
        n_copies.into_iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("day4_example.txt");

    #[test]
    fn test_part1() {
        let input = Day4::parse_input(EXAMPLE_INPUT);
        let output = Day4::part_1(input);
        assert_eq!(output, 13);
    }

    #[test]
    fn test_part2() {
        let input = Day4::parse_input(EXAMPLE_INPUT);
        let output = Day4::part_2(input);
        assert_eq!(output, 30);
    }
}
