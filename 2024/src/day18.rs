use {
    aoc_core::{
        binary_search,
        grid::{dijkstra_shortest_path, NeighborsCreator, Position},
        iter::AtMost,
        Solution,
    },
    std::collections::HashSet,
};

pub struct Day18;

impl Solution<'_> for Day18 {
    type Input = Vec<Position>;
    type Output1 = u64;
    type Output2 = String;

    fn parse_input(data: &str) -> Self::Input {
        data.lines()
            .map(|line| {
                let (x, y) = line.split_once(',').expect("Has comma");
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let walls = input[..1024].iter().copied().collect();
        shortest_path(&walls, (70, 70)).expect("Has exit path")
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let output = last_tile(&input, 1024, (70, 70));
        format!("{},{}", output.0, output.1)
    }
}

fn shortest_path(walls: &HashSet<Position>, max_position: Position) -> Option<u64> {
    fn weights(_u: &Position, _v: &Position) -> u64 {
        1
    }

    let nc = NeighborsCreator {
        n_rows: max_position.0 + 1,
        n_cols: max_position.1 + 1,
    };
    let neighbors = |u: &Position| -> AtMost<Position, 4> {
        AtMost::some(nc.cardinal_neighbors_of(*u).filter(|v| !walls.contains(v)))
    };

    let (dist, _) = dijkstra_shortest_path((0, 0), weights, neighbors);
    dist.get(&max_position).copied()
}

fn last_tile(tiles: &[Position], lower: usize, max_position: Position) -> Position {
    let condition = |index: usize| -> bool {
        let walls = tiles[..index].iter().copied().collect();
        shortest_path(&walls, max_position).is_none()
    };
    let index = binary_search(lower, tiles.len(), condition);
    tiles[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day18_example.txt");

    #[test]
    fn test_part1() {
        let input = Day18::parse_input(EXAMPLE_INPUT);
        let walls = input[..12].iter().copied().collect();
        let output = shortest_path(&walls, (6, 6)).unwrap();
        assert_eq!(output, 22);
    }

    #[test]
    fn test_part2() {
        let input = Day18::parse_input(EXAMPLE_INPUT);
        let output = last_tile(&input, 12, (6, 6));
        let output = format!("{},{}", output.0, output.1);
        assert_eq!(output, "6,1");
    }
}
