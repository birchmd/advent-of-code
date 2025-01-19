use {aoc_core::Solution, std::collections::HashMap};

pub struct Day1;

impl Solution<'_> for Day1 {
    type Input = (Vec<u64>, Vec<u64>);
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &str) -> Self::Input {
        aoc_core::fold_lines(data, (Vec::new(), Vec::new()), |acc, line| {
            let (left, right) = acc;
            let mut iter = line.split_whitespace();
            left.push(iter.next().unwrap().parse().unwrap());
            right.push(iter.next().unwrap().parse().unwrap());
        })
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (mut left, mut right) = input;

        left.sort_unstable();
        right.sort_unstable();

        let mut total = 0;
        for (a, b) in left.into_iter().zip(right) {
            if a < b {
                total += b - a;
            } else {
                total += a - b;
            }
        }
        total
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (left, right) = input;
        let mut counts: HashMap<u64, u64> = HashMap::new();

        for r in right {
            let entry = counts.entry(r).or_default();
            *entry += 1;
        }

        let mut total = 0;
        for m in left {
            let n = counts.get(&m).copied().unwrap_or_default();
            total += n * m;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day1_example.txt");

    #[test]
    fn test_part1() {
        let input = Day1::parse_input(EXAMPLE_INPUT);
        let output = Day1::part_1(input);
        assert_eq!(output, 11);
    }

    #[test]
    fn test_part2() {
        let input = Day1::parse_input(EXAMPLE_INPUT);
        let output = Day1::part_2(input);
        assert_eq!(output, 31);
    }
}
