use {
    aoc_core::{
        basic_grid,
        grid::{Grid, NeighborsCreator, Position},
        iter::AtMost,
        Solution,
    },
    std::collections::HashSet,
};

pub struct Day16;

impl Solution<'_> for Day16 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &'_ str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let visited = follow_beam(BeamState::new((0, 0), Direction::Right), &input);
        visited.len()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let n = input.n_rows();
        let m = input.n_cols();
        let top = (0..m).map(|j| BeamState::new((0, j), Direction::Down));
        let left = (0..n).map(|i| BeamState::new((i, 0), Direction::Right));
        let bottom = (0..m).map(|j| BeamState::new((n - 1, j), Direction::Up));
        let right = (0..n).map(|i| BeamState::new((i, m - 1), Direction::Left));

        top.chain(left)
            .chain(bottom)
            .chain(right)
            .map(|initial_state| follow_beam(initial_state, &input).len())
            .max()
            .unwrap()
    }
}

fn follow_beam(initial_state: BeamState, grid: &Grid<u8>) -> HashSet<Position> {
    let mut visited = HashSet::new();
    let mut stack = vec![initial_state];
    let nc = grid.neighbor_context();

    while let Some(state) = stack.pop() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        match grid[state.position] {
            b'.' => {
                maybe_push(
                    &mut stack,
                    state.direction.update_position(state.position, &nc),
                );
            }
            b'/' => {
                let direction = state.direction.reflect_fwd();
                maybe_push(&mut stack, direction.update_position(state.position, &nc));
            }
            b'\\' => {
                let direction = state.direction.reflect_bk();
                maybe_push(&mut stack, direction.update_position(state.position, &nc));
            }
            b'-' => {
                for direction in state.direction.split_h() {
                    maybe_push(&mut stack, direction.update_position(state.position, &nc));
                }
            }
            b'|' => {
                for direction in state.direction.split_v() {
                    maybe_push(&mut stack, direction.update_position(state.position, &nc));
                }
            }
            _ => panic!("Unknown symbol"),
        }
    }

    visited.into_iter().map(|state| state.position).collect()
}

fn maybe_push<T>(xs: &mut Vec<T>, mx: Option<T>) {
    if let Some(x) = mx {
        xs.push(x);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BeamState {
    position: Position,
    direction: Direction,
}

impl BeamState {
    fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn update_position(self, x: Position, nc: &NeighborsCreator) -> Option<BeamState> {
        let position = match self {
            Self::Left => nc.left(x),
            Self::Right => nc.right(x),
            Self::Up => nc.up(x),
            Self::Down => nc.down(x),
        }?;
        Some(BeamState {
            position,
            direction: self,
        })
    }

    // Reflection from `/`
    fn reflect_fwd(&self) -> Self {
        match self {
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }

    // Reflection from `\`
    fn reflect_bk(&self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
        }
    }

    // Splitting on `-`
    fn split_h(self) -> AtMost<Self, 2> {
        match self {
            Self::Left | Self::Right => AtMost::one(self),
            Self::Up | Self::Down => AtMost::new([Self::Left, Self::Right]),
        }
    }

    // Splitting on `|`
    fn split_v(self) -> AtMost<Self, 2> {
        match self {
            Self::Left | Self::Right => AtMost::new([Self::Up, Self::Down]),
            Self::Up | Self::Down => AtMost::one(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day16_example.txt");

    #[test]
    fn test_part1() {
        let input = Day16::parse_input(EXAMPLE_INPUT);
        let output = Day16::part_1(input);
        assert_eq!(output, 46);
    }

    #[test]
    fn test_part2() {
        let input = Day16::parse_input(EXAMPLE_INPUT);
        let output = Day16::part_2(input);
        assert_eq!(output, 51);
    }
}
