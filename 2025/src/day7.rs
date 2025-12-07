use {
    aoc_core::{grid::Grid, Solution},
    std::collections::{BTreeSet, VecDeque},
};

pub struct Day7;

impl Solution<'_> for Day7 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = u64;

    fn parse_input(data: &'_ str) -> Self::Input {
        aoc_core::basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let mut queue = VecDeque::new();
        let start = input.rows[0]
            .iter()
            .position(|x| *x == b'S')
            .expect("Start exists");
        let mut splits = BTreeSet::new();
        queue.push_front((1_usize, start));
        while let Some(coords) = queue.pop_back() {
            let (row, col) = coords;
            if row == input.n_rows() || splits.contains(&coords) {
                continue;
            }
            if input[coords] == b'^' {
                splits.insert(coords);
                queue.push_back((row + 1, col - 1));
                queue.push_back((row + 1, col + 1));
            } else {
                queue.push_back((row + 1, col));
            }
        }
        splits.len()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let mut worlds_grid = Grid {
            rows: vec![vec![0_u64; input.n_cols()]; input.n_rows()],
        };
        let start = input.rows[0]
            .iter()
            .position(|x| *x == b'S')
            .expect("Start exists");
        let initial_coords = (1_usize, start);
        worlds_grid[initial_coords] = 1;

        for i in 1..(input.n_rows() - 1) {
            for j in 0..input.n_cols() {
                let n_worlds = worlds_grid[(i, j)];
                if n_worlds == 0 {
                    continue;
                }
                let destination = (i + 1, j);
                if input[destination] == b'^' {
                    worlds_grid[(i + 1, j - 1)] += n_worlds;
                    worlds_grid[(i + 1, j + 1)] += n_worlds;
                } else {
                    worlds_grid[destination] += n_worlds;
                }
            }
        }

        worlds_grid
            .rows
            .last()
            .expect("Grid is non-empty")
            .iter()
            .copied()
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day7_example.txt");

    #[test]
    fn test_part1() {
        let input = Day7::parse_input(EXAMPLE_INPUT);
        let output = Day7::part_1(input);
        assert_eq!(output, 21);
    }

    #[test]
    fn test_part2() {
        let input = Day7::parse_input(EXAMPLE_INPUT);
        let output = Day7::part_2(input);
        assert_eq!(output, 40);
    }
}
