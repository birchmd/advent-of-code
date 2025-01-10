use aoc_core::Solution;

pub struct Day18;

impl<'a> Solution<'a> for Day18 {
    type Input = Vec<(Direction, isize, &'a str)>;
    type Output1 = isize;
    type Output2 = isize;

    fn parse_input(data: &'a str) -> Self::Input {
        data.lines()
            .map(|line| {
                let (d, remainder) = line.split_once(' ').expect("Space separated");
                let (l, remainder) = remainder.split_once(' ').expect("Space separated");
                (Direction::from_str(d), l.parse().unwrap(), remainder)
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        compute_area(input.into_iter().map(|(d, l, _)| (d, l)))
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let instructions = input.into_iter().map(|(_, _, hex)| {
            let l = isize::from_str_radix(&hex[2..7], 16).expect("Hex numbers");
            let d = match &hex[7..8] {
                "0" => Direction::Right,
                "1" => Direction::Down,
                "2" => Direction::Left,
                "3" => Direction::Up,
                _ => panic!("Unknown direction"),
            };
            (d, l)
        });
        compute_area(instructions)
    }
}

fn compute_area<I>(instructions: I) -> isize
where
    I: IntoIterator<Item = (Direction, isize)>,
{
    let mut vertices = Vec::new();
    let mut current: (isize, isize) = (0, 0);
    let mut perimeter = 0;
    vertices.push(current);
    for (d, l) in instructions {
        let (di, dj) = d.delta();
        current.0 += di * l;
        current.1 += dj * l;
        vertices.push(current);
        perimeter += l;
    }

    // See https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area = 0;
    for ((yi, xi), (yj, xj)) in vertices
        .iter()
        .copied()
        .zip(vertices.iter().skip(1).copied())
    {
        area += (yi + yj) * (xi - xj)
    }
    (area.abs() / 2) + (perimeter / 2) + 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_str(data: &str) -> Self {
        match data {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => panic!("Unknown direction"),
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day18_example.txt");

    #[test]
    fn test_part1() {
        let input = Day18::parse_input(EXAMPLE_INPUT);
        let output = Day18::part_1(input);
        assert_eq!(output, 62);
    }

    #[test]
    fn test_part2() {
        let input = Day18::parse_input(EXAMPLE_INPUT);
        let output = Day18::part_2(input);
        assert_eq!(output, 952_408_144_115);
    }
}
