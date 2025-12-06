use aoc_core::Solution;

pub struct Day6;

impl<'a> Solution<'a> for Day6 {
    type Input = &'a str;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &'a str) -> Self::Input {
        data
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let problems = MathProblem::parse_part1(input);
        MathProblem::solve_problems(problems)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let problems = MathProblem::parse_part2(input);
        MathProblem::solve_problems(problems)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathProblem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl MathProblem {
    fn parse_part1(data: &str) -> Vec<Self> {
        let rows: Vec<Vec<&str>> = data
            .lines()
            .map(|line| line.trim().split_ascii_whitespace().collect())
            .collect();
        let n_rows = rows.len();
        let last_row = n_rows - 1;
        let n_cols = rows[0].len();
        debug_assert!(
            rows.iter().all(|row| row.len() == n_cols),
            "Rows should all line up"
        );
        (0..n_cols)
            .map(|j| {
                let numbers: Vec<u64> = rows[0..last_row]
                    .iter()
                    .map(|row| row[j].parse().expect("Math problem has numbers"))
                    .collect();
                let operation = match rows[last_row][j] {
                    "+" => Operation::Plus,
                    "*" => Operation::Times,
                    _ => panic!("Unexpected operation"),
                };
                MathProblem { numbers, operation }
            })
            .collect()
    }

    fn parse_part2(data: &str) -> Vec<Self> {
        let grid = aoc_core::basic_grid(data);
        // Transpose the grid so that accessing data column by column is easier.
        let grid = grid.transposed();
        let operation_position = grid.n_cols() - 1;

        let problem_breaks =
            (0..grid.n_rows()).filter(|i| grid.rows[*i].iter().all(|cell| *cell == b' '));

        let mut problem_start = 0;
        let mut problems = Vec::new();
        // Need to chain on an extra index because there is no break after the last problem
        for problem_end in problem_breaks.chain(std::iter::once(grid.n_rows())) {
            let numbers: Vec<u64> = grid.rows[problem_start..problem_end]
                .iter()
                .map(|row| {
                    let number_str = str::from_utf8(&row[0..operation_position])
                        .expect("Grid contains ascii chars");
                    number_str
                        .trim()
                        .parse()
                        .expect("Each row in transposed grid is a number")
                })
                .collect();
            let operation = match grid.rows[problem_start][operation_position] {
                b'+' => Operation::Plus,
                b'*' => Operation::Times,
                _ => panic!("Unexpected operation"),
            };
            problems.push(MathProblem { numbers, operation });
            problem_start = problem_end + 1;
        }
        problems
    }

    pub fn solve_problems(problems: Vec<Self>) -> u64 {
        problems
            .into_iter()
            .map(|problem| {
                let operation = problem.operation.as_fn();
                problem
                    .numbers
                    .into_iter()
                    .reduce(operation)
                    .expect("Problems are not empty")
            })
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Plus,
    Times,
}

impl Operation {
    fn plus_fn(a: u64, b: u64) -> u64 {
        a + b
    }

    fn times_fn(a: u64, b: u64) -> u64 {
        a * b
    }

    pub fn as_fn(self) -> fn(u64, u64) -> u64 {
        match self {
            Self::Plus => Self::plus_fn,
            Self::Times => Self::times_fn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day6_example.txt");

    #[test]
    fn test_part1() {
        let input = Day6::parse_input(EXAMPLE_INPUT);
        let output = Day6::part_1(input);
        assert_eq!(output, 4277556);
    }

    #[test]
    fn test_part2() {
        let input = Day6::parse_input(EXAMPLE_INPUT);
        let output = Day6::part_2(input);
        assert_eq!(output, 3263827);
    }
}
