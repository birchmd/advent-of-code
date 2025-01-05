use {
    aoc_core::{
        basic_grid,
        grid::{Grid, Position},
        Solution,
    },
    std::{cmp::Ordering, collections::HashSet},
};

const CORNERS: [u8; 4] = [b'7', b'J', b'L', b'F'];

pub struct Day10;

impl Solution<'_> for Day10 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let start = input.index_of(&b'S').expect("Has starting point");
        let mut prev_a = start;
        let mut prev_b = start;
        let [mut a, mut b] = interpret_start(start, &input);
        let mut count = 1;
        while a != b {
            let c = next_position(prev_a, a, &input);
            let d = next_position(prev_b, b, &input);
            prev_a = a;
            prev_b = b;
            a = c;
            b = d;
            count += 1;
        }
        count
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let start = input.index_of(&b'S').expect("Has starting point");
        let [a, b] = interpret_start(start, &input);
        let path = follow_path(start, a, &input);
        let path_set = path.iter().copied().collect();
        let mut input = input;
        // Set the 'S' space to be the right pipe
        for pipe in CORNERS {
            input[start] = pipe;
            let neighbors = connections(start, &input);
            if neighbors == [a, b] || neighbors == [b, a] {
                break;
            }
        }
        input
            .index_range()
            .filter(|x| winding_number(*x, &path, &path_set, &input) != 0)
            .count()
    }
}

fn follow_path(start: Position, first: Position, map: &Grid<u8>) -> Vec<Position> {
    let mut result = vec![start];
    let mut prev_a = start;
    let mut a = first;
    while a != start {
        result.push(a);
        let b = next_position(prev_a, a, map);
        prev_a = a;
        a = b;
    }
    result
}

// See https://en.wikipedia.org/wiki/Point_in_polygon#Winding_number_algorithm
fn winding_number(
    x: Position,
    path: &[Position],
    path_set: &HashSet<Position>,
    map: &Grid<u8>,
) -> isize {
    if path_set.contains(&x) {
        return 0;
    }

    let n = path.len();
    let i = x.0;
    let mut prev_corner_parity: Option<isize> = None;
    let mut total = 0;
    for j in (x.1 + 1)..(map.n_cols()) {
        let mut y = (i, j);
        if path_set.contains(&y) {
            let pipe = map[y];
            let q = path.iter().position(|z| z == &y).unwrap();
            let q = (q + 1) % n;
            let z = path[q];
            if CORNERS.contains(&pipe) {
                y = path[(q + n - 2) % n];
            }
            let parity = match z.0.cmp(&y.0) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
            if pipe == b'|' {
                prev_corner_parity = None;
            } else if CORNERS.contains(&pipe) {
                // If we hit two corners with the same parity in a row
                // they only count once (e.g. F--J only counts as a single crossing).
                if prev_corner_parity == Some(parity) {
                    prev_corner_parity = None;
                    continue;
                }
                prev_corner_parity = Some(parity);
            }
            total += parity;
        }
    }
    total
}

fn next_position(prev_x: Position, x: Position, map: &Grid<u8>) -> Position {
    let [a, b] = connections(x, map);
    if a != prev_x {
        a
    } else {
        b
    }
}

// Each point on the loop is connected to two other points.
fn connections(x: Position, map: &Grid<u8>) -> [Position; 2] {
    let (i, j) = x;
    match map[x] {
        b'|' => [(i - 1, j), (i + 1, j)],
        b'-' => [(i, j - 1), (i, j + 1)],
        b'L' => [(i - 1, j), (i, j + 1)],
        b'J' => [(i - 1, j), (i, j - 1)],
        b'7' => [(i + 1, j), (i, j - 1)],
        b'F' => [(i + 1, j), (i, j + 1)],
        other => panic!("Unexpected element: {}", char::from(other)),
    }
}

// Figure out what two neighbors are connected to the start
fn interpret_start(start: Position, map: &Grid<u8>) -> [Position; 2] {
    let nc = map.neighbor_context();
    let mut neighbors = nc
        .cardinal_neighbors_of(start)
        .filter(|x| map[*x] != b'.' && connections(*x, map).contains(&start));
    let result = [neighbors.next().unwrap(), neighbors.next().unwrap()];
    assert!(neighbors.next().is_none(), "Exactly two connections");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: &str = include_str!("res/day10_example_part1.txt");
    const EXAMPLE_INPUT2: &str = include_str!("res/day10_example_part2.txt");

    #[test]
    fn test_part1() {
        let input = Day10::parse_input(EXAMPLE_INPUT1);
        let output = Day10::part_1(input);
        assert_eq!(output, 8);
    }

    #[test]
    fn test_part2() {
        let input = Day10::parse_input(EXAMPLE_INPUT2);
        let output = Day10::part_2(input);
        assert_eq!(output, 10);
    }
}
