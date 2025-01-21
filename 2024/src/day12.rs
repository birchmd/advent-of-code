use {
    aoc_core::{
        basic_grid,
        grid::{Grid, Position},
        Solution,
    },
    std::collections::HashSet,
};

pub struct Day12;

impl Solution<'_> for Day12 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let regions = create_regions(&input);
        regions
            .iter()
            .map(|r| calculate_price(r, &input))
            .sum::<usize>()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let regions = create_regions(&input);
        regions
            .iter()
            .map(|r| calculate_bulk_price(r, &input))
            .sum::<usize>()
    }
}

fn fill_from(
    start: Position,
    remaining: &mut HashSet<Position>,
    grid: &Grid<u8>,
) -> HashSet<Position> {
    let nc = grid.neighbor_context();
    let plant = grid[start];
    let mut region = HashSet::new();
    let mut stack = vec![start];
    while let Some(x) = stack.pop() {
        if !remaining.remove(&x) {
            continue;
        }
        region.insert(x);
        for y in nc.cardinal_neighbors_of(x) {
            if grid[y] == plant {
                stack.push(y);
            }
        }
    }
    region
}

fn create_regions(grid: &Grid<u8>) -> Vec<HashSet<Position>> {
    let mut remaining: HashSet<Position> = grid.index_range().collect();
    let mut regions = Vec::new();

    while let Some(start) = remaining.iter().next().copied() {
        regions.push(fill_from(start, &mut remaining, grid));
    }

    regions
}

fn calculate_price(region: &HashSet<Position>, grid: &Grid<u8>) -> usize {
    let nc = grid.neighbor_context();
    let area = region.len();
    let perimeter = region
        .iter()
        .map(|x| {
            4 - nc
                .cardinal_neighbors_of(*x)
                .filter(|y| region.contains(y))
                .count()
        })
        .sum::<usize>();

    area * perimeter
}

fn calculate_bulk_price(region: &HashSet<Position>, grid: &Grid<u8>) -> usize {
    let nc = grid.neighbor_context();
    let area = region.len();
    let mut n_vertices = 0;

    let in_region =
        |x: Option<Position>| -> bool { x.map(|x| region.contains(&x)).unwrap_or(false) };

    for &x in region {
        let is_top_edge = !in_region(nc.up(x));
        let is_bottom_edge = !in_region(nc.down(x));
        let is_left_edge = !in_region(nc.left(x));
        let is_right_edge = !in_region(nc.right(x));

        if is_top_edge && is_left_edge {
            // Corner of the form
            // BB
            // BA
            n_vertices += 1;
        }

        if is_top_edge && is_right_edge {
            // Corner of the form
            // BB
            // AB
            n_vertices += 1;
        }

        if is_bottom_edge && is_left_edge {
            // Corner of the form
            // BA
            // BB
            n_vertices += 1;
        }

        if is_bottom_edge && is_right_edge {
            // Corner of the form
            // AB
            // BB
            n_vertices += 1;
        }

        if !is_top_edge && !is_left_edge && !in_region(nc.up_left(x)) {
            // Corner of the form
            // BA
            // AA
            n_vertices += 1;
        }

        if !is_top_edge && !is_right_edge && !in_region(nc.up_right(x)) {
            // Corner of the form
            // AB
            // AA
            n_vertices += 1;
        }

        if !is_bottom_edge && !is_left_edge && !in_region(nc.down_left(x)) {
            // Corner of the form
            // AA
            // BA
            n_vertices += 1;
        }

        if !is_bottom_edge && !is_right_edge && !in_region(nc.down_right(x)) {
            // Corner of the form
            // AA
            // AB
            n_vertices += 1;
        }
    }

    // Euler's characteristic for a planar graph
    let n_sides = n_vertices;
    area * n_sides
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day12_example.txt");

    #[test]
    fn test_part1() {
        let input = Day12::parse_input(EXAMPLE_INPUT);
        let output = Day12::part_1(input);
        assert_eq!(output, 1930);
    }

    #[test]
    fn test_part2() {
        let input = Day12::parse_input(EXAMPLE_INPUT);
        let output = Day12::part_2(input);
        assert_eq!(output, 1206);
    }
}
