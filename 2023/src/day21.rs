use {
    aoc_core::{
        basic_grid,
        grid::{Grid, Position},
        Solution,
    },
    std::collections::HashSet,
};

pub struct Day21;

impl Solution<'_> for Day21 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &'_ str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        reachable_plots(&input, find_start(&input), 64)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        // Observation: the natural length scale of the problem is the
        // size of the grid.

        // Observation: the natural scaling law of the problem is
        // quadratic (think area of a circle ~ r^2).

        // Meta-gaming: we know the answer is an integer and the given number
        // for the problem was probably chosen such that the answer can
        // be computed using these natural observations.

        let target = 26_501_365;
        let length_scale = input.n_cols();
        let offset = target % length_scale;
        let start = find_start(&input);

        // Quadratic interpolation
        let ns = (0..3).map(|x| x * length_scale + offset);
        let ys: Vec<usize> = ns
            .map(|n| reachable_plots(&input, start, n as u64))
            .collect();
        let c = ys[0];
        let a = ((ys[2] - c) - 2 * (ys[1] - c)) / 2;
        let b = ys[1] - a - c;

        let x = (target - offset) / length_scale;
        a * x * x + b * x + c
    }
}

fn find_start(grid: &Grid<u8>) -> Position {
    grid.index_range()
        .find(|x| grid[*x] == b'S')
        .expect("Has start")
}

fn reachable_plots(grid: &Grid<u8>, start: Position, max_steps: u64) -> usize {
    let n = grid.n_rows() as isize;
    let m = grid.n_cols() as isize;
    let start = (start.0 as isize, start.1 as isize);
    // Allow moving out to the rest of the infinitely tiled grid.
    let neighbors = |x: (isize, isize)| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(move |(di, dj)| (x.0 + di, x.1 + dj))
            .filter(|y| {
                let i = ((y.0 % n) + n) % n;
                let j = ((y.1 % m) + m) % m;
                grid[(i as usize, j as usize)] != b'#'
            })
    };

    let mut reachable = HashSet::new();
    let mut reached = HashSet::new();
    reached.insert(start);
    for _ in 0..max_steps {
        for x in reached.drain() {
            for y in neighbors(x) {
                reachable.insert(y);
            }
        }
        core::mem::swap(&mut reachable, &mut reached);
    }

    reached.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day21_example.txt");

    #[test]
    fn test_part1() {
        let input = Day21::parse_input(EXAMPLE_INPUT);
        let output = reachable_plots(&input, find_start(&input), 6);
        assert_eq!(output, 16);
    }

    #[test]
    fn test_part2() {
        let input = Day21::parse_input(EXAMPLE_INPUT);
        let start = find_start(&input);
        let max_steps = [6, 10, 50, 100];
        let expected = [16, 50, 1594, 6536];
        for (a, b) in max_steps.into_iter().zip(expected) {
            assert_eq!(reachable_plots(&input, start, a), b);
        }
    }
}
