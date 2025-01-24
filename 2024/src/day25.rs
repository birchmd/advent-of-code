use aoc_core::{basic_grid, blocks, grid::Grid, MerryChristmas, Solution};

pub struct Day25;

impl Solution<'_> for Day25 {
    type Input = (Vec<Height>, Vec<Height>);
    type Output1 = usize;
    type Output2 = MerryChristmas;

    fn parse_input(data: &str) -> Self::Input {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        let empty = ".....";
        for grid in blocks(data) {
            let is_key = grid.starts_with(empty);
            let grid: Grid<u8> = basic_grid(grid);
            let mut height = [0u8; 5];
            for j in 0..5 {
                height[j] = (0..7).filter(|i| grid[(*i, j)] == b'#').count() as u8 - 1;
            }

            if is_key {
                keys.push(height);
            } else {
                locks.push(height);
            }
        }

        (locks, keys)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (locks, keys) = input;
        count_pairs(&locks, &keys)
    }

    fn part_2(_input: Self::Input) -> Self::Output2 {
        MerryChristmas
    }
}

pub type Height = [u8; 5];

fn count_pairs(locks: &[Height], keys: &[Height]) -> usize {
    let mut total = 0;
    for lock in locks {
        for key in keys {
            if lock.iter().zip(key).all(|(a, b)| *a + *b < 6) {
                total += 1;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day25_example.txt");

    #[test]
    fn test_part1() {
        let input = Day25::parse_input(EXAMPLE_INPUT);
        let output = Day25::part_1(input);
        assert_eq!(output, 3);
    }

    #[test]
    fn test_part2() {
        let input = Day25::parse_input(EXAMPLE_INPUT);
        let output = Day25::part_2(input);
        assert_eq!(output, MerryChristmas);
    }
}
