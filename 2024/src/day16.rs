use {
    aoc_core::{
        basic_grid,
        grid::{dijkstra_shortest_path, Grid, NeighborsCreator, Position},
        iter::AtMost,
        Solution,
    },
    std::collections::{HashMap, HashSet},
};

pub struct Day16;

const MOVEMENT_COST: u64 = 1;
const ROTATE_COST: u64 = 1000;

impl Solution<'_> for Day16 {
    type Input = Grid<u8>;
    type Output1 = u64;
    type Output2 = usize;

    fn parse_input(data: &'_ str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (dist, _) = solve_maze(&input);
        let end_position = find_end_position(&input);
        dist.into_iter()
            .filter_map(|(node, score)| {
                if node.position == end_position {
                    Some(score)
                } else {
                    None
                }
            })
            .min()
            .expect("End is reachable")
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (dist, prev) = solve_maze(&input);
        let end_position = find_end_position(&input);
        let (end_node, _) = dist
            .into_iter()
            .filter(|(n, _)| n.position == end_position)
            .min_by_key(|(_, s)| *s)
            .expect("End is reachable");
        count_path(&end_node, &prev)
    }
}

fn find_end_position(input: &Grid<u8>) -> Position {
    input
        .index_range()
        .find(|x| input[*x] == b'E')
        .expect("Has end")
}

fn count_path(end: &Node, prev: &HashMap<Node, Vec<Node>>) -> usize {
    let mut visited = HashSet::new();
    let mut path = HashSet::new();
    let mut stack = vec![end];
    while let Some(node) = stack.pop() {
        if visited.contains(node) {
            continue;
        }
        visited.insert(node);
        path.insert(node.position);
        for p in prev.get(node).into_iter().flatten() {
            stack.push(p);
        }
    }
    path.len()
}

fn solve_maze(input: &Grid<u8>) -> (HashMap<Node, u64>, HashMap<Node, Vec<Node>>) {
    let nc = input.neighbor_context();
    let start_position = input
        .index_range()
        .find(|x| input[*x] == b'S')
        .expect("Has start");
    let start_node = Node {
        position: start_position,
        direction: Direction::East,
    };

    let weights = |u: &Node, v: &Node| -> u64 {
        if u.direction != v.direction {
            ROTATE_COST
        } else {
            MOVEMENT_COST
        }
    };

    let neighbors = |x: &Node| -> AtMost<Node, 3> {
        let position = x.position;
        let direction = x.direction;
        let mut result = AtMost::some(direction.rotate().into_iter().map(|direction| Node {
            position,
            direction,
        }));
        result.inner[2] = direction
            .apply(position, &nc)
            .filter(|x| input[*x] != b'#')
            .map(|position| Node {
                position,
                direction,
            });
        result
    };

    dijkstra_shortest_path(start_node, weights, neighbors)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn apply(self, x: Position, nc: &NeighborsCreator) -> Option<Position> {
        match self {
            Self::North => nc.up(x),
            Self::East => nc.right(x),
            Self::South => nc.down(x),
            Self::West => nc.left(x),
        }
    }

    fn rotate(self) -> [Self; 2] {
        match self {
            Self::North => [Self::East, Self::West],
            Self::East => [Self::South, Self::North],
            Self::South => [Self::West, Self::East],
            Self::West => [Self::North, Self::South],
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
        assert_eq!(output, 11048);
    }

    #[test]
    fn test_part2() {
        let input = Day16::parse_input(EXAMPLE_INPUT);
        let output = Day16::part_2(input);
        assert_eq!(output, 64);
    }
}
