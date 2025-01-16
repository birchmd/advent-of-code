use {
    aoc_core::Solution,
    std::{
        cmp::Reverse,
        collections::{HashMap, HashSet},
        ops::RangeInclusive,
    },
};

pub struct Day22;

impl Solution<'_> for Day22 {
    type Input = Vec<(Triple, Triple)>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &'_ str) -> Self::Input {
        data.lines()
            .map(|line| {
                let (left, right) = line.split_once('~').expect("Has tilde");
                (Triple::from_str(left), Triple::from_str(right))
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let mut bricks: Vec<Brick> = input.into_iter().map(Into::into).collect();
        let lowered_bricks = lower_bricks(&mut bricks);
        let (supporters, _) = compute_support_maps(&lowered_bricks);
        let unremovable = find_single_supporters(&supporters);
        lowered_bricks.len() - unremovable.len()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let mut bricks: Vec<Brick> = input.into_iter().map(Into::into).collect();
        let lowered_bricks = lower_bricks(&mut bricks);
        let (supporters, supporting) = compute_support_maps(&lowered_bricks);
        let single_supporters = find_single_supporters(&supporters);

        let mut total = 0;
        let mut falling = HashSet::new();
        let mut stack = Vec::new();
        for brick in single_supporters {
            stack.clear();
            falling.clear();
            falling.insert(brick);
            stack.push(brick);
            while let Some(a) = stack.pop() {
                for b in &supporting[a] {
                    if supporters[b].iter().all(|c| falling.contains(c)) {
                        falling.insert(b);
                        stack.push(b);
                    }
                }
            }
            total += falling.len() - 1;
        }
        total
    }
}

fn find_single_supporters(supporters: &HashMap<Brick, Vec<Brick>>) -> HashSet<&Brick> {
    let mut single_supporters = HashSet::new();
    for b in supporters.values() {
        if b.len() == 1 {
            single_supporters.insert(b.first().unwrap());
        }
    }
    single_supporters
}

fn compute_support_maps(
    lowered_bricks: &[Brick],
) -> (HashMap<Brick, Vec<Brick>>, HashMap<Brick, Vec<Brick>>) {
    let mut supporters: HashMap<Brick, Vec<Brick>> = lowered_bricks
        .iter()
        .map(|b| (b.clone(), Vec::new()))
        .collect();
    let mut supporting = supporters.clone();
    for a in lowered_bricks {
        for b in lowered_bricks {
            if a == b {
                continue;
            }
            if supports(a, b) {
                supporters.get_mut(b).unwrap().push(a.clone());
                supporting.get_mut(a).unwrap().push(b.clone());
            }
        }
    }
    (supporters, supporting)
}

fn lower_bricks(bricks: &mut Vec<Brick>) -> Vec<Brick> {
    bricks.sort_unstable_by_key(|brick| Reverse(brick.min_z()));

    // First block is safe to drop to the ground (no other block can be below it)
    let first_height = bricks.last().unwrap().min_z() - 1;
    bricks.last_mut().unwrap().lower(first_height);

    let mut lowered_bricks = Vec::with_capacity(bricks.len());
    lowered_bricks.push(bricks.pop().unwrap());
    while let Some(mut brick) = bricks.pop() {
        while brick.min_z() > 0 && !lowered_bricks.iter().any(|b| collides(b, &brick)) {
            brick.lower(1);
        }
        brick.raise(1);
        lowered_bricks.push(brick);
    }
    lowered_bricks
}

fn supports(a: &Brick, b: &Brick) -> bool {
    // a supports b if `collides(a, b.lower(1))`
    let mut b = b.clone();
    b.lower(1);
    collides(a, &b)
}

fn collides(a: &Brick, b: &Brick) -> bool {
    overlap(a.x(), b.x()) && overlap(a.y(), b.y()) && overlap(a.z(), b.z())
}

fn overlap(a: RangeInclusive<usize>, b: RangeInclusive<usize>) -> bool {
    if a.start() < b.start() {
        b.start() <= a.end()
    } else {
        a.start() <= b.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

enum Brick {
    Singleton(Triple),
    X {
        x: RangeInclusive<usize>,
        y: usize,
        z: usize,
    },
    Y {
        x: usize,
        y: RangeInclusive<usize>,
        z: usize,
    },
    Z {
        x: usize,
        y: usize,
        z: RangeInclusive<usize>,
    },
}

impl Brick {
    fn x(&self) -> RangeInclusive<usize> {
        match self {
            Self::Singleton(t) => (t.x)..=(t.x),
            Self::X { x, .. } => x.clone(),
            Self::Y { x, .. } | Self::Z { x, .. } => (*x)..=(*x),
        }
    }

    fn y(&self) -> RangeInclusive<usize> {
        match self {
            Self::Singleton(t) => (t.y)..=(t.y),
            Self::Y { y, .. } => y.clone(),
            Self::X { y, .. } | Self::Z { y, .. } => (*y)..=(*y),
        }
    }

    fn z(&self) -> RangeInclusive<usize> {
        match self {
            Self::Singleton(t) => (t.z)..=(t.z),
            Self::Z { z, .. } => z.clone(),
            Self::X { z, .. } | Self::Y { z, .. } => (*z)..=(*z),
        }
    }

    fn min_z(&self) -> usize {
        match self {
            Self::Singleton(t) => t.z,
            Self::X { z, .. } | Self::Y { z, .. } => *z,
            Self::Z { z, .. } => *z.start(),
        }
    }

    fn lower(&mut self, amount: usize) {
        match self {
            Brick::Singleton(triple) => triple.z -= amount,
            Brick::X { z, .. } | Brick::Y { z, .. } => *z -= amount,
            Brick::Z { z, .. } => {
                let start = *z.start();
                let end = *z.end();
                *z = (start - amount)..=(end - amount);
            }
        }
    }

    fn raise(&mut self, amount: usize) {
        match self {
            Brick::Singleton(triple) => triple.z += amount,
            Brick::X { z, .. } | Brick::Y { z, .. } => *z += amount,
            Brick::Z { z, .. } => {
                let start = *z.start();
                let end = *z.end();
                *z = (start + amount)..=(end + amount);
            }
        }
    }
}

impl From<(Triple, Triple)> for Brick {
    fn from(value: (Triple, Triple)) -> Self {
        let (a, b) = if value.0 < value.1 {
            (value.0, value.1)
        } else {
            (value.1, value.0)
        };
        if a.x != b.x {
            Self::X {
                x: (a.x)..=(b.x),
                y: a.y,
                z: a.z,
            }
        } else if a.y != b.y {
            Self::Y {
                x: a.x,
                y: (a.y)..=(b.y),
                z: a.z,
            }
        } else if a.z != b.z {
            Self::Z {
                x: a.x,
                y: a.y,
                z: (a.z)..=(b.z),
            }
        } else {
            Self::Singleton(a)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Triple {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Triple {
    fn from_str(data: &str) -> Self {
        let mut buf = [0; 3];

        for (x, b) in data.split(',').zip(buf.iter_mut()) {
            *b = x.parse().expect("Triples contain numbers");
        }

        Self {
            x: buf[0],
            y: buf[1],
            z: buf[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day22_example.txt");

    #[test]
    fn test_part1() {
        let input = Day22::parse_input(EXAMPLE_INPUT);
        let output = Day22::part_1(input);
        assert_eq!(output, 5);
    }

    #[test]
    fn test_part2() {
        let input = Day22::parse_input(EXAMPLE_INPUT);
        let output = Day22::part_2(input);
        assert_eq!(output, 7);
    }
}
