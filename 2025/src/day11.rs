use {aoc_core::Solution, std::collections::HashMap};

pub struct Day11;

impl<'a> Solution<'a> for Day11 {
    type Input = HashMap<&'a str, Vec<&'a str>>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &'a str) -> Self::Input {
        data.lines()
            .map(|l| {
                let (label, neighbors) = l.split_once(':').expect("Has colon");
                let neighbors: Vec<&'a str> = neighbors.trim().split(' ').collect();
                (label, neighbors)
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        // Can ignore `dac` and `fft` in part 1.
        memoized_paths_to_out(("you", true, true), &input, &mut HashMap::new())
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        memoized_paths_to_out(("svr", false, false), &input, &mut HashMap::new())
    }
}

type MemoInput<'a> = (&'a str, bool, bool);

fn memoized_paths_to_out<'a>(
    x: MemoInput<'a>,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<MemoInput<'a>, usize>,
) -> usize {
    if let Some(result) = memo.get(&x) {
        return *result;
    }

    let (source, mut has_dac, mut has_fft) = x;

    match source {
        "out" if has_dac && has_fft => {
            return 1;
        }
        "out" => {
            return 0;
        }
        "dac" => {
            has_dac = true;
        }
        "fft" => {
            has_fft = true;
        }
        _ => (),
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(source) {
        for &next_node in neighbors {
            let x = (next_node, has_dac, has_fft);
            total += memoized_paths_to_out(x, graph, memo);
        }
    }
    memo.insert(x, total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day11_example.txt");
    const EXAMPLE_INPUT_2: &str = include_str!("res/day11_example2.txt");

    #[test]
    fn test_part1() {
        let input = Day11::parse_input(EXAMPLE_INPUT);
        let output = Day11::part_1(input);
        assert_eq!(output, 5);
    }

    #[test]
    fn test_part2() {
        let input = Day11::parse_input(EXAMPLE_INPUT_2);
        let output = Day11::part_2(input);
        assert_eq!(output, 2);
    }
}
