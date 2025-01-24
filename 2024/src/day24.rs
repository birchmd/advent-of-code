use {
    aoc_core::Solution,
    std::collections::{HashMap, VecDeque},
};

pub struct Day24;

impl<'a> Solution<'a> for Day24 {
    type Input = (HashMap<&'a str, u8>, VecDeque<Gate<'a>>);
    type Output1 = u64;
    type Output2 = String;

    fn parse_input(data: &'a str) -> Self::Input {
        let (inputs_block, gates_block) = data.split_once("\n\n").expect("Blank line");

        let mut inputs = HashMap::new();
        for line in inputs_block.lines() {
            let (label, value) = line.split_once(':').expect("Has colon");
            inputs.insert(label, value.trim().parse().unwrap());
        }

        let mut gates = VecDeque::new();
        for line in gates_block.lines() {
            let words: Vec<&'a str> = line.split(' ').collect();
            gates.push_back(Gate {
                in1: words[0],
                in2: words[2],
                op: Op::parse(words[1]),
                out: words[4],
            });
        }

        (inputs, gates)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (mut wires, mut gates) = input;
        simulate(&mut wires, &mut gates);
        get_number_from_bits("z", &wires)
    }

    // Need to match up the wrong_outputs with wrong_inner
    fn part_2(input: Self::Input) -> Self::Output2 {
        // The circuit is a 45-bit Ripple-carry adder.
        // This has a specific structure. We can look for gates
        // that fail to meet this structure.

        let (mut wires, mut gates) = input;

        // All output gates (except the last one) should be XOR
        let wrong_outputs: Vec<usize> = gates
            .iter()
            .enumerate()
            .filter_map(|(index, gate)| {
                if gate.out.starts_with('z') && gate.out != "z45" && gate.op != Op::XOr {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        // Inner gates should only use AND/OR
        let wrong_inner: Vec<usize> = gates
            .iter()
            .enumerate()
            .filter_map(|(index, gate)| {
                if is_inner_gate(gate) && gate.op == Op::XOr {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        // Need to match up the wrong_outputs with wrong_inner
        for &inner in &wrong_inner {
            let goal = first_z_using(gates[inner].out, &gates).unwrap();
            let output = wrong_outputs
                .iter()
                .find(|index| gates[**index].out == goal)
                .copied()
                .unwrap();
            let tmp = gates[inner].out;
            gates[inner].out = gates[output].out;
            gates[output].out = tmp;
        }

        // With those swaps done, now we need to find the carry gates that are wrong.
        // We compare the right answer to the simulation and find where the carry was incorrect.
        let x = get_number_from_bits("x", &wires);
        let y = get_number_from_bits("y", &wires);
        simulate(&mut wires, &mut gates.clone());
        let run = get_number_from_bits("z", &wires);
        let false_carry = (x + y) ^ run;
        let trailing_zeros = format!("{:02}", false_carry.trailing_zeros());
        let trailing_gates = gates.iter().filter(|gate| {
            gate.in1.ends_with(&trailing_zeros) && gate.in2.ends_with(&trailing_zeros)
        });

        // Finally, print all the wires we changed!
        let mut all_changed_gates: Vec<&'a str> = trailing_gates
            .chain(wrong_outputs.into_iter().map(|index| &gates[index]))
            .chain(wrong_inner.into_iter().map(|index| &gates[index]))
            .map(|gate| gate.out)
            .collect();
        all_changed_gates.sort();
        all_changed_gates.join(",")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gate<'a> {
    in1: &'a str,
    in2: &'a str,
    op: Op,
    out: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    And,
    XOr,
    Or,
}

impl Op {
    pub fn parse(word: &str) -> Self {
        match word {
            "AND" => Self::And,
            "XOR" => Self::XOr,
            "OR" => Self::Or,
            _ => panic!("Unexpected op"),
        }
    }
}

fn is_ready<'a>(wires: &HashMap<&'a str, u8>, gate: &Gate<'a>) -> bool {
    wires.contains_key(gate.in1) && wires.contains_key(gate.in2)
}

fn get_number_from_bits(prefix: &str, wires: &HashMap<&str, u8>) -> u64 {
    let mut output = 0;
    let mut power = 1;
    for n in 0..46 {
        let wire = [prefix, &format!("{n:02}")].concat();
        let Some(bit) = wires.get(wire.as_str()) else {
            break;
        };
        output += (*bit as u64) * power;
        power *= 2;
    }
    output
}

fn simulate<'a>(wires: &mut HashMap<&'a str, u8>, gates: &mut VecDeque<Gate<'a>>) {
    while let Some(gate) = gates.pop_front() {
        if !is_ready(wires, &gate) {
            gates.push_back(gate);
            continue;
        }

        let input1 = wires[gate.in1];
        let input2 = wires[gate.in2];
        let output = match gate.op {
            Op::And => input1 & input2,
            Op::XOr => input1 ^ input2,
            Op::Or => input1 | input2,
        };
        wires.insert(gate.out, output);
    }
}

fn is_inner_gate(gate: &Gate<'_>) -> bool {
    fn starts_with_x_or_y(s: &str) -> bool {
        s.starts_with('x') || s.starts_with('y')
    }
    !starts_with_x_or_y(gate.in1) && !starts_with_x_or_y(gate.in2) && !gate.out.starts_with('z')
}

fn first_z_using<'a>(wire: &'a str, gates: &VecDeque<Gate<'a>>) -> Option<String> {
    let users: Vec<&Gate<'a>> = gates
        .iter()
        .filter(|g| g.in1 == wire || g.in2 == wire)
        .collect();
    let maybe_z_user = users.iter().find(|g| g.out.starts_with('z'));
    if let Some(z_user) = maybe_z_user {
        let number = z_user.out[1..].parse::<usize>().unwrap() - 1;
        Some(format!("z{number:02}"))
    } else {
        for gate in users {
            let result = first_z_using(gate.out, gates);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day24_example.txt");

    #[test]
    fn test_part1() {
        let input = Day24::parse_input(EXAMPLE_INPUT);
        let output = Day24::part_1(input);
        assert_eq!(output, 2024);
    }
}
