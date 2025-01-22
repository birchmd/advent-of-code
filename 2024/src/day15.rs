use aoc_core::{
    basic_grid,
    grid::{Grid, Position},
    Solution,
};

pub struct Day15;

impl Solution<'_> for Day15 {
    type Input = (Grid<u8>, Vec<Direction>);
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        let (grid, directions) = data.split_once("\n\n").expect("Blank line");
        let directions = directions
            .bytes()
            .filter_map(|b| match b {
                b'^' => Some(Direction::Up),
                b'v' => Some(Direction::Down),
                b'<' => Some(Direction::Left),
                b'>' => Some(Direction::Right),
                b'\n' => None,
                _ => panic!("Unknown char"),
            })
            .collect();
        (basic_grid(grid), directions)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (mut grid, moves) = input;
        let mut robot = find_robot(&grid);
        for dir in moves {
            if let Some(new_coors) = apply_move(robot, &dir, &mut grid) {
                robot = new_coors;
            }
        }
        compute_gps(&grid, b'O')
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (grid, moves) = input;
        let mut grid = scale_up(grid);
        let mut robot = find_robot(&grid);
        for dir in moves {
            let old_state = grid.clone();
            if let Some(new_coors) = apply_move2(robot, &dir, &mut grid) {
                robot = new_coors;
            } else {
                grid = old_state;
            }
        }
        compute_gps(&grid, b'[')
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, x: Position) -> Position {
        match self {
            Self::Up => (x.0 - 1, x.1),
            Self::Down => (x.0 + 1, x.1),
            Self::Left => (x.0, x.1 - 1),
            Self::Right => (x.0, x.1 + 1),
        }
    }
}

fn find_robot(grid: &Grid<u8>) -> Position {
    grid.index_range()
        .find(|x| grid[*x] == b'@')
        .expect("Has robot")
}

fn compute_gps(grid: &Grid<u8>, box_symbol: u8) -> usize {
    grid.rows
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, symbol)| {
                if symbol == &box_symbol {
                    Some(100 * i + j)
                } else {
                    None
                }
            })
        })
        .sum::<usize>()
}

fn apply_move(coords: Position, dir: &Direction, grid: &mut Grid<u8>) -> Option<Position> {
    let new_coords = dir.apply(coords);
    match grid[new_coords] {
        b'.' => {
            swap(coords, new_coords, grid);
            Some(new_coords)
        }
        b'#' => None,
        _ => {
            if apply_move(new_coords, dir, grid).is_some() {
                swap(coords, new_coords, grid);
                Some(new_coords)
            } else {
                None
            }
        }
    }
}

fn apply_move2(coords: Position, dir: &Direction, grid: &mut Grid<u8>) -> Option<Position> {
    let new_coords = dir.apply(coords);
    match grid[new_coords] {
        b'.' => {
            swap(coords, new_coords, grid);
            Some(new_coords)
        }
        b'#' => None,
        b'[' | b']' if matches!(dir, Direction::Left | Direction::Right) => {
            if apply_move2(new_coords, dir, grid).is_some() {
                swap(coords, new_coords, grid);
                Some(new_coords)
            } else {
                None
            }
        }
        b'[' => {
            let left_update = apply_move2(new_coords, dir, grid);
            let right_update = apply_move2((new_coords.0, new_coords.1 + 1), dir, grid);
            if left_update.is_some() && right_update.is_some() {
                swap(coords, new_coords, grid);
                Some(new_coords)
            } else {
                None
            }
        }
        b']' => {
            let left_update = apply_move2((new_coords.0, new_coords.1 - 1), dir, grid);
            let right_update = apply_move2(new_coords, dir, grid);
            if left_update.is_some() && right_update.is_some() {
                swap(coords, new_coords, grid);
                Some(new_coords)
            } else {
                None
            }
        }
        _ => panic!("Unknown symbol"),
    }
}

fn swap(x: Position, y: Position, grid: &mut Grid<u8>) {
    let (i, j) = x;
    let (new_i, new_j) = y;
    grid.rows[new_i][new_j] = grid[x];
    grid.rows[i][j] = b'.';
}

fn scale_up(grid: Grid<u8>) -> Grid<u8> {
    let mut rows = Vec::with_capacity(grid.rows.len());
    for row in grid.rows {
        let mut new_row = Vec::with_capacity(row.len() * 2);
        for symbol in row {
            match symbol {
                b'#' => {
                    new_row.push(b'#');
                    new_row.push(b'#');
                }
                b'O' => {
                    new_row.push(b'[');
                    new_row.push(b']');
                }
                b'.' => {
                    new_row.push(b'.');
                    new_row.push(b'.');
                }
                b'@' => {
                    new_row.push(b'@');
                    new_row.push(b'.');
                }
                _ => panic!("Unknown symbol"),
            }
        }
        rows.push(new_row);
    }
    Grid { rows }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day15_example.txt");

    #[test]
    fn test_part1() {
        let input = Day15::parse_input(EXAMPLE_INPUT);
        let output = Day15::part_1(input);
        assert_eq!(output, 10092);
    }

    #[test]
    fn test_part2() {
        let input = Day15::parse_input(EXAMPLE_INPUT);
        let output = Day15::part_2(input);
        assert_eq!(output, 9021);
    }
}
