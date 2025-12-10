use {aoc_core::Solution, std::ops::RangeInclusive};

pub struct Day5;

impl Solution<'_> for Day5 {
    type Input = (Vec<RangeInclusive<u64>>, Vec<u64>);
    type Output1 = usize;
    type Output2 = u64;

    fn parse_input(data: &'_ str) -> Self::Input {
        let (ranges, ids) = data.split_once("\n\n").expect("Separated by newline");
        let ranges = ranges
            .lines()
            .map(|line| {
                let (lower, upper) = line.split_once('-').expect("Ranges defined by -");
                let lower: u64 = lower.parse().expect("Ranges are numbers");
                let upper: u64 = upper.parse().expect("Ranges are numbers");
                lower..=upper
            })
            .collect();
        let ids: Vec<u64> = ids
            .lines()
            .map(|line| line.parse().expect("IDs are numbers"))
            .collect();
        (ranges, ids)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (ranges, ids) = input;
        ids.iter()
            .filter(|id| ranges.iter().any(|range| range.contains(*id)))
            .count()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (mut ranges, _) = input;

        // Sort ranges in descending order so that we can pop off the end
        // to get them in order from smallest to largest
        ranges.sort_by(|a, b| b.start().cmp(a.start()));

        let mut total = 0;
        let mut last_fresh_id = 0;

        while let Some(range) = ranges.pop() {
            let end = *range.end();
            if end <= last_fresh_id {
                continue;
            }
            let start = (*range.start()).max(last_fresh_id + 1);
            total += end - start + 1;
            last_fresh_id = end;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day5_example.txt");

    #[test]
    fn test_part1() {
        let input = Day5::parse_input(EXAMPLE_INPUT);
        let output = Day5::part_1(input);
        assert_eq!(output, 3);
    }

    #[test]
    fn test_part2() {
        let input = Day5::parse_input(EXAMPLE_INPUT);
        let output = Day5::part_2(input);
        assert_eq!(output, 14);
    }
}
