use std::cmp::Ordering;

use aoc_core::{iter::AtMost, Solution};

pub struct Day24;

impl Solution<'_> for Day24 {
    type Input = Vec<HailStone>;
    type Output1 = usize;
    type Output2 = i128;

    fn parse_input(data: &str) -> Self::Input {
        data.lines()
            .map(|line| {
                let (position, velocity) = line.split_once(" @ ").expect("Contains @");
                let position: AtMost<i128, 3> =
                    AtMost::some(position.split(',').map(|x| x.trim().parse().unwrap()));
                let velocity: AtMost<i128, 3> =
                    AtMost::some(velocity.split(',').map(|x| x.trim().parse().unwrap()));

                HailStone {
                    position: position.into(),
                    velocity: velocity.into(),
                }
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        count_xy_intersections(&input, 200_000_000_000_000, 400_000_000_000_000)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        find_trajectory(&input)
    }
}

fn count_xy_intersections(stones: &[HailStone], lower_bound: i128, upper_bound: i128) -> usize {
    let mut total = 0;
    for (i, a) in stones.iter().enumerate() {
        for b in &stones[(i + 1)..] {
            if intersects_xy(a, b, lower_bound, upper_bound) {
                total += 1;
            }
        }
    }
    total
}

fn intersects_xy(a: &HailStone, b: &HailStone, lower_bound: i128, upper_bound: i128) -> bool {
    let x_coeff = a.velocity.y * b.velocity.x - a.velocity.x * b.velocity.y;
    let y_coeff = -x_coeff;
    let x_lhs = a.velocity.y * b.velocity.x * a.position.x
        - a.velocity.x * b.velocity.y * b.position.x
        + a.velocity.x * b.velocity.x * (b.position.y - a.position.y);
    let y_lhs = a.velocity.x * b.velocity.y * a.position.y
        - a.velocity.y * b.velocity.x * b.position.y
        + a.velocity.y * b.velocity.y * (b.position.x - a.position.x);

    let crosses_inside = match x_coeff.cmp(&0) {
        Ordering::Less => {
            x_coeff * upper_bound <= x_lhs
                && x_lhs <= x_coeff * lower_bound
                && y_coeff * lower_bound <= y_lhs
                && y_lhs <= y_coeff * upper_bound
        }
        Ordering::Greater => {
            x_coeff * lower_bound <= x_lhs
                && x_lhs <= x_coeff * upper_bound
                && y_coeff * upper_bound <= y_lhs
                && y_lhs <= y_coeff * lower_bound
        }
        Ordering::Equal => {
            return x_lhs == 0;
        }
    };

    let future_for_a =
        (x_lhs - x_coeff * a.position.x).signum() == (x_coeff * a.velocity.x).signum();
    let future_for_b =
        (x_lhs - x_coeff * b.position.x).signum() == (x_coeff * b.velocity.x).signum();

    crosses_inside && future_for_a && future_for_b
}

fn find_trajectory(stones: &[HailStone]) -> i128 {
    let p1 = Triple::diff(stones[1].position, stones[0].position);
    let p2 = Triple::diff(stones[2].position, stones[0].position);
    let v1 = Triple::diff(stones[1].velocity, stones[0].velocity);
    let v2 = Triple::diff(stones[2].velocity, stones[0].velocity);

    let t1 = -dot_product(cross_product(p1, p2), v2) / dot_product(cross_product(v1, p2), v2);
    let t2 = -dot_product(cross_product(p1, p2), v1) / dot_product(cross_product(p1, v2), v1);

    let c1 = Triple::sum(stones[1].position, stones[1].velocity.scalar_mul(t1));
    let c2 = Triple::sum(stones[2].position, stones[2].velocity.scalar_mul(t2));
    let v = Triple::diff(c2, c1).scalar_div(t2 - t1);
    let p = Triple::diff(c1, v.scalar_mul(t1));
    p.x + p.y + p.z
}

fn cross_product(u: Triple, v: Triple) -> Triple {
    Triple {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

fn dot_product(u: Triple, v: Triple) -> i128 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HailStone {
    position: Triple,
    velocity: Triple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Triple {
    x: i128,
    y: i128,
    z: i128,
}

impl Triple {
    fn diff(u: Self, v: Self) -> Self {
        Self {
            x: u.x - v.x,
            y: u.y - v.y,
            z: u.z - v.z,
        }
    }

    fn sum(u: Self, v: Self) -> Self {
        Self {
            x: u.x + v.x,
            y: u.y + v.y,
            z: u.z + v.z,
        }
    }

    fn scalar_mul(&self, k: i128) -> Self {
        Self {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    fn scalar_div(&self, k: i128) -> Self {
        Self {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }
}

impl From<AtMost<i128, 3>> for Triple {
    fn from(value: AtMost<i128, 3>) -> Self {
        Self {
            x: value.inner[0].unwrap(),
            y: value.inner[1].unwrap(),
            z: value.inner[2].unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day24_example.txt");

    #[test]
    fn test_part1() {
        let input = Day24::parse_input(EXAMPLE_INPUT);
        let output = count_xy_intersections(&input, 7, 24);
        assert_eq!(output, 2);
    }

    #[test]
    fn test_part2() {
        let input = Day24::parse_input(EXAMPLE_INPUT);
        let output = Day24::part_2(input);
        assert_eq!(output, 47);
    }
}
