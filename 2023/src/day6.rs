use aoc_core::{isqrt, strip_label, Solution};

pub struct Day6;

impl Solution<'_> for Day6 {
    type Input = Vec<Race>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &str) -> Self::Input {
        let (times, distances) = data.split_once('\n').expect("There are two lines");
        let times = strip_label(times)
            .split_whitespace()
            .map(|x| x.parse().expect("Times are numbers"));
        let distances = strip_label(distances)
            .split_whitespace()
            .map(|x| x.parse().expect("Distances are numbers"));
        times
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let mut total = 1;
        for race in input {
            total *= n_wins(race.time, race.distance);
        }
        total
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let time: String = input.iter().map(|r| r.time.to_string()).collect();
        let distance: String = input.iter().map(|r| r.distance.to_string()).collect();
        n_wins(time.parse().unwrap(), distance.parse().unwrap())
    }
}

fn n_wins(time: u64, distance: u64) -> u64 {
    let result = |t: u64| t * (time - t);
    let center = time / 2;
    let len = isqrt(center * center - distance);
    let mut lower = center - len - 2;
    let mut upper = center + len + 2;
    while result(lower) <= distance {
        lower += 1;
    }
    while result(upper) <= distance {
        upper -= 1;
    }
    upper - lower + 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Race {
    time: u64,
    distance: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day6_example.txt");

    #[test]
    fn test_part1() {
        let input = Day6::parse_input(EXAMPLE_INPUT);
        let output = Day6::part_1(input);
        assert_eq!(output, 288);
    }

    #[test]
    fn test_part2() {
        let input = Day6::parse_input(EXAMPLE_INPUT);
        let output = Day6::part_2(input);
        assert_eq!(output, 71503);
    }
}
