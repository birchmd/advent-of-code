use {aoc_core::Solution, std::collections::HashMap};

pub struct Day2;

const COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

impl Color {
    fn parse_counts(sample: &str) -> HashMap<Self, usize> {
        let mut result = HashMap::new();

        for part in sample.split(',').map(str::trim) {
            let (number, color) = part
                .split_once(' ')
                .expect("space between number and color");
            let number = number.parse().unwrap();
            match color {
                "red" => result.insert(Color::Red, number),
                "green" => result.insert(Color::Green, number),
                "blue" => result.insert(Color::Blue, number),
                other => panic!("Unknown color {other}"),
            };
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    id: u64,
    samples: Vec<HashMap<Color, usize>>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let (id, samples) = line.split_once(':').expect("Must contain colon");
        let id = id
            .split(' ')
            .next_back()
            .unwrap()
            .parse()
            .expect("ID is a number");
        let samples = samples.split(';').map(Color::parse_counts).collect();
        Self { id, samples }
    }
}

impl Solution<'_> for Day2 {
    type Input = Vec<Game>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &str) -> Self::Input {
        data.lines().map(Game::parse).collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        const MAX_COLORS: [usize; 3] = [12, 13, 14];

        let mut total = 0;
        for game in input {
            let mut samples = COLORS.iter().flat_map(|c| {
                game.samples
                    .iter()
                    .map(|s| (*c as u8, s.get(c).copied().unwrap_or_default()))
            });

            let possible = samples.all(|(c, number)| number <= MAX_COLORS[c as usize]);
            if possible {
                total += game.id;
            }
        }
        total
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let min_cubes = |c: &Color, game: &Game| {
            let min = game
                .samples
                .iter()
                .map(|sample| sample.get(c).copied().unwrap_or_default())
                .max()
                .expect("There is at least one sample");
            min as u64
        };

        let mut total = 0;
        for game in input {
            let min_red = min_cubes(&Color::Red, &game);
            let min_green = min_cubes(&Color::Green, &game);
            let min_blue = min_cubes(&Color::Blue, &game);
            total += min_red * min_green * min_blue;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day2_example.txt");

    #[test]
    fn test_part1() {
        let input = Day2::parse_input(EXAMPLE_INPUT);
        let output = Day2::part_1(input);
        assert_eq!(output, 8);
    }

    #[test]
    fn test_part2() {
        let input = Day2::parse_input(EXAMPLE_INPUT);
        let output = Day2::part_2(input);
        assert_eq!(output, 2286);
    }
}
