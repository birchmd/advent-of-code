use {
    aoc_core::{lcm, Solution},
    std::collections::HashMap,
};

pub struct Day8;

impl<'a> Solution<'a> for Day8 {
    type Input = (Vec<Instruction>, HashMap<&'a str, Node<'a>>);
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &'a str) -> Self::Input {
        let (instructions, nodes) = data
            .split_once("\n\n")
            .expect("Break between instructions and nodes");
        let instructions = instructions
            .bytes()
            .map(|b| match b {
                b'L' => Instruction::Left,
                b'R' => Instruction::Right,
                _ => panic!("Unknown instruction"),
            })
            .collect();
        let nodes = nodes.lines().map(Node::parse).collect();
        (instructions, nodes)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (instructions, nodes) = input;
        count_steps("AAA", |label| label == "ZZZ", &instructions, &nodes)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (instructions, nodes) = input;
        let n_steps = nodes.keys().filter_map(|label| {
            if !label.ends_with('A') {
                return None;
            }
            Some(count_steps(
                label,
                |l| l.ends_with('Z'),
                &instructions,
                &nodes,
            ))
        });
        n_steps.reduce(lcm).expect("At least one starting point")
    }
}

fn count_steps<F>(
    start: &str,
    end_condition: F,
    instructions: &[Instruction],
    nodes: &HashMap<&str, Node<'_>>,
) -> u64
where
    F: Fn(&'_ str) -> bool,
{
    let mut iter = instructions.iter().cycle();
    let mut current_node = start;
    let mut count = 0;
    while !end_condition(current_node) {
        let instruction = iter.next().unwrap();
        current_node = match instruction {
            Instruction::Left => nodes[current_node].left,
            Instruction::Right => nodes[current_node].right,
        };
        count += 1;
    }
    count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn parse(data: &'a str) -> (&'a str, Node<'a>) {
        let (label, remainder) = data.split_once('=').expect("Contains equals");
        let (left, right) = remainder.trim().split_once(',').expect("Contains comma");
        let left = left.trim();
        let right = right.trim();
        (
            label.trim(),
            Self {
                left: &left[1..],
                right: &right[0..(right.len() - 1)],
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_PART1: &str = include_str!("res/day8_example_part1.txt");
    const EXAMPLE_INPUT_PART2: &str = include_str!("res/day8_example_part2.txt");

    #[test]
    fn test_part1() {
        let input = Day8::parse_input(EXAMPLE_INPUT_PART1);
        let output = Day8::part_1(input);
        assert_eq!(output, 6);
    }

    #[test]
    fn test_part2() {
        let input = Day8::parse_input(EXAMPLE_INPUT_PART2);
        let output = Day8::part_2(input);
        assert_eq!(output, 6);
    }
}
