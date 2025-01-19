use {aoc_core::Solution, std::collections::HashMap};

pub struct Day5;

pub type Rule = (usize, fn(usize, usize) -> bool);

impl Solution<'_> for Day5 {
    type Input = (HashMap<usize, Vec<Rule>>, Vec<Vec<usize>>);
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        let (rules, pages) = data
            .split_once("\n\n")
            .expect("Blank line between sections");

        let mut parsed_rules: HashMap<usize, Vec<Rule>> = HashMap::new();
        for line in rules.lines() {
            let (earlier, later) = line.split_once('|').expect("Has pipe");
            let earlier: usize = earlier.parse().expect("Is number");
            let later: usize = later.parse().expect("Is number");
            let rule1: Rule = (later, less);
            let rule2: Rule = (earlier, greater);
            parsed_rules.entry(earlier).or_default().push(rule1);
            parsed_rules.entry(later).or_default().push(rule2);
        }

        let pages = pages
            .lines()
            .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();

        (parsed_rules, pages)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let (rules, pages) = input;
        let mut total = 0;
        for ps in pages {
            if check_rules(&ps, &rules) {
                total += ps[ps.len() / 2];
            }
        }
        total
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let (rules, pages) = input;
        let mut total = 0;
        for ps in pages {
            if !check_rules(&ps, &rules) {
                let sorted = resort(&ps, &rules);
                total += sorted[ps.len() / 2];
            }
        }
        total
    }
}

fn less(i: usize, j: usize) -> bool {
    i < j
}

fn greater(i: usize, j: usize) -> bool {
    i > j
}

fn check_rules(pages: &[usize], rules: &HashMap<usize, Vec<Rule>>) -> bool {
    let index: HashMap<usize, usize> = pages.iter().enumerate().map(|(i, p)| (*p, i)).collect();
    find_failing_index(&index, rules).is_none()
}

fn find_failing_index(
    index: &HashMap<usize, usize>,
    rules: &HashMap<usize, Vec<Rule>>,
) -> Option<((usize, usize), (usize, usize))> {
    index.iter().find_map(|(p, i)| {
        let rules = rules.get(p).map(|rs| rs.as_slice()).unwrap_or(&[]);
        rules.iter().find_map(move |(q, rule)| {
            let j = *index.get(q)?;
            let i = *i;
            if rule(i, j) {
                None
            } else {
                Some(((*p, i), (*q, j)))
            }
        })
    })
}

fn fix_single_rule(
    index: &mut HashMap<usize, usize>,
    rules: &HashMap<usize, Vec<Rule>>,
) -> Option<()> {
    let ((p, i), (q, j)) = find_failing_index(index, rules)?;
    *index.get_mut(&p).unwrap() = j;
    *index.get_mut(&q).unwrap() = i;
    Some(())
}

fn resort(pages: &[usize], rules: &HashMap<usize, Vec<Rule>>) -> Vec<usize> {
    let mut index: HashMap<usize, usize> = pages.iter().enumerate().map(|(i, p)| (*p, i)).collect();
    while fix_single_rule(&mut index, rules).is_some() {}
    let n = pages.len();
    let mut result = vec![0; n];
    for (p, i) in index {
        result[i] = p;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day5_example.txt");

    #[test]
    fn test_part1() {
        let input = Day5::parse_input(EXAMPLE_INPUT);
        let output = Day5::part_1(input);
        assert_eq!(output, 143);
    }

    #[test]
    fn test_part2() {
        let input = Day5::parse_input(EXAMPLE_INPUT);
        let output = Day5::part_2(input);
        assert_eq!(output, 123);
    }
}
