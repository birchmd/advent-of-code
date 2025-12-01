use {
    aoc_core::{strip_label, Solution},
    std::{collections::HashMap, ops::Range},
};

pub struct Day19;

impl<'a> Solution<'a> for Day19 {
    type Input = (HashMap<&'a str, Rule<'a>>, Vec<Part>);
    type Output1 = u64;
    type Output2 = usize;

    fn parse_input(data: &'a str) -> Self::Input {
        let (rules, parts) = data.split_once("\n\n").expect("Blocks");

        let rules = rules
            .lines()
            .map(|line| {
                let rule = Rule::from_str(line);
                (rule.name, rule)
            })
            .collect();
        let parts = parts.lines().map(Part::from_str).collect();

        (rules, parts)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (rules, parts) = input;
        parts
            .into_iter()
            .filter(|p| matches!(evaluate_part(p, &rules), Evaluation::Accepted))
            .map(|p| p.x + p.m + p.a + p.s)
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (rules, _) = input;
        evaluate_range(&rules)
    }
}

fn evaluate_part<'a>(part: &Part, rules: &HashMap<&'a str, Rule<'a>>) -> Evaluation {
    let mut label = "in";
    'outer: while label != "A" && label != "R" {
        let rule = &rules[label];
        for (condition, target) in &rule.conditions {
            if part.matches(condition) {
                label = target;
                continue 'outer;
            }
        }
        label = rule.fallback;
    }
    if label == "A" {
        Evaluation::Accepted
    } else {
        Evaluation::Rejected
    }
}

fn evaluate_range<'a>(rules: &HashMap<&'a str, Rule<'a>>) -> usize {
    let mut accepted = Vec::new();
    let mut stack = vec![("in", PartRange::default())];

    while let Some((label, range)) = stack.pop() {
        let rule = &rules[label];
        let mut fall_through = range;
        for (condition, target) in &rule.conditions {
            let target = *target;
            let split = fall_through.split(condition);
            let new_range = split.0;
            fall_through = split.1;
            if target == "A" {
                accepted.push(new_range);
            } else if target != "R" {
                stack.push((target, new_range));
            }
        }
        let target = rule.fallback;
        if target == "A" {
            accepted.push(fall_through);
        } else if target != "R" {
            stack.push((target, fall_through));
        }
    }

    accepted
        .into_iter()
        .map(|r| r.x.len() * r.m.len() * r.a.len() * r.s.len())
        .sum::<usize>()
}

enum Evaluation {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule<'a> {
    name: &'a str,
    conditions: Vec<(Condition, &'a str)>,
    fallback: &'a str,
}

impl<'a> Rule<'a> {
    fn from_str(data: &'a str) -> Self {
        let (name, remainder) = data.split_once('{').unwrap();
        let remainder = remainder.strip_suffix('}').unwrap();
        let conditions = remainder
            .split(',')
            .filter_map(|rule| {
                let (condition, target) = rule.split_once(':')?;
                let mut iter = condition.bytes();
                let quantity = Quantity::from_u8(iter.next().unwrap());
                let operator = Operator::from_u8(iter.next().unwrap());
                let value = condition[2..].parse().unwrap();
                let condition = Condition {
                    quantity,
                    operator,
                    value,
                };
                Some((condition, target))
            })
            .collect();
        let fallback = remainder.split(',').next_back().unwrap();
        Self {
            name,
            conditions,
            fallback,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Condition {
    quantity: Quantity,
    operator: Operator,
    value: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantity {
    X,
    M,
    A,
    S,
}

impl Quantity {
    fn from_u8(b: u8) -> Self {
        match b {
            b'x' => Self::X,
            b'm' => Self::M,
            b'a' => Self::A,
            b's' => Self::S,
            _ => panic!("Unknown quantity"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Lt,
    Gt,
}

impl Operator {
    fn from_u8(b: u8) -> Self {
        match b {
            b'<' => Self::Lt,
            b'>' => Self::Gt,
            _ => panic!("Unknown operator"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRange {
    fn split(&self, condition: &Condition) -> (Self, Self) {
        use {Operator::*, Quantity::*};
        let value = match condition.operator {
            Lt => condition.value as usize,
            Gt => (condition.value + 1) as usize,
        };
        let (lt, gt) = match condition.quantity {
            X => (
                Self {
                    x: (self.x.start)..(value),
                    ..self.clone()
                },
                Self {
                    x: (value)..(self.x.end),
                    ..self.clone()
                },
            ),
            M => (
                Self {
                    m: (self.m.start)..(value),
                    ..self.clone()
                },
                Self {
                    m: (value)..(self.m.end),
                    ..self.clone()
                },
            ),
            A => (
                Self {
                    a: (self.a.start)..(value),
                    ..self.clone()
                },
                Self {
                    a: (value)..(self.a.end),
                    ..self.clone()
                },
            ),
            S => (
                Self {
                    s: (self.s.start)..(value),
                    ..self.clone()
                },
                Self {
                    s: (value)..(self.s.end),
                    ..self.clone()
                },
            ),
        };
        match condition.operator {
            Lt => (lt, gt),
            Gt => (gt, lt),
        }
    }
}

impl Default for PartRange {
    fn default() -> Self {
        Self {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn from_str(data: &str) -> Self {
        let n = data.len();
        let remainder = &data[1..(n - 1)];
        let (x, remainder) = remainder.split_once(',').unwrap();
        let (m, remainder) = remainder.split_once(',').unwrap();
        let (a, s) = remainder.split_once(',').unwrap();
        Self {
            x: strip_label(x, '=').parse().unwrap(),
            m: strip_label(m, '=').parse().unwrap(),
            a: strip_label(a, '=').parse().unwrap(),
            s: strip_label(s, '=').parse().unwrap(),
        }
    }

    fn matches(&self, condition: &Condition) -> bool {
        use {Operator::*, Quantity::*};
        match (condition.quantity, condition.operator) {
            (X, Lt) => self.x < condition.value,
            (M, Lt) => self.m < condition.value,
            (A, Lt) => self.a < condition.value,
            (S, Lt) => self.s < condition.value,
            (X, Gt) => self.x > condition.value,
            (M, Gt) => self.m > condition.value,
            (A, Gt) => self.a > condition.value,
            (S, Gt) => self.s > condition.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day19_example.txt");

    #[test]
    fn test_part1() {
        let input = Day19::parse_input(EXAMPLE_INPUT);
        let output = Day19::part_1(input);
        assert_eq!(output, 19_114);
    }

    #[test]
    fn test_part2() {
        let input = Day19::parse_input(EXAMPLE_INPUT);
        let output = Day19::part_2(input);
        assert_eq!(output, 167_409_079_868_000);
    }
}
