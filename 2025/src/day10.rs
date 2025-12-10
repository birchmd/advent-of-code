use {
    aoc_core::Solution,
    good_lp::{
        default_solver, variable, variables, Expression, Solution as _, SolverModel, Variable,
    },
    std::{
        collections::{HashSet, VecDeque},
        ops::BitXor,
    },
};

pub struct Day10;

impl Solution<'_> for Day10 {
    type Input = Vec<Machine>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input(data: &'_ str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut parts = l.split(' ').peekable();

                let lights_schematic = parts.next().expect("Lights come first");
                let lights = LightsState::parse(lights_schematic);

                let mut buttons = Vec::new();
                while parts.peek().is_some_and(|s| s.starts_with('(')) {
                    let button_schematic = parts.next().expect("Is Some because we peeked");
                    buttons.push(Button::parse(button_schematic));
                }

                let joltages = parts.next().expect("Joltages are last");
                let n = joltages.len() - 1;
                let joltages = joltages[1..n]
                    .split(',')
                    .map(|j| j.parse().expect("Joltages are numbers"))
                    .collect();
                Machine {
                    target_lights: lights,
                    buttons,
                    joltages,
                }
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        input.into_iter().map(|m| m.lights_bfs()).sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        input.into_iter().map(|m| m.joltage_solve()).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightsState(u16);

impl LightsState {
    fn parse(schematic: &str) -> Self {
        let lights_len = schematic.len() - 1;

        let (value, _) =
            schematic[1..lights_len]
                .bytes()
                .fold((0, 1), |(value, offset), c| match c {
                    b'#' => (value ^ offset, offset << 1),
                    b'.' => (value, offset << 1),
                    _ => panic!("Unexpected char in lights schematic"),
                });

        Self(value)
    }
}

impl BitXor<Button> for LightsState {
    type Output = Self;

    fn bitxor(self, rhs: Button) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Button(u16);

impl Button {
    fn parse(schematic: &str) -> Self {
        let buttons_len = schematic.len() - 1;

        let value = schematic[1..buttons_len].split(',').fold(0, |acc, s| {
            let offset: u16 = s.parse().expect("Buttons described by numbers");
            let value = 1 << offset;
            acc ^ value
        });

        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machine {
    target_lights: LightsState,
    buttons: Vec<Button>,
    joltages: Vec<u32>,
}

impl Machine {
    fn lights_bfs(&self) -> u32 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((LightsState(0), 0));

        while let Some((state, n_presses)) = queue.pop_front() {
            if state == self.target_lights {
                return n_presses;
            }
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state);

            let next_presses = n_presses + 1;
            for b in &self.buttons {
                let new_state = state ^ *b;
                queue.push_back((new_state, next_presses));
            }
        }

        panic!("No solution found!");
    }

    fn joltage_solve(&self) -> u32 {
        let mut problem = variables!();

        let vars: Vec<Variable> = (0..self.buttons.len())
            .map(|_| problem.add(variable().min(0).integer()))
            .collect();

        let objective: Expression = vars.iter().sum();
        let mut model = problem.minimise(objective).using(default_solver);

        let mut offset = 1;
        for target_val in &self.joltages {
            let expr =
                self.buttons
                    .iter()
                    .enumerate()
                    .fold(Expression::from(0), |expr, (i, button)| {
                        if 0 < button.0 & offset {
                            expr + vars[i]
                        } else {
                            expr
                        }
                    });

            model.add_constraint(expr.eq(*target_val));
            offset <<= 1;
        }

        match model.solve() {
            Ok(solution) => solution.eval(vars.iter().sum::<Expression>()).round() as u32,
            Err(e) => panic!("Failed to solve: {e:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day10_example.txt");

    #[test]
    fn test_part1() {
        let input = Day10::parse_input(EXAMPLE_INPUT);
        let output = Day10::part_1(input);
        assert_eq!(output, 7);
    }

    #[test]
    fn test_part2() {
        let input = Day10::parse_input(EXAMPLE_INPUT);
        let output = Day10::part_2(input);
        assert_eq!(output, 33);
    }
}
