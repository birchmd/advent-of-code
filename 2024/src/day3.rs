//! I know that regex with capture groups is the right way
//! to solve this problem. But I thought writing out the state
//! machine by hand would be fun.

use aoc_core::Solution;

pub struct Day3;

impl<'a> Solution<'a> for Day3 {
    type Input = &'a str;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input(data: &'a str) -> Self::Input {
        data
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        execute_state_machine(input, false)
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        execute_state_machine(input, true)
    }
}

fn execute_state_machine(input: &str, dos_and_donts: bool) -> u64 {
    let mut state = State::new(dos_and_donts);
    for c in input.bytes() {
        state = state.transition(c);
    }
    state.total
}

enum Label {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
}

struct State {
    label: Label,
    x: String,
    y: String,
    total: u64,
    enabled: bool,
    dos_and_donts: bool,
}

impl State {
    fn new(dos_and_donts: bool) -> Self {
        Self {
            label: Label::One,
            x: String::new(),
            y: String::new(),
            total: 0,
            enabled: true,
            dos_and_donts,
        }
    }

    fn transition(mut self, c: u8) -> Self {
        let label = match self.label {
            Label::One => self.t1(c),
            Label::Two => self.t2(c),
            Label::Three => self.t3(c),
            Label::Four => self.t4(c),
            Label::Five => self.t5(c),
            Label::Six => self.t6(c),
            Label::Seven => self.t7(c),
            Label::Eight => self.t8(c),
            Label::Nine => self.t9(c),
            Label::Ten => self.t10(c),
            Label::Eleven => self.t11(c),
            Label::Twelve => self.t12(c),
            Label::Thirteen => self.t13(c),
            Label::Fourteen => self.t14(c),
            Label::Fifteen => self.t15(c),
            Label::Sixteen => self.t16(c),
            Label::Seventeen => self.t17(c),
            Label::Eighteen => self.t18(c),
            Label::Nineteen => self.t19(c),
        };
        Self { label, ..self }
    }

    fn mul_finish(&mut self) -> Label {
        self.total += self.x.parse::<u64>().unwrap() * self.y.parse::<u64>().unwrap();
        Label::One
    }

    fn t1(&mut self, c: u8) -> Label {
        self.x.clear();
        self.y.clear();
        if c == b'm' && self.enabled {
            Label::Two
        } else if c == b'd' && self.dos_and_donts {
            Label::Thirteen
        } else {
            Label::One
        }
    }

    fn t2(&self, c: u8) -> Label {
        if c == b'u' {
            Label::Three
        } else {
            Label::One
        }
    }

    fn t3(&self, c: u8) -> Label {
        if c == b'l' {
            Label::Four
        } else {
            Label::One
        }
    }

    fn t4(&self, c: u8) -> Label {
        if c == b'(' {
            Label::Five
        } else {
            Label::One
        }
    }

    fn t5(&mut self, c: u8) -> Label {
        if c.is_ascii_digit() {
            self.x.push(c.into());
            Label::Six
        } else {
            Label::One
        }
    }

    fn t6(&mut self, c: u8) -> Label {
        if c.is_ascii_digit() {
            self.x.push(c.into());
            Label::Seven
        } else if c == b',' {
            Label::Nine
        } else {
            Label::One
        }
    }

    fn t7(&mut self, c: u8) -> Label {
        if c.is_ascii_digit() {
            self.x.push(c.into());
            Label::Eight
        } else if c == b',' {
            Label::Nine
        } else {
            Label::One
        }
    }

    fn t8(&self, c: u8) -> Label {
        if c == b',' {
            Label::Nine
        } else {
            Label::One
        }
    }

    fn t9(&mut self, c: u8) -> Label {
        if c.is_ascii_digit() {
            self.y.push(c.into());
            Label::Ten
        } else {
            Label::One
        }
    }

    fn t10(&mut self, c: u8) -> Label {
        if c.is_ascii_digit() {
            self.y.push(c.into());
            Label::Eleven
        } else if c == b')' {
            self.mul_finish()
        } else {
            Label::One
        }
    }

    fn t11(&mut self, c: u8) -> Label {
        if c.is_ascii_digit() {
            self.y.push(c.into());
            Label::Twelve
        } else if c == b')' {
            self.mul_finish()
        } else {
            Label::One
        }
    }

    fn t12(&mut self, c: u8) -> Label {
        if c == b')' {
            self.mul_finish()
        } else {
            Label::One
        }
    }

    fn t13(&self, c: u8) -> Label {
        if c == b'o' {
            Label::Fourteen
        } else {
            Label::One
        }
    }

    fn t14(&self, c: u8) -> Label {
        if c == b'(' {
            Label::Fifteen
        } else if c == b'n' {
            Label::Sixteen
        } else {
            Label::One
        }
    }

    fn t15(&mut self, c: u8) -> Label {
        if c == b')' {
            self.enabled = true;
        }
        Label::One
    }

    fn t16(&self, c: u8) -> Label {
        if c == b'\'' {
            Label::Seventeen
        } else {
            Label::One
        }
    }

    fn t17(&self, c: u8) -> Label {
        if c == b't' {
            Label::Eighteen
        } else {
            Label::One
        }
    }

    fn t18(&self, c: u8) -> Label {
        if c == b'(' {
            Label::Nineteen
        } else {
            Label::One
        }
    }

    fn t19(&mut self, c: u8) -> Label {
        if c == b')' {
            self.enabled = false;
        }
        Label::One
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day3_example.txt");
    const EXAMPLE_INPUT2: &str = include_str!("res/day3_part2_example.txt");

    #[test]
    fn test_part1() {
        let input = Day3::parse_input(EXAMPLE_INPUT);
        let output = Day3::part_1(input);
        assert_eq!(output, 161);
    }

    #[test]
    fn test_part2() {
        let input = Day3::parse_input(EXAMPLE_INPUT2);
        let output = Day3::part_2(input);
        assert_eq!(output, 48);
    }
}
