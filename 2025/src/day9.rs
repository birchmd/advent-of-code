use aoc_core::{grid::Position, Solution};

pub struct Day9;

impl Solution<'_> for Day9 {
    type Input = Vec<Position>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &'_ str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut coords = [0_usize; 2];
                for (a, b) in coords.iter_mut().zip(l.split(',')) {
                    *a = b.parse().expect("Coords are numbers");
                }
                (coords[0], coords[1])
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let mut max_area = 0;

        for a in &input {
            for b in &input {
                // The area function is symmetric, so we only
                // consider the pairs with a < b.
                if b <= a {
                    continue;
                }
                let x = aoc_core::abs_diff(a.0, b.0);
                let y = aoc_core::abs_diff(a.1, b.1);
                let area = (x + 1) * (y + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let n = input.len();
        let mut rectangles = Vec::with_capacity(input.len() * input.len() / 2);
        let mut ymax = 0;

        // Construct all rectangles and look for the largest y coordinate of any corner.
        // The max y coordinate is used to allocate memory for the edges matrix later.
        for a in &input {
            if a.1 > ymax {
                ymax = a.1;
            }

            for b in &input {
                if b <= a {
                    continue;
                }

                let rectangle = Rectangle::new(*a, *b);
                rectangles.push(rectangle);
            }
        }

        // Sort rectangles by area (decreasing)
        rectangles.sort_by_key(|r| std::cmp::Reverse(r.area()));

        // Record edges of polygon as a sparse matrix.
        // At each y coordinate only store the x coordinates that belong to edges
        let mut polygon: Vec<Vec<usize>> = vec![Vec::new(); ymax + 1];
        for (i, a) in input.iter().enumerate() {
            // Edges of the polygon are defined by neighboring inputs.
            let b = &input[(i + 1) % n];

            let (xmin, xmax) = aoc_core::min_max(a.0, b.0);
            let (ymin, ymax) = aoc_core::min_max(a.1, b.1);

            if xmin == xmax {
                // Vertical edge has points on many y values
                for xcoords in polygon.iter_mut().take(ymax + 1).skip(ymin) {
                    xcoords.push(xmin);
                }
            } else if ymin == ymax {
                // Horizontal edge has points on many x values
                for x in xmin..=xmax {
                    polygon[ymin].push(x);
                }
            } else {
                panic!("All edges are horizontal or vertical");
            }
        }

        // Look for the first rectangle (aka largest area thanks to the sorting)
        // that fits in the polygon
        let largest = rectangles
            .iter()
            .find(|rectangle| contains(&polygon, rectangle))
            .expect("There is a solution");
        largest.area()
    }
}

// Check if the given rectangle fits inside the polygon by
// comparing against the sparse matrix of edges.
// If a side of the rectangle intersects one of the edges of the polygon
// then the rectangle is not contained.
fn contains(polygon: &[Vec<usize>], rectangle: &Rectangle) -> bool {
    let (xmin, ymin) = rectangle.upper_left_corner;
    let (xmax, ymax) = rectangle.lower_right_corner;

    // Only need to check the y values that the rectangle spans
    for xcoords in polygon.iter().take(ymax).skip(ymin + 1) {
        for &edge_point in xcoords {
            // The side of the rectangle intersects an edge of the polygon
            if xmin < edge_point && edge_point < xmax {
                return false;
            }
        }
    }

    true
}

#[derive(Debug)]
struct Rectangle {
    upper_left_corner: Position,
    lower_right_corner: Position,
}

impl Rectangle {
    fn new(corner1: Position, corner2: Position) -> Self {
        let (left, right) = aoc_core::min_max(corner1.0, corner2.0);
        let (upper, lower) = aoc_core::min_max(corner1.1, corner2.1);
        Self {
            upper_left_corner: (left, upper),
            lower_right_corner: (right, lower),
        }
    }

    fn area(&self) -> usize {
        let x = self.lower_right_corner.0 - self.upper_left_corner.0;
        let y = self.lower_right_corner.1 - self.upper_left_corner.1;
        (x + 1) * (y + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day9_example.txt");

    #[test]
    fn test_part1() {
        let input = Day9::parse_input(EXAMPLE_INPUT);
        let output = Day9::part_1(input);
        assert_eq!(output, 50);
    }

    #[test]
    fn test_part2() {
        let input = Day9::parse_input(EXAMPLE_INPUT);
        let output = Day9::part_2(input);
        assert_eq!(output, 24);
    }
}
