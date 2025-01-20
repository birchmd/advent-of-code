use {
    aoc_core::{
        basic_grid,
        grid::{Grid, Position},
        Solution,
    },
    std::collections::HashSet,
};

pub struct Day6;

impl Solution<'_> for Day6 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        get_path(&input, get_initial_position(&input)).len()
    }

    fn part_2(mut input: Self::Input) -> Self::Output2 {
        let initial_position = get_initial_position(&input);
        let mut original_path = get_path(&input, initial_position);
        original_path.remove(&initial_position);

        let mut total = 0;
        for obstacle_position in original_path {
            input[obstacle_position] = b'#';

            if is_loop(&input, initial_position) {
                total += 1;
            }

            input[obstacle_position] = b'.';
        }
        total
    }
}

fn get_initial_position(grid: &Grid<u8>) -> Position {
    grid.index_range()
        .find(|x| grid[*x] == b'^')
        .expect("Has start")
}

fn get_path(grid: &Grid<u8>, initial_position: Position) -> HashSet<Position> {
    let nc = grid.neighbor_context();
    let directions = nc.cardinal_directions();

    let mut result = HashSet::new();
    let mut guard_position = initial_position;
    let mut direction = 0;
    result.insert(guard_position);

    while let Some(mut next_position) = directions[direction](guard_position) {
        while grid[next_position] == b'#' {
            direction = (direction + 1) % 4;
            next_position = directions[direction](guard_position).unwrap();
        }
        guard_position = next_position;
        result.insert(guard_position);
    }

    result
}

fn is_loop(grid: &Grid<u8>, initial_position: Position) -> bool {
    let nc = grid.neighbor_context();
    let directions = nc.cardinal_directions();

    let mut history: HashSet<(Position, usize)> = HashSet::new();
    let mut guard_position = initial_position;
    let mut direction = 0;
    history.insert((guard_position, direction));

    while let Some(mut next_position) = directions[direction](guard_position) {
        while grid[next_position] == b'#' {
            direction = (direction + 1) % 4;
            next_position = directions[direction](guard_position).unwrap();
        }
        guard_position = next_position;
        let vector = (guard_position, direction);
        if history.contains(&vector) {
            return true;
        }
        history.insert(vector);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day6_example.txt");

    #[test]
    fn test_part1() {
        let input = Day6::parse_input(EXAMPLE_INPUT);
        let output = Day6::part_1(input);
        assert_eq!(output, 41);
    }

    #[test]
    fn test_part2() {
        let input = Day6::parse_input(EXAMPLE_INPUT);
        let output = Day6::part_2(input);
        assert_eq!(output, 6);
    }
}
