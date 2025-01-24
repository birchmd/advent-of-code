use {
    aoc_core::Solution,
    std::{collections::HashMap, sync::LazyLock},
};

pub struct Day21;

impl<'a> Solution<'a> for Day21 {
    type Input = Vec<&'a str>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &'a str) -> Self::Input {
        data.lines().collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        compute_complexity(&input, 2)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        compute_complexity(&input, 25)
    }
}

fn shortest_path(code: &str) -> String {
    let mut output = String::new();
    let mut current = b'A';
    for dest in code.bytes() {
        output.push_str(ALL_PATHS[&(current, dest)]);
        output.push('A');
        current = dest;
    }
    output
}

// Idea: split the code sequence at 'A' since we always start from
// 'A' in the first place. This allows us to consider identical sequences
// just once instead of repeatedly.
// Note that we must skip the last section because nothing comes after it
// (and therefore it doesn't contribute any moves).
fn accumulate_sections(code: &str, counts: &mut HashMap<String, u64>, step: u64) {
    let parts: Vec<&str> = code.split('A').collect();
    let n = parts.len();
    for part in &parts[0..(n - 1)] {
        let key = [*part, "A"].concat();
        *counts.entry(key).or_default() += step;
    }
}

fn iterated_shortest_path(code: &str, depth: usize) -> u64 {
    let code = shortest_path(code);
    let mut counts = HashMap::new();
    let mut iter_counts = HashMap::new();
    accumulate_sections(&code, &mut counts, 1);
    for _ in 0..depth {
        for (part, count) in counts.drain() {
            let code = shortest_path(&part);
            accumulate_sections(&code, &mut iter_counts, count);
        }
        core::mem::swap(&mut counts, &mut iter_counts);
    }
    counts
        .into_iter()
        .map(|(part, count)| count * (part.len() as u64))
        .sum()
}

fn compute_complexity(codes: &[&str], depth: usize) -> u64 {
    codes
        .iter()
        .map(|code| {
            let path_length = iterated_shortest_path(code, depth);
            let n = code.len();
            let x: u64 = code[0..(n - 1)].parse().unwrap();
            path_length * x
        })
        .sum()
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
const NUMERIC_PATHS: &[((u8, u8), &[&str])] = &[
    ((b'A', b'A'), &[""]),
    ((b'A', b'0'), &["<"]),
    ((b'A', b'1'), &["^<<"]),
    ((b'A', b'2'), &["<^", "^<"]),
    ((b'A', b'3'), &["^"]),
    ((b'A', b'4'), &["^^<<"]),
    ((b'A', b'5'), &["<^^", "^^<"]),
    ((b'A', b'6'), &["^^"]),
    ((b'A', b'7'), &["^^^<<"]),
    ((b'A', b'8'), &["<^^^", "^^^<"]),
    ((b'A', b'9'), &["^^^"]),
    ((b'0', b'A'), &[">"]),
    ((b'0', b'0'), &[""]),
    ((b'0', b'1'), &["^<"]),
    ((b'0', b'2'), &["^"]),
    ((b'0', b'3'), &["^>", ">^"]),
    ((b'0', b'4'), &["^^<"]),
    ((b'0', b'5'), &["^^"]),
    ((b'0', b'6'), &["^^>", ">^^"]),
    ((b'0', b'7'), &["^^^<"]),
    ((b'0', b'8'), &["^^^"]),
    ((b'0', b'9'), &["^^^>", ">^^^"]),
    ((b'1', b'A'), &[">>v"]),
    ((b'1', b'0'), &[">v"]),
    ((b'1', b'1'), &[""]),
    ((b'1', b'2'), &[">"]),
    ((b'1', b'3'), &[">>"]),
    ((b'1', b'4'), &["^"]),
    ((b'1', b'5'), &["^>", ">^"]),
    ((b'1', b'6'), &["^>>", ">>^"]),
    ((b'1', b'7'), &["^^"]),
    ((b'1', b'8'), &["^^>", ">^^"]),
    ((b'1', b'9'), &["^^>>", ">>^^"]),
    ((b'2', b'A'), &["v>", ">v"]),
    ((b'2', b'0'), &["v"]),
    ((b'2', b'1'), &["<"]),
    ((b'2', b'2'), &[""]),
    ((b'2', b'3'), &[">"]),
    ((b'2', b'4'), &["<^", "^<"]),
    ((b'2', b'5'), &["^"]),
    ((b'2', b'6'), &["^>", ">^"]),
    ((b'2', b'7'), &["<^^", "^^<"]),
    ((b'2', b'8'), &["^^"]),
    ((b'2', b'9'), &["^^>", ">^^"]),
    ((b'3', b'A'), &["v"]),
    ((b'3', b'0'), &["<v", "v<"]),
    ((b'3', b'1'), &["<<"]),
    ((b'3', b'2'), &["<"]),
    ((b'3', b'3'), &[""]),
    ((b'3', b'4'), &["<<^", "^<<"]),
    ((b'3', b'5'), &["<^", "^<"]),
    ((b'3', b'6'), &["^"]),
    ((b'3', b'7'), &["<<^^", "^^<<"]),
    ((b'3', b'8'), &["<^^", "^^<"]),
    ((b'3', b'9'), &["^^"]),
    ((b'4', b'A'), &[">>vv"]),
    ((b'4', b'0'), &[">vv"]),
    ((b'4', b'1'), &["v"]),
    ((b'4', b'2'), &["v>", ">v"]),
    ((b'4', b'3'), &["v>>", ">>v"]),
    ((b'4', b'4'), &[""]),
    ((b'4', b'5'), &[">"]),
    ((b'4', b'6'), &[">>"]),
    ((b'4', b'7'), &["^"]),
    ((b'4', b'8'), &["^>", ">^"]),
    ((b'4', b'9'), &["^>>", ">>^"]),
    ((b'5', b'A'), &["vv>", ">vv"]),
    ((b'5', b'0'), &["vv"]),
    ((b'5', b'1'), &["<v", "v<"]),
    ((b'5', b'2'), &["v"]),
    ((b'5', b'3'), &["v>", ">v"]),
    ((b'5', b'4'), &["<"]),
    ((b'5', b'5'), &[""]),
    ((b'5', b'6'), &[">"]),
    ((b'5', b'7'), &["<^", "^<"]),
    ((b'5', b'8'), &["^"]),
    ((b'5', b'9'), &["^>", ">^"]),
    ((b'6', b'A'), &["vv"]),
    ((b'6', b'0'), &["<vv", "vv<"]),
    ((b'6', b'1'), &["v<<", "<<v"]),
    ((b'6', b'2'), &["<v", "v<"]),
    ((b'6', b'3'), &["v"]),
    ((b'6', b'4'), &["<<"]),
    ((b'6', b'5'), &["<"]),
    ((b'6', b'6'), &[""]),
    ((b'6', b'7'), &["<<^", "^<<"]),
    ((b'6', b'8'), &["<^", "^<"]),
    ((b'6', b'9'), &["^"]),
    ((b'7', b'A'), &[">>vvv"]),
    ((b'7', b'0'), &[">vvv"]),
    ((b'7', b'1'), &["vv"]),
    ((b'7', b'2'), &["vv>", ">vv"]),
    ((b'7', b'3'), &["vv>>", ">>vv"]),
    ((b'7', b'4'), &["v"]),
    ((b'7', b'5'), &["v>", ">v"]),
    ((b'7', b'6'), &["v>>", ">>v"]),
    ((b'7', b'7'), &[""]),
    ((b'7', b'8'), &[">"]),
    ((b'7', b'9'), &[">>"]),
    ((b'8', b'A'), &["vvv>", ">vvv"]),
    ((b'8', b'0'), &["vvv"]),
    ((b'8', b'1'), &["<vv", "vv<"]),
    ((b'8', b'2'), &["vv"]),
    ((b'8', b'3'), &["vv>", ">vv"]),
    ((b'8', b'4'), &["<v", "v<"]),
    ((b'8', b'5'), &["v"]),
    ((b'8', b'6'), &["v>", ">v"]),
    ((b'8', b'7'), &["<"]),
    ((b'8', b'8'), &[""]),
    ((b'8', b'9'), &[">"]),
    ((b'9', b'A'), &["vvv"]),
    ((b'9', b'0'), &["<vvv", "vvv<"]),
    ((b'9', b'1'), &["<<vv", "vv<<"]),
    ((b'9', b'2'), &["<vv", "vv<"]),
    ((b'9', b'3'), &["vv"]),
    ((b'9', b'4'), &["<<v", "v<<"]),
    ((b'9', b'5'), &["<v", "v<"]),
    ((b'9', b'6'), &["v"]),
    ((b'9', b'7'), &["<<"]),
    ((b'9', b'8'), &["<"]),
    ((b'9', b'9'), &[""]),
];

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
const DIRECTIONAL_PATHS: &[((u8, u8), &[&str])] = &[
    ((b'A', b'A'), &[""]),
    ((b'A', b'^'), &["<"]),
    ((b'A', b'<'), &["v<<"]),
    ((b'A', b'v'), &["<v", "v<"]),
    ((b'A', b'>'), &["v"]),
    ((b'^', b'A'), &[">"]),
    ((b'^', b'^'), &[""]),
    ((b'^', b'<'), &["v<"]),
    ((b'^', b'v'), &["v"]),
    ((b'^', b'>'), &["v>", ">v"]),
    ((b'<', b'A'), &[">>^"]),
    ((b'<', b'^'), &[">^"]),
    ((b'<', b'<'), &[""]),
    ((b'<', b'v'), &[">"]),
    ((b'<', b'>'), &[">>"]),
    ((b'v', b'A'), &["^>", ">^"]),
    ((b'v', b'^'), &["^"]),
    ((b'v', b'<'), &["<"]),
    ((b'v', b'v'), &[""]),
    ((b'v', b'>'), &[">"]),
    ((b'>', b'A'), &["^"]),
    ((b'>', b'^'), &["<^", "^<"]),
    ((b'>', b'<'), &["<<"]),
    ((b'>', b'v'), &["<"]),
    ((b'>', b'>'), &[""]),
];

// Note: in the paths defined above, when there are multiple
// possibilities the first option is chosen with the preference
// < before ^ before v before >. Using this preference,
// we always obtain the optimal solution and therefore we ignore
// the second option when constructing `ALL_PATHS`.
// This preferential order was discovered empirically, I'm not sure
// what the theoretical justification for it is.
static ALL_PATHS: LazyLock<HashMap<(u8, u8), &str>> = LazyLock::new(|| {
    let mut all_paths = HashMap::new();

    for (k, v) in NUMERIC_PATHS {
        all_paths.insert(*k, v[0]);
    }
    for (k, v) in DIRECTIONAL_PATHS {
        all_paths.insert(*k, v[0]);
    }

    all_paths
});

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day21_example.txt");

    #[test]
    fn test_part1() {
        let input = Day21::parse_input(EXAMPLE_INPUT);
        let output = Day21::part_1(input);
        assert_eq!(output, 126384);
    }

    #[test]
    fn test_part2() {
        let input = Day21::parse_input(EXAMPLE_INPUT);
        let output = Day21::part_2(input);
        assert_eq!(output, 154115708116294);
    }
}
