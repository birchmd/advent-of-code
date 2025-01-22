use aoc_core::{blocks, digit_value, iter::AtMost, Solution};

pub struct Day13;

impl Solution<'_> for Day13 {
    type Input = Vec<Input>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input(data: &str) -> Self::Input {
        blocks(data)
            .map(|block| {
                let numbers = AtMost::<[i64; 2], 3>::some(block.lines().map(i64_pair));
                let [Some([a, b]), Some([c, d]), Some(goal)] = numbers.inner else {
                    panic!("3 lines per block")
                };
                Input {
                    matrix: [[a, c], [b, d]],
                    goal,
                }
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        total(input.into_iter())
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let extra = 10_000_000_000_000;
        total(input.into_iter().map(|x| Input {
            matrix: x.matrix,
            goal: [x.goal[0] + extra, x.goal[1] + extra],
        }))
    }
}

fn total<I>(input: I) -> i64
where
    I: Iterator<Item = Input>,
{
    input
        .filter_map(|x| solve(x.matrix, x.goal))
        .map(|[n, m]| 3 * n + m)
        .sum()
}

fn solve(matrix: [[i64; 2]; 2], goal: [i64; 2]) -> Option<[i64; 2]> {
    let [[a, b], [c, d]] = matrix;
    let det = (a * d) - (b * c);

    // Row swap to change determinant sign
    if det < 0 {
        return solve([[c, d], [a, b]], [goal[1], goal[0]]);
    }
    debug_assert!(det > 0);

    let [x, y] = goal;

    let n = (d * x) - (b * y);
    let m = (a * y) - (c * x);

    if n % det != 0 || m % det != 0 {
        return None;
    }

    Some([n / det, m / det])
}

fn i64_pair(line: &str) -> [i64; 2] {
    let (left, right) = line.split_once(',').expect("Has comma");
    [parse_i64(left), parse_i64(right)]
}

fn parse_i64(line: &str) -> i64 {
    let mut result = 0;
    let mut p10 = 1;
    for c in line.bytes().rev() {
        let Some(d) = digit_value(c) else {
            break;
        };
        result += (d as i64) * p10;
        p10 *= 10;
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    matrix: [[i64; 2]; 2],
    goal: [i64; 2],
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day13_example.txt");

    #[test]
    fn test_part1() {
        let input = Day13::parse_input(EXAMPLE_INPUT);
        let output = Day13::part_1(input);
        assert_eq!(output, 480);
    }

    #[test]
    fn test_part2() {
        let input = Day13::parse_input(EXAMPLE_INPUT);
        let output = Day13::part_2(input);
        assert_eq!(output, 875_318_608_908);
    }
}
