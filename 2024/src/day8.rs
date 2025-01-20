use {
    aoc_core::{
        basic_grid,
        grid::{signed, SignedPosition},
        Solution,
    },
    std::collections::{HashMap, HashSet},
};

pub struct Day8;

impl Solution<'_> for Day8 {
    type Input = (HashMap<u8, Vec<SignedPosition>>, isize, isize);
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        let grid = basic_grid(data);

        let mut result: HashMap<u8, Vec<SignedPosition>> = HashMap::new();
        for x in grid.index_range() {
            let f = grid[x];
            if f != b'.' {
                result.entry(f).or_default().push(signed(x));
            }
        }

        (result, grid.n_rows() as isize, grid.n_cols() as isize)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (coords, max_row, max_col) = input;
        let mut all_nodes = HashSet::new();
        for antennas in coords.values() {
            for n in find_nodes(antennas, max_row, max_col) {
                all_nodes.insert(n);
            }
        }
        all_nodes.len()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (coords, max_row, max_col) = input;
        let mut all_nodes = HashSet::new();
        for antennas in coords.values() {
            for n in find_nodes2(antennas, max_row, max_col) {
                all_nodes.insert(n);
            }
        }
        all_nodes.len()
    }
}

fn in_bounds(x: SignedPosition, max_row: isize, max_col: isize) -> bool {
    0 <= x.0 && x.0 < max_row && 0 <= x.1 && x.1 < max_col
}

fn find_nodes(
    antennas: &[SignedPosition],
    max_row: isize,
    max_col: isize,
) -> HashSet<SignedPosition> {
    let mut nodes = HashSet::new();
    for x in antennas {
        for y in antennas {
            if x == y {
                continue;
            }
            let dx = (y.0 - x.0, y.1 - x.1);
            let z = (x.0 + 2 * dx.0, x.1 + 2 * dx.1);
            if in_bounds(z, max_row, max_col) {
                nodes.insert(z);
            }
        }
    }
    nodes
}

fn find_nodes2(
    antennas: &[SignedPosition],
    max_row: isize,
    max_col: isize,
) -> HashSet<SignedPosition> {
    let mut nodes = HashSet::new();
    for x in antennas {
        for y in antennas {
            if x == y {
                continue;
            }
            let dx = (y.0 - x.0, y.1 - x.1);
            let mut z = (x.0 + dx.0, x.1 + dx.1);
            while in_bounds(z, max_row, max_col) {
                nodes.insert(z);
                z.0 += dx.0;
                z.1 += dx.1;
            }
        }
    }
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day8_example.txt");

    #[test]
    fn test_part1() {
        let input = Day8::parse_input(EXAMPLE_INPUT);
        let output = Day8::part_1(input);
        assert_eq!(output, 14);
    }

    #[test]
    fn test_part2() {
        let input = Day8::parse_input(EXAMPLE_INPUT);
        let output = Day8::part_2(input);
        assert_eq!(output, 34);
    }
}
