use {
    aoc_core::{iter::DynCartesianProduct, matrix::IntegerMatrix, Solution},
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

    fn contains_index(&self, i: u16) -> bool {
        self.0 & (1 << i) > 0
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
        let n = self.joltages.len();
        let m = self.buttons.len();

        // A button can be pressed at most a number of times equal to
        // the smallest required joltage it is connected to.
        let mut max_presses = vec![u32::MAX; m];

        // Set up augmented matrix
        let mut matrix = IntegerMatrix {
            rows: vec![vec![0; m + 1]; n],
        };
        for (j, button) in self.buttons.iter().enumerate() {
            for (i, joltage) in self.joltages.iter().enumerate() {
                if button.contains_index(i as u16) {
                    max_presses[j] = max_presses[j].min(*joltage);
                    matrix.rows[i][j] = 1;
                }
            }
        }
        for (i, joltage) in self.joltages.iter().enumerate() {
            matrix.rows[i][m] = (*joltage) as i64;
        }

        // Row reduce
        let (free_vars, fixed_vars) = {
            let (mut tmp, fixed_vars) = matrix.partial_row_reduce();
            // Pop off the last index because the augment column does not represent a variable.
            tmp.pop()
                .expect("Augment matrix has at least one column without a leading 1");
            (tmp, fixed_vars)
        };

        if free_vars.is_empty() {
            // System is determined!
            // So the answer is just the sum of the augment entries.
            let total: i64 = matrix.rows.into_iter().map(|r| r[m]).sum();
            return total as u32;
        }

        // Brute force search over free vars state space
        let state_space =
            DynCartesianProduct::new(free_vars.iter().map(|j| max_presses[*j] + 1).collect())
                .expect("All buttons can be pressed at least once");

        let mut min_presses = u32::MAX;
        'outer: for values in state_space {
            let mut total_presses: u32 = values.iter().copied().sum();
            for var in &fixed_vars {
                // Only accept whole number presses
                let free_values = values.iter().map(|x| *x as i64);
                let Some(presses) =
                    matrix.evaluate_augmented_row(*var, free_vars.iter().copied().zip(free_values))
                else {
                    continue 'outer;
                };
                // Only accept non-negative presses
                if presses < 0 {
                    continue 'outer;
                }
                total_presses += presses as u32;
            }
            // New smallest solution found!
            if total_presses < min_presses {
                min_presses = total_presses;
            }
        }

        min_presses
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
