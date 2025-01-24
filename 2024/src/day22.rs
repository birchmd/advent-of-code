use {
    aoc_core::Solution,
    std::collections::{HashMap, HashSet},
};

pub struct Day22;

impl Solution<'_> for Day22 {
    type Input = Vec<u64>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &'_ str) -> Self::Input {
        data.lines().map(|x| x.parse().unwrap()).collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        input.into_iter().map(two_thousandth).sum()
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let buyers: Vec<HashMap<[i8; 4], u8>> = input.into_iter().map(rank_sequences).collect();
        let distinct_seqs: HashSet<[i8; 4]> =
            buyers.iter().flat_map(|b| b.keys()).copied().collect();
        distinct_seqs
            .into_iter()
            .map(|seq| total_bananas(seq, &buyers))
            .max()
            .unwrap()
    }
}

fn next_secret(mut secret: u64) -> u64 {
    let x = secret * 64;
    secret = (x ^ secret) % 16777216;
    let y = secret / 32;
    secret = (y ^ secret) % 16777216;
    let z = secret * 2048;
    (z ^ secret) % 16777216
}

fn two_thousandth(mut secret: u64) -> u64 {
    for _ in 0..2000 {
        secret = next_secret(secret);
    }
    secret
}

fn price_diffs(secret: u64) -> (Vec<u8>, Vec<i8>) {
    let mut secrets = Vec::with_capacity(2001);
    secrets.push(secret);
    for i in 1..2001 {
        secrets.push(next_secret(secrets[i - 1]));
    }
    let prices: Vec<u8> = secrets.into_iter().map(|s| (s % 10) as u8).collect();
    let dps: Vec<i8> = prices
        .iter()
        .zip(&prices[1..])
        .map(|(p0, p1)| (*p1 as i8) - (*p0 as i8))
        .collect();
    (prices[1..].to_vec(), dps)
}

fn rank_sequences(secret: u64) -> HashMap<[i8; 4], u8> {
    let (prices, dps) = price_diffs(secret);
    let mut seq_prices = HashMap::new();
    let n = dps.len();
    for i in 3..n {
        let seq = [dps[i - 3], dps[i - 2], dps[i - 1], dps[i]];
        seq_prices.entry(seq).or_insert(prices[i]);
    }
    seq_prices
}

fn total_bananas(seq: [i8; 4], buyers: &[HashMap<[i8; 4], u8>]) -> u64 {
    buyers
        .iter()
        .filter_map(|b| b.get(&seq).map(|p| *p as u64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day22_example.txt");
    const EXAMPLE_INPUT2: &str = include_str!("res/day22_example_part2.txt");

    #[test]
    fn test_part1() {
        let input = Day22::parse_input(EXAMPLE_INPUT);
        let output = Day22::part_1(input);
        assert_eq!(output, 37_327_623);
    }

    #[test]
    fn test_part2() {
        let input = Day22::parse_input(EXAMPLE_INPUT2);
        let output = Day22::part_2(input);
        assert_eq!(output, 23);
    }
}
