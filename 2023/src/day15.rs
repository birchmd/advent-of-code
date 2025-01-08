use aoc_core::Solution;

pub struct Day15;

impl<'a> Solution<'a> for Day15 {
    type Input = &'a str;
    type Output1 = u64;
    type Output2 = usize;

    fn parse_input(data: &'a str) -> Self::Input {
        data
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        input.replace('\n', "").split(',').map(hash_algorithm).sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let input = input.replace('\n', "");
        let mut state: Vec<Vec<Lens>> = Vec::with_capacity(256);
        for _ in 0..256 {
            state.push(Vec::new());
        }

        let commands = input.split(',').map(Command::parse);
        for command in commands {
            let label = command.label();
            let box_index = hash_algorithm(label) as usize;
            let box_state = &mut state[box_index];
            let lens_position = box_state.iter().position(|l| l.label == label);
            match (command, lens_position) {
                (Command::Insert(lens), Some(position)) => {
                    box_state[position].focal_length = lens.focal_length;
                }
                (Command::Insert(lens), None) => {
                    box_state.push(lens);
                }
                (Command::Remove(_), Some(position)) => {
                    box_state.remove(position);
                }
                (Command::Remove(_), None) => (),
            }
        }

        let mut focusing_power = 0;
        for (box_index, box_state) in state.iter().enumerate() {
            for (lens_index, lens) in box_state.iter().enumerate() {
                focusing_power += (box_index + 1) * (lens_index + 1) * lens.focal_length;
            }
        }
        focusing_power
    }
}

fn hash_algorithm(data: &str) -> u64 {
    let mut current_value = 0;
    for b in data.bytes() {
        current_value += b as u64;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

enum Command<'a> {
    Insert(Lens<'a>),
    Remove(&'a str),
}

impl<'a> Command<'a> {
    fn parse(data: &'a str) -> Self {
        if let Some((label, focal_length)) = data.split_once('=') {
            return Self::Insert(Lens {
                label,
                focal_length: focal_length.parse().expect("Focal length is a number"),
            });
        }

        let Some((label, _)) = data.split_once('-') else {
            panic!("Command without = or -");
        };
        Self::Remove(label)
    }

    fn label(&self) -> &'a str {
        match self {
            Self::Insert(lens) => lens.label,
            Self::Remove(label) => label,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day15_example.txt");

    #[test]
    fn test_part1() {
        let input = Day15::parse_input(EXAMPLE_INPUT);
        let output = Day15::part_1(input);
        assert_eq!(output, 1320);
    }

    #[test]
    fn test_part2() {
        let input = Day15::parse_input(EXAMPLE_INPUT);
        let output = Day15::part_2(input);
        assert_eq!(output, 145);
    }
}
