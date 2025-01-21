use {
    aoc_core::{
        create_grid, digit_value,
        grid::{Grid, Position},
        Solution,
    },
    std::collections::HashSet,
};

pub struct Day10;

impl Solution<'_> for Day10 {
    type Input = Grid<u8>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &'_ str) -> Self::Input {
        create_grid(data, |x| digit_value(x).unwrap())
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let mut visited = HashSet::new();
        sum_scores(&input, &mut visited)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        // Part 2 is the same as Part 1 except with a no-op `VisitedTracker`.
        sum_scores(&input, &mut ())
    }
}

fn sum_scores<V>(grid: &Grid<u8>, visited: &mut V) -> u64
where
    V: VisitedTracker,
{
    grid.index_range()
        .filter_map(|x| {
            if grid[x] == 0 {
                Some(compute_score(x, grid, visited))
            } else {
                None
            }
        })
        .sum()
}

fn compute_score<V>(trail_head: Position, grid: &Grid<u8>, visited: &mut V) -> u64
where
    V: VisitedTracker,
{
    visited.reset();
    let nc = grid.neighbor_context();
    let mut score = 0;
    let mut stack = vec![(trail_head, 0)];
    while let Some((pt, h)) = stack.pop() {
        visited.add(pt);
        if h == 9 {
            score += 1;
            continue;
        }
        let required_h = h + 1;
        for x in nc.cardinal_neighbors_of(pt) {
            let next_h = grid[x];
            if next_h == required_h && !visited.has(&x) {
                stack.push((x, next_h));
            }
        }
    }
    score
}

trait VisitedTracker {
    fn add(&mut self, x: Position);
    fn has(&self, x: &Position) -> bool;
    fn reset(&mut self);
}

impl VisitedTracker for HashSet<Position> {
    fn add(&mut self, x: Position) {
        self.insert(x);
    }

    fn has(&self, x: &Position) -> bool {
        self.contains(x)
    }

    fn reset(&mut self) {
        self.clear();
    }
}

impl VisitedTracker for () {
    fn add(&mut self, _x: Position) {}

    fn has(&self, _x: &Position) -> bool {
        false
    }

    fn reset(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day10_example.txt");

    #[test]
    fn test_part1() {
        let input = Day10::parse_input(EXAMPLE_INPUT);
        let output = Day10::part_1(input);
        assert_eq!(output, 36);
    }

    #[test]
    fn test_part2() {
        let input = Day10::parse_input(EXAMPLE_INPUT);
        let output = Day10::part_2(input);
        assert_eq!(output, 81);
    }
}
