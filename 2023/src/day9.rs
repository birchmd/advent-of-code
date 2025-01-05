use aoc_core::{linked_list::LinkedList, Solution};

pub struct Day9;

impl Solution<'_> for Day9 {
    type Input = Vec<Vec<i64>>;
    type Output1 = i64;
    type Output2 = i64;

    fn parse_input(data: &str) -> Self::Input {
        data.lines()
            .map(|line| {
                line.split(' ')
                    .map(|n| n.parse().expect("List elements are numbers"))
                    .collect()
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        input.into_iter().map(extrapolate_forward).sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        input.into_iter().map(extrapolate_backward).sum()
    }
}

fn extrapolate_forward(xs: Vec<i64>) -> i64 {
    let matrix = create_matrix(xs);
    let (mut zeros, mut matrix) = matrix.pop().unwrap();
    zeros.push(0);
    let mut prev_row = zeros;
    while let LinkedList::Cons { mut head, tail } = matrix {
        let next_elem = prev_row.last().copied().unwrap() + head.last().copied().unwrap();
        head.push(next_elem);
        prev_row = head;
        matrix = *tail;
    }
    prev_row.last().copied().unwrap()
}

fn extrapolate_backward(xs: Vec<i64>) -> i64 {
    let matrix = create_matrix(xs);
    let (mut zeros, mut matrix) = matrix.pop().unwrap();
    zeros.insert(0, 0);
    let mut prev_row = zeros;
    while let LinkedList::Cons { mut head, tail } = matrix {
        let first_elem = head.first().copied().unwrap() - prev_row.first().copied().unwrap();
        head.insert(0, first_elem);
        prev_row = head;
        matrix = *tail;
    }
    prev_row.first().copied().unwrap()
}

fn create_matrix(xs: Vec<i64>) -> LinkedList<Vec<i64>> {
    let mut matrix = LinkedList::new().cons(xs);
    while matrix.head().unwrap().iter().any(|&x| x != 0) {
        let ys = first_diff(matrix.head().unwrap());
        matrix = matrix.cons(ys);
    }
    matrix
}

fn first_diff(xs: &[i64]) -> Vec<i64> {
    xs.iter()
        .zip(xs.iter().skip(1))
        .map(|(&a, &b)| b - a)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day9_example.txt");

    #[test]
    fn test_part1() {
        let input = Day9::parse_input(EXAMPLE_INPUT);
        let output = Day9::part_1(input);
        assert_eq!(output, 114);
    }

    #[test]
    fn test_part2() {
        let input = Day9::parse_input(EXAMPLE_INPUT);
        let output = Day9::part_2(input);
        assert_eq!(output, 2);
    }
}
