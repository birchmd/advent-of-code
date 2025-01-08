use {
    aoc_core::{basic_grid, grid::Grid, Solution},
    std::collections::HashSet,
};

pub struct Day14;

impl Solution<'_> for Day14 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let tilted = tilt_north(&input);
        calculate_load(&tilted)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let mut history = vec![input.clone()];
        let mut seen = HashSet::new();
        let mut count = 0;
        let current = loop {
            let current = history.last().unwrap();
            seen.insert(current.clone());
            count += 1;
            let next = tilt_cycle(current);
            if seen.contains(&next) {
                break next;
            } else {
                history.push(next);
            }
        };
        let cycle_start_index = history
            .iter()
            .position(|x| x == &current)
            .expect("We have seen the cycle start already");
        let cycle_len = history.len() - cycle_start_index;
        let remainder = 1_000_000_000 - count;
        let final_state = &history[cycle_start_index + (remainder % cycle_len)];
        calculate_load(final_state)
    }
}

fn tilt_cycle(grid: &Grid<u8>) -> Grid<u8> {
    let mut a = tilt_north(grid);
    tilt_west(&mut a);
    let mut b = tilt_south(&a);
    tilt_east(&mut b);
    b
}

fn tilt_west(grid: &mut Grid<u8>) {
    let n = grid.n_cols();
    for row in grid.rows.iter_mut() {
        'outer: loop {
            for i in 0..(n - 1) {
                if let (b'.', b'O') = (row[i], row[i + 1]) {
                    row[i] = b'O';
                    row[i + 1] = b'.';
                    continue 'outer;
                }
            }
            break;
        }
    }
}

fn tilt_east(grid: &mut Grid<u8>) {
    let n = grid.n_cols();
    for row in grid.rows.iter_mut() {
        'outer: loop {
            for i in 0..(n - 1) {
                if let (b'O', b'.') = (row[i], row[i + 1]) {
                    row[i] = b'.';
                    row[i + 1] = b'O';
                    continue 'outer;
                }
            }
            break;
        }
    }
}

fn tilt_north(grid: &Grid<u8>) -> Grid<u8> {
    let mut col_first = grid.transposed();
    tilt_west(&mut col_first);
    col_first.transposed()
}

fn tilt_south(grid: &Grid<u8>) -> Grid<u8> {
    let mut col_first = grid.transposed();
    tilt_east(&mut col_first);
    col_first.transposed()
}

fn calculate_load(grid: &Grid<u8>) -> usize {
    let mut row_load = grid.n_rows();
    let mut total = 0;
    for row in &grid.rows {
        let n_stones = row.iter().filter(|x| **x == b'O').count();
        total += n_stones * row_load;
        row_load -= 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day14_example.txt");

    #[test]
    fn test_part1() {
        let input = Day14::parse_input(EXAMPLE_INPUT);
        let output = Day14::part_1(input);
        assert_eq!(output, 136);
    }

    #[test]
    fn test_part2() {
        let input = Day14::parse_input(EXAMPLE_INPUT);
        let output = Day14::part_2(input);
        assert_eq!(output, 64);
    }
}
