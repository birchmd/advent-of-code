use {
    aoc_core::{
        basic_grid,
        grid::{Grid, Position},
        iter::AtMost,
        Solution,
    },
    std::collections::HashSet,
};

pub struct Day23;

impl Solution<'_> for Day23 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let start = find_start(&input);
        let end = find_end(&input);

        let nc = input.neighbor_context();
        let neighbors = |x: Position| -> AtMost<Position, 4> {
            match input[x] {
                b'>' => AtMost::some(nc.right(x)),
                b'<' => AtMost::some(nc.left(x)),
                b'^' => AtMost::some(nc.up(x)),
                b'v' => AtMost::some(nc.down(x)),
                _ => AtMost::some(nc.cardinal_neighbors_of(x).filter(|y| input[*y] != b'#')),
            }
        };

        longest_path(start, end, neighbors)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let start = find_start(&input);
        let end = find_end(&input);

        let nc = input.neighbor_context();
        let neighbors = |x: Position| -> AtMost<Position, 4> {
            AtMost::some(nc.cardinal_neighbors_of(x).filter(|y| input[*y] != b'#'))
        };

        longest_path(start, end, neighbors)
    }
}

fn find_start(grid: &Grid<u8>) -> Position {
    let start_col = grid.rows[0]
        .iter()
        .position(|b| *b == b'.')
        .expect("There is a start");
    (0, start_col)
}

fn find_end(grid: &Grid<u8>) -> Position {
    let n = grid.n_rows();
    let end_col = grid.rows[n - 1]
        .iter()
        .position(|b| *b == b'.')
        .expect("There is an end");
    (n - 1, end_col)
}

fn longest_path<F>(start: Position, end: Position, neighbors: F) -> usize
where
    F: Fn(Position) -> AtMost<Position, 4>,
{
    let mut stack = vec![(start, 0, HashSet::new())];
    let mut complete = Vec::new();
    while let Some((x, s, mut visited)) = stack.pop() {
        if visited.contains(&x) {
            continue;
        }
        if x == end {
            complete.push(s);
            continue;
        }
        visited.insert(x);
        for y in neighbors(x).into_iter().filter(|y| !visited.contains(y)) {
            stack.push((y, s + 1, visited.clone()));
        }
    }
    complete.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day23_example.txt");

    #[test]
    fn test_part1() {
        let input = Day23::parse_input(EXAMPLE_INPUT);
        let output = Day23::part_1(input);
        assert_eq!(output, 94);
    }

    #[test]
    fn test_part2() {
        let input = Day23::parse_input(EXAMPLE_INPUT);
        let output = Day23::part_2(input);
        assert_eq!(output, 154);
    }
}
