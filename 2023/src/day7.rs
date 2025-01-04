use aoc_core::{count_distinct, Solution};

pub struct Day7;

impl Solution<'_> for Day7 {
    type Input = Vec<(Hand, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &str) -> Self::Input {
        data.lines()
            .map(|line| {
                let (cards, bid) = line.split_once(' ').expect("Contains space");
                (Hand::from_str(cards), bid.parse().expect("Bid is number"))
            })
            .collect()
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let mut hands = input;
        hands.sort_by_key(|(hand, _)| (hand.kind(), hand.0.clone()));
        rank_sum(hands)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        let mut hands = input;
        for (hand, _) in &mut hands {
            hand.jacks_are_jokers();
        }
        hands.sort_by_key(|(hand, _)| (hand.kind(), hand.0.clone()));
        rank_sum(hands)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand(Vec<Card>);

impl Hand {
    fn from_str(data: &str) -> Self {
        Self(data.bytes().map(Card::from_u8).collect())
    }

    fn kind(&self) -> Kind {
        let mut counts = count_distinct(&self.0);
        if let Some(joker_count) = counts.remove(&&Card::Joker) {
            let Some(highest_count) = counts.keys().max_by_key(|c| counts[*c]) else {
                // If Joker was the only card then removing the joker
                // count leaves the map empty, so this max function returns
                // None. But if Joker was the only card then it is was
                // five of a kind hand.
                return Kind::FiveOf;
            };
            let high_count = counts.entry(highest_count).or_default();
            *high_count += joker_count;
        }
        let mut counts: Vec<usize> = counts.into_values().collect();
        counts.sort_unstable();
        match counts.as_slice() {
            [5] => Kind::FiveOf,
            [1, 4] => Kind::FourOf,
            [2, 3] => Kind::FullHouse,
            [1, 1, 3] => Kind::ThreeOf,
            [1, 2, 2] => Kind::TwoPair,
            [1, 1, 1, 2] => Kind::OnePair,
            _ => Kind::HighCard,
        }
    }

    fn jacks_are_jokers(&mut self) {
        for card in self.0.iter_mut() {
            if let Card::Jack = card {
                *card = Card::Joker;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_u8(x: u8) -> Self {
        match x {
            b'2' => Self::Two,
            b'3' => Self::Three,
            b'4' => Self::Four,
            b'5' => Self::Five,
            b'6' => Self::Six,
            b'7' => Self::Seven,
            b'8' => Self::Eight,
            b'9' => Self::Nine,
            b'T' => Self::Ten,
            b'J' => Self::Jack,
            b'Q' => Self::Queen,
            b'K' => Self::King,
            b'A' => Self::Ace,
            _ => panic!("Unknown card"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOf,
    FullHouse,
    FourOf,
    FiveOf,
}

fn rank_sum(hands: Vec<(Hand, u64)>) -> u64 {
    let mut total = 0;
    for (i, (_, bid)) in hands.into_iter().enumerate() {
        let rank = (i as u64) + 1;
        total += rank * bid;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day7_example.txt");

    #[test]
    fn test_part1() {
        let input = Day7::parse_input(EXAMPLE_INPUT);
        let output = Day7::part_1(input);
        assert_eq!(output, 6440);
    }

    #[test]
    fn test_part2() {
        let input = Day7::parse_input(EXAMPLE_INPUT);
        let output = Day7::part_2(input);
        assert_eq!(output, 5905);
    }
}
