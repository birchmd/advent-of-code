use aoc_core::{
    construct_base_10, create_grid, digit_value,
    grid::{Grid, Position},
    linked_list::LinkedList,
    Solution,
};

pub struct Day3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Digit(u8),
    Symbol(u8),
    Empty,
}

impl Element {
    fn from_byte(b: u8) -> Self {
        if b == b'.' {
            return Self::Empty;
        }
        if let Some(d) = digit_value(b) {
            return Self::Digit(d);
        }
        Self::Symbol(b)
    }

    fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }
}

struct PartNumber {
    value: u64,
    cells: LinkedList<Position>,
}

fn find_part_numbers(input: &Grid<Element>) -> Vec<PartNumber> {
    let nc = input.neighbor_context();
    let mut result = Vec::new();
    let mut add_part_number = |digit_acc: LinkedList<u8>, points_acc: LinkedList<Position>| {
        let has_symbol = points_acc
            .iter()
            .copied()
            .flat_map(|x| nc.all_neighbors_of(x).map(|y| input[y]))
            .any(|el| el.is_symbol());
        if !digit_acc.is_empty() && has_symbol {
            let number = construct_base_10(digit_acc);
            result.push(PartNumber {
                value: number,
                cells: points_acc,
            });
        }
    };
    for (i, row) in input.rows.iter().enumerate() {
        let (digit_acc, points_acc) = row.iter().enumerate().fold(
            (LinkedList::Nil, LinkedList::Nil),
            |(digit_acc, points_acc), (j, element)| {
                if let Element::Digit(d) = element {
                    (digit_acc.cons(*d), points_acc.cons((i, j)))
                } else {
                    add_part_number(digit_acc, points_acc);
                    (LinkedList::Nil, LinkedList::Nil)
                }
            },
        );
        add_part_number(digit_acc, points_acc);
    }
    result
}

impl Solution<'_> for Day3 {
    type Input = Grid<Element>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &str) -> Self::Input {
        create_grid(data, Element::from_byte)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let part_numbers = find_part_numbers(&input);
        part_numbers.into_iter().map(|pn| pn.value).sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let part_numbers = find_part_numbers(&input);
        let nc = input.neighbor_context();
        let mut total = 0;
        for x in input.index_range() {
            let element = input[x];
            if element != Element::Symbol(b'*') {
                continue;
            }
            let mut adjacent = part_numbers.iter().filter(|pn| {
                pn.cells
                    .iter()
                    .any(|y| nc.all_neighbors_of(*y).any(|q| q == x))
            });
            let Some(one) = adjacent.next() else {
                continue;
            };
            let Some(two) = adjacent.next() else {
                continue;
            };
            if adjacent.next().is_none() {
                total += one.value * two.value;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day3_example.txt");

    #[test]
    fn test_part1() {
        let input = Day3::parse_input(EXAMPLE_INPUT);
        let output = Day3::part_1(input);
        assert_eq!(output, 4361);
    }

    #[test]
    fn test_part2() {
        let input = Day3::parse_input(EXAMPLE_INPUT);
        let output = Day3::part_2(input);
        assert_eq!(output, 467835);
    }
}
