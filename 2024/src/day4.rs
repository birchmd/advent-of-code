use aoc_core::{
    basic_grid,
    grid::{Grid, Position},
    iter::AtMost,
    Solution,
};

pub struct Day4;

impl Solution<'_> for Day4 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let nc = input.neighbor_context();
        let x_positions = input.index_range().filter(|x| input[*x] == b'X');
        let directions: [Box<dyn Fn(Position) -> Option<Position>>; 8] = [
            Box::new(|x| nc.up(x)),
            Box::new(|x| nc.up_left(x)),
            Box::new(|x| nc.left(x)),
            Box::new(|x| nc.down_left(x)),
            Box::new(|x| nc.down(x)),
            Box::new(|x| nc.down_right(x)),
            Box::new(|x| nc.right(x)),
            Box::new(|x| nc.up_right(x)),
        ];
        let mut total = 0;
        for x in x_positions {
            for dir in &directions {
                let letters = repeat_3(x, dir);
                if (&letters).into_iter().count() < 3 {
                    continue;
                }
                let is_xmas = letters
                    .into_iter()
                    .map(|a| input[a])
                    .zip([b'M', b'A', b'S'])
                    .all(|(a, b)| a == b);
                if is_xmas {
                    total += 1;
                }
            }
        }
        total
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let nc = input.neighbor_context();
        let a_positions = input.index_range().filter(|x| input[*x] == b'A');

        let xs = a_positions.filter_map(|a| {
            let m1 = nc.up_left(a)?;
            let s1 = nc.down_right(a)?;
            let m2 = nc.up_right(a)?;
            let s2 = nc.down_left(a)?;
            Some((
                [input[m1], input[a], input[s1]],
                [input[m2], input[a], input[s2]],
            ))
        });
        let x_mass = xs.filter(|(left, right)| {
            (left == b"MAS" || left == b"SAM") && (right == b"MAS" || right == b"SAM")
        });
        x_mass.count()
    }
}

fn repeat_3<F>(w: Position, f: F) -> AtMost<Position, 3>
where
    F: Fn(Position) -> Option<Position>,
{
    let x = f(w);
    let y = x.and_then(&f);
    let z = y.and_then(&f);
    AtMost { inner: [x, y, z] }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day4_example.txt");

    #[test]
    fn test_part1() {
        let input = Day4::parse_input(EXAMPLE_INPUT);
        let output = Day4::part_1(input);
        assert_eq!(output, 18);
    }

    #[test]
    fn test_part2() {
        let input = Day4::parse_input(EXAMPLE_INPUT);
        let output = Day4::part_2(input);
        assert_eq!(output, 9);
    }
}
