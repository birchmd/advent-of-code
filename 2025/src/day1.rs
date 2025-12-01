use aoc_core::Solution;

pub struct Day1;

const INITIAL_POSITION: i32 = 50;

impl Solution<'_> for Day1 {
    type Input = Vec<i32>;
    type Output1 = u32;
    type Output2 = u32;

    fn parse_input(data: &'_ str) -> Self::Input {
        data.lines()
            .filter_map(|l| {
                let l = l.trim();
                let sign = match l.bytes().next()? {
                    b'L' => -1,
                    b'R' => 1,
                    _ => panic!("Unexpected prefix"),
                };
                let number: i32 = l[1..].parse().expect("After prefix should be a number");
                Some(sign * number)
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (n_zeros, _) =
            input
                .into_iter()
                .fold((0, INITIAL_POSITION), |(n_zeros, position), rotation| {
                    let new_position = (position + rotation) % 100;
                    if new_position == 0 {
                        (n_zeros + 1, new_position)
                    } else {
                        (n_zeros, new_position)
                    }
                });
        n_zeros
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (n_zeros, _) =
            input
                .into_iter()
                .fold((0, INITIAL_POSITION), |(n_zeros, position), rotation| {
                    // Each full rotation must hit zero once.
                    let full_rotations = (rotation.abs() / 100) as u32;
                    let new_position = position + (rotation % 100);

                    // If the position starts at 0 we cannot hit zero again with
                    // less than one full rotation.
                    // If the starting position was not 0 and we ended out of the normalized range
                    // then we must have hit zero during the rotation.
                    let hit_zero = if position != 0 && (new_position <= 0 || 100 <= new_position) {
                        1
                    } else {
                        0
                    };

                    // Normalize the position to be in the range [0, 100).
                    let new_position = (new_position + 100) % 100;
                    (n_zeros + hit_zero + full_rotations, new_position)
                });
        n_zeros
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day1_example.txt");

    #[test]
    fn test_part1() {
        let input = Day1::parse_input(EXAMPLE_INPUT);
        let output = Day1::part_1(input);
        assert_eq!(output, 3);
    }

    #[test]
    fn test_part2() {
        let input = Day1::parse_input(EXAMPLE_INPUT);
        let output = Day1::part_2(input);
        assert_eq!(output, 6);
    }
}
