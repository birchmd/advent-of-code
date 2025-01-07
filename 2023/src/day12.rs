use {
    aoc_core::Solution,
    std::{cmp::Ordering, collections::HashMap},
};

pub struct Day12;

impl Solution<'_> for Day12 {
    type Input = Vec<(SpringData, ContiguousData)>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &'_ str) -> Self::Input {
        data.lines()
            .map(|line| {
                let (spring_data, contiguous_data) = line.split_once(' ').expect("Has space");
                (
                    spring_data.bytes().map(SpringDatum::from_u8).collect(),
                    contiguous_data
                        .split(',')
                        .map(|x| x.parse().expect("Contiguous data consists of numbers"))
                        .collect(),
                )
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let mut cache = HashMap::new();
        input
            .into_iter()
            .map(|(spring_data, contiguous_data)| {
                count_configurations(spring_data, contiguous_data, &mut cache)
            })
            .sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let mut cache = HashMap::new();
        input
            .iter()
            .map(|(spring_data, contiguous_data)| {
                let unfolded_spring_data =
                    vec![spring_data.to_vec(); 5].join(&SpringDatum::Unknown);
                let unfolded_contiguous_data = std::iter::repeat_n(contiguous_data, 5)
                    .flatten()
                    .copied()
                    .collect();
                count_configurations(unfolded_spring_data, unfolded_contiguous_data, &mut cache)
            })
            .sum()
    }
}

fn count_configurations(
    mut spring_data: SpringData,
    mut contiguous_data: ContiguousData,
    cache: &mut HashMap<(SpringData, ContiguousData), u64>,
) -> u64 {
    let cache_key = (spring_data.clone(), contiguous_data.clone());

    if let Some(result) = cache.get(&cache_key) {
        return *result;
    }

    if contiguous_data.is_empty() {
        if spring_data.contains(&SpringDatum::Broken) {
            return 0;
        } else {
            return 1;
        }
    }

    let result = match spring_data.pop() {
        None => 0,
        Some(SpringDatum::Operational) => count_configurations(spring_data, contiguous_data, cache),
        Some(SpringDatum::Unknown) => {
            let operational = spring_data.clone();

            let mut broken = spring_data;
            broken.push(SpringDatum::Broken);

            let a = count_configurations(operational, contiguous_data.clone(), cache);
            let b = count_configurations(broken, contiguous_data, cache);

            a + b
        }
        Some(SpringDatum::Broken) => {
            let Some(goal_length) = contiguous_data.pop() else {
                cache.insert(cache_key, 0);
                return 0;
            };
            let mut count = 1;
            while let Some(SpringDatum::Broken) = spring_data.last() {
                spring_data.pop();
                count += 1;
            }
            match count.cmp(&goal_length) {
                Ordering::Greater => 0,
                Ordering::Equal => {
                    spring_data.pop();
                    count_configurations(spring_data, contiguous_data, cache)
                }
                Ordering::Less => {
                    for _ in 0..(goal_length - count) {
                        if let Some(SpringDatum::Operational) | None = spring_data.pop() {
                            cache.insert(cache_key, 0);
                            return 0;
                        }
                    }
                    if let Some(SpringDatum::Broken) = spring_data.last() {
                        cache.insert(cache_key, 0);
                        return 0;
                    }
                    spring_data.pop();
                    count_configurations(spring_data, contiguous_data, cache)
                }
            }
        }
    };

    cache.insert(cache_key, result);
    result
}

pub type SpringData = Vec<SpringDatum>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpringDatum {
    Operational,
    Broken,
    Unknown,
}

impl SpringDatum {
    fn from_u8(x: u8) -> Self {
        match x {
            b'.' => Self::Operational,
            b'#' => Self::Broken,
            b'?' => Self::Unknown,
            _ => panic!("Bad datum"),
        }
    }
}

pub type ContiguousData = Vec<usize>;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day12_example.txt");

    #[test]
    fn test_part1() {
        let input = Day12::parse_input(EXAMPLE_INPUT);
        let output = Day12::part_1(input);
        assert_eq!(output, 21);
    }

    #[test]
    fn test_part2() {
        let input = Day12::parse_input(EXAMPLE_INPUT);
        let output = Day12::part_2(input);
        assert_eq!(output, 525152);
    }
}
