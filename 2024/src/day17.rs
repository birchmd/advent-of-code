use aoc_core::{iter::AtMost, strip_label, Solution};

pub struct Day17;

impl Solution<'_> for Day17 {
    type Input = (State, Vec<u8>);
    type Output1 = String;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        let (state, program) = data.split_once("\n\n").expect("Blank line");
        let state = AtMost::<usize, 3>::some(
            state
                .lines()
                .map(|l| strip_label(l, ':').parse().expect("Registers are numbers")),
        );
        let program = strip_label(program, ':')
            .split(',')
            .map(|x| x.parse().expect("Opcodes are numbers"))
            .collect();
        (State::new(state), program)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (initial_state, program) = input;
        let output = execute(&program, initial_state);
        let mut result = String::with_capacity(2 * output.len());
        let chrs = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        result.push(chrs[output[0]]);
        for x in &output[1..] {
            result.push(',');
            result.push(chrs[*x]);
        }
        result
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (mut state, program) = input;
        quine_search(&mut state, &program, 0, program.len() - 1).expect("Has solution")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    a: usize,
    b: usize,
    c: usize,
}

impl State {
    fn new(registers: AtMost<usize, 3>) -> Self {
        Self {
            a: registers.inner[0].unwrap(),
            b: registers.inner[1].unwrap(),
            c: registers.inner[2].unwrap(),
        }
    }
}

fn combo_operand(operand: u8, state: &State) -> usize {
    match operand {
        x if x <= 3 => operand as usize,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!("Invalid combo operand"),
    }
}

fn div(operand: u8, state: &State) -> usize {
    let numerator = state.a;
    let exponent = combo_operand(operand, state);
    numerator >> exponent
}

fn execute(program: &[u8], initial_state: State) -> Vec<usize> {
    let mut output = Vec::new();
    let mut instruction_pointer = 0;
    let mut state = initial_state;
    while instruction_pointer < program.len() {
        let opcode = program[instruction_pointer];
        let operand = program[instruction_pointer + 1];
        match opcode {
            0 => {
                state.a = div(operand, &state);
                instruction_pointer += 2;
            }
            1 => {
                state.b ^= operand as usize;
                instruction_pointer += 2;
            }
            2 => {
                state.b = combo_operand(operand, &state) % 8;
                instruction_pointer += 2;
            }
            3 => {
                if state.a != 0 {
                    instruction_pointer = operand as usize;
                } else {
                    instruction_pointer += 2;
                }
            }
            4 => {
                state.b ^= state.c;
                instruction_pointer += 2;
            }
            5 => {
                output.push(combo_operand(operand, &state) % 8);
                instruction_pointer += 2;
            }
            6 => {
                state.b = div(operand, &state);
                instruction_pointer += 2;
            }
            7 => {
                state.c = div(operand, &state);
                instruction_pointer += 2;
            }
            _ => panic!("Invalid opcode"),
        }
    }
    output
}

fn quine_search(state: &mut State, program: &[u8], init_a: usize, index: usize) -> Option<usize> {
    let goal = program[index] as usize;
    let mut possible_next_bits = Vec::new();
    for i in 0..8 {
        state.a = init_a * 8 + i;
        let output = execute(program, state.clone());
        if output[0] == goal {
            possible_next_bits.push(i);
        }
    }
    if index == 0 {
        return possible_next_bits.first().map(|&i| init_a * 8 + i);
    }
    for next_bits in possible_next_bits {
        let x = quine_search(state, program, init_a * 8 + next_bits, index - 1);
        if x.is_some() {
            return x;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day17_example.txt");
    const EXAMPLE_INPUT2: &str = include_str!("res/day17_example_part2.txt");

    #[test]
    fn test_part1() {
        let input = Day17::parse_input(EXAMPLE_INPUT);
        let output = Day17::part_1(input);
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let input = Day17::parse_input(EXAMPLE_INPUT2);
        let output = Day17::part_2(input);
        assert_eq!(output, 117440);
    }
}
