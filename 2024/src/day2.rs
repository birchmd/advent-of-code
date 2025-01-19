use std::cmp::Ordering;

use aoc_core::Solution;

pub struct Day2;

impl Solution<'_> for Day2 {
    type Input = Vec<Vec<i32>>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        data.lines()
            .map(|line| {
                line.split(' ')
                    .map(|x| x.parse().expect("Entries are numbers"))
                    .collect()
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        input.into_iter().filter(|r| is_safe(r)).count()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        input
            .into_iter()
            .filter(|r| is_safe_with_dampener(r))
            .count()
    }
}

fn is_safe(report: &[i32]) -> bool {
    let sign = match report[0].cmp(&report[1]) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => return false,
    };
    report.iter().zip(report.iter().skip(1)).all(|(&a, &b)| {
        let diff = a - b;
        diff.signum() == sign && diff.abs() <= 3
    })
}

fn is_safe_with_dampener(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }

    let n = report.len();
    let mut alt_report = report[1..].to_vec();
    for i in 0..n {
        alt_report[0..i].copy_from_slice(&report[0..i]);
        alt_report[i..(n - 1)].copy_from_slice(&report[(i + 1)..n]);
        if is_safe(&alt_report) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day2_example.txt");

    #[test]
    fn test_part1() {
        let input = Day2::parse_input(EXAMPLE_INPUT);
        let output = Day2::part_1(input);
        assert_eq!(output, 2);
    }

    #[test]
    fn test_part2() {
        let input = Day2::parse_input(EXAMPLE_INPUT);
        let output = Day2::part_2(input);
        assert_eq!(output, 4);
    }
}
