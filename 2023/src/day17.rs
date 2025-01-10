use {
    aoc_core::{
        create_grid, digit_value,
        grid::{dijkstra_shortest_path, Grid, NeighborsCreator, Position},
        iter::AtMost,
        Solution,
    },
    std::{collections::VecDeque, iter},
};

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];

pub struct Day17;

impl Solution<'_> for Day17 {
    type Input = Grid<u8>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &'_ str) -> Self::Input {
        create_grid(data, |x| {
            digit_value(x).expect("Heat loss values are numbers")
        })
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let nc = input.neighbor_context();

        let weights =
            |_from: &CrucibleNode, to: &CrucibleNode| -> u64 { input[to.position] as u64 };

        let neighbors = |x: &CrucibleNode| -> AtMost<CrucibleNode, 3> {
            let n = x.history_size();

            match n {
                0 => AtMost::some(
                    ALL_DIRECTIONS
                        .into_iter()
                        .filter_map(|d| d.update_node(x, &nc)),
                ),
                1 | 2 => {
                    let d0 = x.history[0].unwrap().inverse();
                    AtMost::some(
                        ALL_DIRECTIONS
                            .into_iter()
                            .filter(|d| d != &d0)
                            .filter_map(|d| d.update_node(x, &nc)),
                    )
                }
                _ => {
                    let d0 = x.history[0].unwrap();
                    let same_direction = x.history.iter().all(|d| *d == Some(d0));
                    if same_direction {
                        // Must turn now
                        AtMost::some(
                            ALL_DIRECTIONS
                                .into_iter()
                                .filter(|d| d != &d0 && *d != d0.inverse())
                                .filter_map(|d| d.update_node(x, &nc)),
                        )
                    } else {
                        let d0 = d0.inverse();
                        AtMost::some(
                            ALL_DIRECTIONS
                                .into_iter()
                                .filter(|d| d != &d0)
                                .filter_map(|d| d.update_node(x, &nc)),
                        )
                    }
                }
            }
        };

        let start = CrucibleNode {
            position: (0, 0),
            history: [None; 3],
        };
        let (distances, _) = dijkstra_shortest_path(start, weights, neighbors);

        let target_position = (input.n_rows() - 1, input.n_cols() - 1);
        let mut answer = u64::MAX;
        for (node, dist) in distances {
            if node.position == target_position {
                answer = std::cmp::min(answer, dist);
            }
        }

        answer
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let nc = input.neighbor_context();
        let target_position = (input.n_rows() - 1, input.n_cols() - 1);

        let weights = |_from: &UltraCrucibleNode, to: &UltraCrucibleNode| -> u64 {
            input[to.position] as u64
        };

        let neighbors = |x: &UltraCrucibleNode| -> AtMost<UltraCrucibleNode, 3> {
            let n = x.history.len();

            let d0 = x.history.front();
            let all_same = x.history.iter().all(|d| d == d0.unwrap());

            if n == 0 {
                // No direction chosen yet
                return AtMost::some(
                    ALL_DIRECTIONS
                        .into_iter()
                        .filter_map(|d| d.update_ultra_node(x, &nc)),
                );
            } else if n < 4 {
                // Must not turn
                let d0 = d0.unwrap();
                return AtMost::some(iter::once(d0.update_ultra_node(x, &nc)).flatten());
            } else if n == 10 && all_same {
                // Must turn
                let d0 = d0.unwrap();
                return AtMost::some(
                    ALL_DIRECTIONS
                        .into_iter()
                        .filter(|d| d != d0 && *d != d0.inverse())
                        .filter_map(|d| d.update_ultra_node(x, &nc))
                        .filter(|n| n.position != target_position),
                );
            }

            let d0 = d0.unwrap();
            let last_4_same = x.history.iter().take(4).all(|d| d == d0);
            let last_3_same = x.history.iter().take(3).all(|d| d == d0);
            if last_4_same {
                // Allowed to turn
                AtMost::some(
                    ALL_DIRECTIONS
                        .into_iter()
                        .filter(|d| *d != d0.inverse())
                        .filter_map(|d| d.update_ultra_node(x, &nc))
                        .filter(|n| n.history.front() == Some(d0) || n.position != target_position),
                )
            } else if last_3_same {
                // Must not turn; allowed to stop
                AtMost::some(iter::once(d0.update_ultra_node(x, &nc)).flatten())
            } else {
                // Must not turn or stop
                AtMost::some(
                    iter::once(d0.update_ultra_node(x, &nc))
                        .flatten()
                        .filter(|n| n.position != target_position),
                )
            }
        };

        let start = UltraCrucibleNode {
            position: (0, 0),
            history: VecDeque::with_capacity(10),
        };
        let (distances, _) = dijkstra_shortest_path(start, weights, neighbors);

        let mut answer = u64::MAX;
        for (node, dist) in distances {
            if node.position == target_position {
                answer = std::cmp::min(answer, dist);
            }
        }

        answer
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn update_node(self, x: &CrucibleNode, nc: &NeighborsCreator) -> Option<CrucibleNode> {
        let position = match self {
            Self::Left => nc.left(x.position),
            Self::Right => nc.right(x.position),
            Self::Up => nc.up(x.position),
            Self::Down => nc.down(x.position),
        }?;
        Some(CrucibleNode {
            position,
            history: x.push_history(self),
        })
    }

    fn update_ultra_node(
        self,
        x: &UltraCrucibleNode,
        nc: &NeighborsCreator,
    ) -> Option<UltraCrucibleNode> {
        let position = match self {
            Self::Left => nc.left(x.position),
            Self::Right => nc.right(x.position),
            Self::Up => nc.up(x.position),
            Self::Down => nc.down(x.position),
        }?;
        Some(UltraCrucibleNode {
            position,
            history: x.push_history(self),
        })
    }

    fn inverse(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CrucibleNode {
    position: Position,
    history: [Option<Direction>; 3],
}

impl CrucibleNode {
    fn history_size(&self) -> usize {
        self.history.iter().filter(|d| d.is_some()).count()
    }

    fn push_history(&self, d0: Direction) -> [Option<Direction>; 3] {
        match &self.history {
            [None, _, _] => [Some(d0), None, None],
            [Some(d1), None, _] => [Some(d0), Some(*d1), None],
            [Some(d1), Some(d2), _] => [Some(d0), Some(*d1), Some(*d2)],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct UltraCrucibleNode {
    position: Position,
    history: VecDeque<Direction>,
}

impl UltraCrucibleNode {
    fn push_history(&self, d0: Direction) -> VecDeque<Direction> {
        let mut ds = self.history.clone();
        ds.push_front(d0);
        if ds.len() > 10 {
            ds.pop_back();
        }
        ds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day17_example.txt");
    const EXAMPLE_INPUT2: &str = include_str!("res/day17_example2.txt");

    #[test]
    fn test_part1() {
        let input = Day17::parse_input(EXAMPLE_INPUT);
        let output = Day17::part_1(input);
        assert_eq!(output, 102);
    }

    #[test]
    fn test_part2() {
        let input = Day17::parse_input(EXAMPLE_INPUT);
        let output = Day17::part_2(input);
        assert_eq!(output, 94);

        let input = Day17::parse_input(EXAMPLE_INPUT2);
        let output = Day17::part_2(input);
        assert_eq!(output, 71);
    }
}
