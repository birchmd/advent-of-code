use {
    aoc_core::{min_heap::MinHeap, Solution},
    std::collections::BTreeSet,
};

pub struct Day8;

impl Solution<'_> for Day8 {
    type Input = Vec<Point3d>;
    type Output1 = usize;
    type Output2 = u64;

    fn parse_input(data: &'_ str) -> Self::Input {
        data.lines().map(Point3d::parse_line).collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        connect_shortest_distances(&input, 1000)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let mut dist_heap = build_dist_heap(&input);

        let mut circuits: Vec<BTreeSet<Point3d>> = Vec::new();
        loop {
            let (pt1, pt2) = next_connection(&mut dist_heap, &mut circuits);
            if circuits[0].len() == input.len() {
                return pt1.x * pt2.x;
            }
        }
    }
}

fn connect_shortest_distances(input: &[Point3d], n_connections: usize) -> usize {
    let mut dist_heap = build_dist_heap(input);

    // Connect the required number of closest pairs
    let mut circuits: Vec<BTreeSet<Point3d>> = Vec::new();
    for _ in 0..n_connections {
        next_connection(&mut dist_heap, &mut circuits);
    }

    // Sort by circuit size (descending order)
    circuits.sort_by_key(|c| std::cmp::Reverse(c.len()));
    // Return the product of the 3 largest circuits
    circuits[0].len() * circuits[1].len() * circuits[2].len()
}

fn build_dist_heap(input: &[Point3d]) -> MinHeap<(u64, &Point3d, &Point3d)> {
    let mut dist_heap = MinHeap::new();

    // Construct all the distance pairs.
    for pt1 in input {
        for pt2 in input {
            // Only record a pair if pt1 < pt2
            // since the distance function is symmetric.
            if pt2 <= pt1 {
                continue;
            }

            let d = pt1.dist_sq(pt2);
            dist_heap.push((d, pt1, pt2));
        }
    }

    dist_heap
}

fn next_connection<'a>(
    dist_heap: &mut MinHeap<(u64, &'a Point3d, &'a Point3d)>,
    circuits: &mut Vec<BTreeSet<Point3d>>,
) -> (&'a Point3d, &'a Point3d) {
    let (_, pt1, pt2) = dist_heap
        .pop()
        .expect("We never ask for more connections than there are pairs.");
    let maybe_existing = circuits
        .iter_mut()
        .filter(|c| c.contains(pt1) || c.contains(pt2))
        .reduce(|x, y| {
            x.append(y);
            x
        });
    match maybe_existing {
        None => {
            let mut new_circuit = BTreeSet::new();
            new_circuit.insert(pt1.clone());
            new_circuit.insert(pt2.clone());
            circuits.push(new_circuit);
        }
        Some(existing) => {
            existing.insert(pt1.clone());
            existing.insert(pt2.clone());

            // Since two circuits were merged into one then we should remove the empty one.
            circuits.retain(|c| !c.is_empty());
        }
    }

    (pt1, pt2)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point3d {
    x: u64,
    y: u64,
    z: u64,
}

impl Point3d {
    fn parse_line(line: &str) -> Self {
        let mut coords = [0_u64; 3];
        for (a, b) in coords.iter_mut().zip(line.split(',')) {
            *a = b.parse().expect("Coordinates are numbers");
        }
        Self {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    fn dist_sq(&self, other: &Self) -> u64 {
        let x = aoc_core::abs_diff(self.x, other.x);
        let y = aoc_core::abs_diff(self.y, other.y);
        let z = aoc_core::abs_diff(self.z, other.z);

        x * x + y * y + z * z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day8_example.txt");

    #[test]
    fn test_part1() {
        let input = Day8::parse_input(EXAMPLE_INPUT);
        let output = connect_shortest_distances(&input, 10);
        assert_eq!(output, 40);
    }

    #[test]
    fn test_part2() {
        let input = Day8::parse_input(EXAMPLE_INPUT);
        let output = Day8::part_2(input);
        assert_eq!(output, 25272);
    }
}
