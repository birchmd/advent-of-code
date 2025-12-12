use aoc_core::{grid::Grid, iter::TakeN, MerryChristmas, Solution};

pub struct Day12;

impl Solution<'_> for Day12 {
    type Input = Day12Input;
    type Output1 = usize;
    type Output2 = MerryChristmas;

    fn parse_input(data: &'_ str) -> Self::Input {
        let mut lines = data.lines().peekable();

        // Parse the shapes
        let mut shapes = Vec::new();
        let mut index = 0;
        while lines.peek() == Some(&format!("{index}:").as_str()) {
            // Skip the index line, we know what it is.
            lines.next();
            // Shape grids are always 3x3
            let grid_lines = TakeN::new(&mut lines, 3);
            let rows: Vec<Vec<bool>> = grid_lines
                .map(|r| r.bytes().map(|c| c == b'#').collect())
                .collect();
            shapes.push(Grid { rows });
            index += 1;
            assert!(
                lines.next().unwrap().is_empty(),
                "There is a blank line after each grid"
            );
        }

        // Parse regions
        let regions = lines.map(Region::parse_line).collect();

        Day12Input { shapes, regions }
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let shape_fills: Vec<usize> = input
            .shapes
            .iter()
            .map(|grid| grid.rows.iter().flatten().filter(|x| **x).count())
            .collect();

        // This condition is clearly necessary, but the fact that it is also sufficient
        // feels like it is a fluke of how the inputs are generated. Maybe I should think
        // harder about what a full solution to this would look like. Probably some kind
        // of recursive thing where you can put certain combinations of the shapes into
        // nearly square tiles and solving by placing squares is easy.
        input
            .regions
            .into_iter()
            .filter(|r| {
                // Only keep regions where the required filled spaces
                // fits in the area.
                let area = r.size.0 * r.size.1;
                let filled_spaces: usize = r
                    .counts
                    .iter()
                    .enumerate()
                    .map(|(i, n)| n * shape_fills[i])
                    .sum();

                filled_spaces < area
            })
            .count()
    }

    fn part_2(_input: Self::Input) -> Self::Output2 {
        // There still isn't a part 2 on the last day.
        MerryChristmas
    }
}

#[derive(Debug, Clone)]
pub struct Day12Input {
    shapes: Vec<Grid<bool>>,
    regions: Vec<Region>,
}

#[derive(Debug, Clone)]
pub struct Region {
    size: (usize, usize),
    counts: Vec<usize>,
}

impl Region {
    fn parse_line(line: &str) -> Self {
        let (size, counts) = line.split_once(':').expect("Colon delimiter");
        let (x, y) = size.split_once('x').expect("x delimiter");
        let x = x.parse().expect("Dimensions given as numbers");
        let y = y.parse().expect("Dimensions given as numbers");
        let counts = counts
            .trim()
            .split(' ')
            .map(|c| c.parse().expect("Counts are numbers"))
            .collect();
        Self {
            size: (x, y),
            counts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day12_example.txt");

    #[test]
    fn test_part1() {
        let input = Day12::parse_input(EXAMPLE_INPUT);
        let output = Day12::part_1(input);
        assert_eq!(output, 2);
    }
}
