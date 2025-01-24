use {
    aoc_core::{
        abs_diff, basic_grid,
        grid::{dijkstra_shortest_path, Grid, Position},
        Solution,
    },
    std::collections::HashMap,
};

pub struct Day20;

impl Solution<'_> for Day20 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        num_2_ps_cheats(&input, 100)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let dist = find_distances_to_end(&input);
        find_20_ps_cheats(&dist, 100)
    }
}

fn num_2_ps_cheats(grid: &Grid<u8>, saving_threshold: u64) -> usize {
    let dist = find_distances_to_end(grid);
    let cheats = find_2_ps_cheats(&dist, grid);
    cheats.values().filter(|s| **s >= saving_threshold).count()
}

fn find_distances_to_end(grid: &Grid<u8>) -> HashMap<Position, u64> {
    let nc = grid.neighbor_context();
    let start = grid
        .index_range()
        .find(|x| grid[*x] == b'E')
        .expect("Has end");
    fn weights(_u: &Position, _v: &Position) -> u64 {
        1
    }
    let neighbors = |x: &Position| nc.cardinal_neighbors_of(*x).filter(|y| grid[*y] != b'#');

    dijkstra_shortest_path(start, weights, neighbors).0
}

fn find_2_ps_cheats(
    dist: &HashMap<Position, u64>,
    grid: &Grid<u8>,
) -> HashMap<(Position, Position), u64> {
    let nc = grid.neighbor_context();
    let mut cheats = HashMap::new();

    let walls = |x: Position| nc.cardinal_neighbors_of(x).filter(|y| grid[*y] == b'#');
    let neighbors = |x: Position| nc.cardinal_neighbors_of(x).filter(|y| grid[*y] != b'#');

    for (cheat_end, dist_end) in dist {
        for q in walls(*cheat_end) {
            for cheat_start in neighbors(q) {
                if &cheat_start == cheat_end {
                    continue;
                }
                let dist_start = dist[&cheat_start];
                if dist_start < dist_end + 2 {
                    continue;
                }
                cheats.insert((cheat_start, *cheat_end), dist_start - dist_end - 2);
            }
        }
    }

    cheats
}

fn find_20_ps_cheats(dist: &HashMap<Position, u64>, saving_threshold: u64) -> usize {
    let mut total = 0;

    for (cheat_start, dist_start) in dist {
        for (cheat_end, dist_end) in dist {
            let path_length =
                abs_diff(cheat_start.0, cheat_end.0) + abs_diff(cheat_start.1, cheat_end.1);
            if path_length > 20 {
                continue;
            }
            if *dist_start >= dist_end + (path_length as u64) + saving_threshold {
                total += 1;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day20_example.txt");

    #[test]
    fn test_part1() {
        let input = Day20::parse_input(EXAMPLE_INPUT);
        let output = num_2_ps_cheats(&input, 20);
        assert_eq!(output, 5);
    }

    #[test]
    fn test_part2() {
        let input = Day20::parse_input(EXAMPLE_INPUT);
        let dist = find_distances_to_end(&input);
        let output = find_20_ps_cheats(&dist, 50);
        assert_eq!(output, 285);
    }
}
