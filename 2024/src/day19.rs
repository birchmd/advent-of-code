use {
    aoc_core::Solution,
    std::collections::{HashMap, HashSet},
};

pub struct Day19;

impl<'a> Solution<'a> for Day19 {
    type Input = (HashSet<&'a str>, Vec<&'a str>);
    type Output1 = usize;
    type Output2 = u64;

    fn parse_input(data: &'a str) -> Self::Input {
        let (towels, patterns) = data.split_once("\n\n").expect("Empty line");
        (
            towels.split(',').map(|t| t.trim()).collect(),
            patterns.lines().collect(),
        )
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (towels, patterns) = input;
        patterns
            .into_iter()
            .filter(|p| is_possible_pattern(p, &towels))
            .count()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (towels, patterns) = input;
        let mut memoize = HashMap::new();
        memoize.insert("", 1);
        patterns
            .into_iter()
            .map(|p| number_patterns(p, &towels, &mut memoize))
            .sum()
    }
}

fn is_possible_pattern<'a>(pattern: &'a str, towels: &HashSet<&'a str>) -> bool {
    if towels.contains(pattern) {
        return true;
    }

    let mut next_patterns = towels.iter().filter_map(|t| pattern.strip_prefix(t));
    next_patterns.any(|p| is_possible_pattern(p, towels))
}

fn number_patterns<'a>(
    pattern: &'a str,
    towels: &HashSet<&'a str>,
    memoize: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(answer) = memoize.get(pattern) {
        return *answer;
    }

    let mut total = 0;
    let next_patterns = towels.iter().filter_map(|t| pattern.strip_prefix(t));
    for p in next_patterns {
        let n = number_patterns(p, towels, memoize);
        memoize.insert(p, n);
        total += n;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day19_example.txt");

    #[test]
    fn test_part1() {
        let input = Day19::parse_input(EXAMPLE_INPUT);
        let output = Day19::part_1(input);
        assert_eq!(output, 6);
    }

    #[test]
    fn test_part2() {
        let input = Day19::parse_input(EXAMPLE_INPUT);
        let output = Day19::part_2(input);
        assert_eq!(output, 16);
    }
}
