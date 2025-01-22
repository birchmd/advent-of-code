use aoc_core::{grid::Grid, strip_label, Solution};

pub struct Day14;

impl Solution<'_> for Day14 {
    type Input = Vec<Robot>;
    type Output1 = i64;
    type Output2 = ();

    fn parse_input(data: &str) -> Self::Input {
        data.lines()
            .map(|line| {
                let (position, velocity) = line.split_once(' ').expect("Space separated");
                let position = Pair::from_str(strip_label(position, '='));
                let velocity = Pair::from_str(strip_label(velocity, '='));
                Robot { position, velocity }
            })
            .collect()
    }

    fn part_1(mut input: Self::Input) -> Self::Output1 {
        part_1_solution(&mut input, 101, 103)
    }

    fn part_2(mut input: Self::Input) -> Self::Output2 {
        let max_x = 101;
        let max_y = 103;

        for t in 1..1_000_000 {
            simulate_step(&mut input, max_x, max_y);
            let grid = render(&input, max_x as usize, max_y as usize);
            // When in christmas tree formation, all robots are in different positions
            if grid.rows.iter().flatten().all(|x| *x <= 1) {
                let grid = Grid {
                    rows: grid
                        .rows
                        .into_iter()
                        .map(|r| {
                            r.into_iter()
                                .map(|x| if x == 1 { '1' } else { '.' })
                                .collect()
                        })
                        .collect(),
                };
                let picture = grid.render();
                // The christmas tree is framed, so look for a long
                // line of single robots
                if picture.contains("11111111111111111111") {
                    println!("{picture}");
                    println!("{t}");
                    break;
                }
            }
        }
    }
}

fn render(robots: &[Robot], max_x: usize, max_y: usize) -> Grid<u8> {
    let mut grid = Grid {
        rows: vec![vec![0; max_x]; max_y],
    };
    for r in robots {
        let Pair { x, y } = r.position;
        grid.rows[y as usize][x as usize] += 1;
    }
    grid
}

fn part_1_solution(robots: &mut [Robot], max_x: i64, max_y: i64) -> i64 {
    for _ in 0..100 {
        simulate_step(robots, max_x, max_y);
    }
    safety_score(robots, max_x, max_y)
}

fn simulate_step(robots: &mut [Robot], max_x: i64, max_y: i64) {
    for r in robots {
        let Pair { x, y } = r.position;
        let Pair { x: vx, y: vy } = r.velocity;
        r.position = Pair {
            x: (x + vx + max_x) % max_x,
            y: (y + vy + max_y) % max_y,
        }
    }
}

fn safety_score(robots: &[Robot], max_x: i64, max_y: i64) -> i64 {
    let mut total = [0, 0, 0, 0];

    let is_q1 = |p: Pair| p.x < max_x / 2 && p.y < max_y / 2;
    let is_q2 = |p: Pair| p.x < max_x / 2 && p.y > max_y / 2;
    let is_q3 = |p: Pair| p.x > max_x / 2 && p.y > max_y / 2;
    let is_q4 = |p: Pair| p.x > max_x / 2 && p.y < max_y / 2;

    let qs: [Box<dyn Fn(Pair) -> bool>; 4] = [
        Box::new(is_q1),
        Box::new(is_q2),
        Box::new(is_q3),
        Box::new(is_q4),
    ];

    for r in robots {
        for (t, is_q) in total.iter_mut().zip(&qs) {
            if is_q(r.position) {
                *t += 1;
                break;
            }
        }
    }

    total[0] * total[1] * total[2] * total[3]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    x: i64,
    y: i64,
}

impl Pair {
    fn from_str(line: &str) -> Self {
        let (x, y) = line.split_once(',').expect("Comma separated");
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Robot {
    position: Pair,
    velocity: Pair,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day14_example.txt");

    #[test]
    fn test_part1() {
        let mut input = Day14::parse_input(EXAMPLE_INPUT);
        let output = part_1_solution(&mut input, 11, 7);
        assert_eq!(output, 12);
    }
}
