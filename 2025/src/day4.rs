use aoc_core::{
    grid::{Grid, NeighborsCreator, Position},
    Solution,
};

pub struct Day4;

impl Solution<'_> for Day4 {
    type Input = Grid<bool>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &'_ str) -> Self::Input {
        aoc_core::create_grid(data, |cell| cell == b'@')
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let nc = input.neighbor_context();
        accessible_positions(&input, &nc).count()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let mut grid = input;
        let nc = grid.neighbor_context();
        let mut can_remove = true;
        let mut total = 0;

        while can_remove {
            let to_remove: Vec<Position> = accessible_positions(&grid, &nc).collect();
            can_remove = !to_remove.is_empty();
            total += to_remove.len();
            for position in to_remove {
                grid[position] = false;
            }
        }

        total
    }
}

fn accessible_positions<'a>(
    grid: &'a Grid<bool>,
    nc: &'a NeighborsCreator,
) -> impl Iterator<Item = Position> + 'a {
    grid.index_range().filter(|position| {
        grid[*position]
            && nc
                .all_neighbors_of(*position)
                .filter(|neighbor| grid[*neighbor])
                .count()
                < 4
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day4_example.txt");

    #[test]
    fn test_part1() {
        let input = Day4::parse_input(EXAMPLE_INPUT);
        let output = Day4::part_1(input);
        assert_eq!(output, 13);
    }

    #[test]
    fn test_part2() {
        let input = Day4::parse_input(EXAMPLE_INPUT);
        let output = Day4::part_2(input);
        assert_eq!(output, 43);
    }
}
