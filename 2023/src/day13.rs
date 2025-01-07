use aoc_core::{basic_grid, blocks, grid::Grid, Solution};

pub struct Day13;

impl Solution<'_> for Day13 {
    type Input = Vec<Grid<u8>>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &'_ str) -> Self::Input {
        blocks(data).map(basic_grid).collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let score_part1 = |grid| score(grid, find_reflecting_row);
        input.iter().map(score_part1).sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let score_part2 = |grid| score(grid, find_reflecting_row_with_smudge);
        input.iter().map(score_part2).sum()
    }
}

fn score<F>(grid: &Grid<u8>, algorithm: F) -> u64
where
    F: Fn(&Grid<u8>) -> Option<usize>,
{
    if let Some(row) = algorithm(grid) {
        return 100 * (row as u64);
    }

    let transposed = grid.transposed();
    let Some(col) = algorithm(&transposed) else {
        panic!("Each example must have a reflection line!");
    };
    col as u64
}

fn find_reflecting_row(grid: &Grid<u8>) -> Option<usize> {
    let n = grid.n_rows();
    let adjacent_rows = grid.rows.iter().zip(grid.rows.iter().skip(1));
    'outer: for (i, (a, b)) in adjacent_rows.enumerate() {
        if a == b {
            for j in 0..i {
                let k = (i - j) + i + 1;
                if k < n && grid.rows[j] != grid.rows[k] {
                    continue 'outer;
                }
            }
            return Some(i + 1);
        }
    }
    None
}

fn find_reflecting_row_with_smudge(grid: &Grid<u8>) -> Option<usize> {
    let n = grid.n_rows();
    let adjacent_rows = grid.rows.iter().zip(grid.rows.iter().skip(1));
    'outer: for (i, (a, b)) in adjacent_rows.enumerate() {
        let diffs = count_differences(a, b);
        if diffs == 0 {
            let mut smudges = 0;
            for j in 0..i {
                let k = (i - j) + i + 1;
                if k < n {
                    smudges += count_differences(&grid.rows[j], &grid.rows[k]);
                    if smudges > 1 {
                        continue 'outer;
                    }
                }
            }
            if smudges == 1 {
                return Some(i + 1);
            }
        } else if diffs == 1 {
            for j in 0..i {
                let k = (i - j) + i + 1;
                if k < n && grid.rows[j] != grid.rows[k] {
                    continue 'outer;
                }
            }
            return Some(i + 1);
        }
    }
    None
}

fn count_differences(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).filter(|(x, y)| x != y).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day13_example.txt");

    #[test]
    fn test_part1() {
        let input = Day13::parse_input(EXAMPLE_INPUT);
        let output = Day13::part_1(input);
        assert_eq!(output, 405);
    }

    #[test]
    fn test_part2() {
        let input = Day13::parse_input(EXAMPLE_INPUT);
        let output = Day13::part_2(input);
        assert_eq!(output, 400);
    }
}
