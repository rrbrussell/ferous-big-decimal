// Temporary during development.
#![allow(dead_code)]

// External imports
use std::fmt;

// Internal module declarations and imports.
mod digits;
use digits::Digits;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MathErrors {
    DivisionByZero,
}

pub struct BigNumber {
    whole: Vec<Digits>,
    fraction: Vec<Digits>,
    negative: bool,
}

impl From<u128> for BigNumber {
    fn from(input: u128) -> Self {
        let mut x = input;
        let mut jim: Vec<Digits> = Vec::with_capacity(16);
        while x > 10 {
            let t = x % 10;
            jim.push(Digits::from(t));
            x /= 10;
        }
        jim.push(Digits::from(x));
        jim.reverse();
        return BigNumber {
            whole: jim,
            fraction: Vec::new(),
            negative: false,
        };
    }
}

impl fmt::Display for BigNumber {
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {write!(formatter, "-")?;};
        let mut whole = String::with_capacity(self.whole.len());
        for x in self.whole.iter() {
            whole.push(x.to_char());
        }
        if self.fraction.len() > 0 {
            let mut fraction = String::with_capacity(self.fraction.len());
            for x in self.fraction.iter() {
                fraction.push(x.to_char());
            }
            if self.whole.len() == 0 {
                return write!(formatter, "0.{fraction}");
            } else {
                return write!(formatter, "{whole}.{fraction}");
            }
        }
        if self.whole.len() == 0 { return write!(formatter, "0"); };
        return write!(formatter, "{whole}");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_big_number_display() {
        let test_data = [
            (BigNumber{ whole: Vec::new(), fraction: Vec::new(), negative: false,}, "0"),
            (BigNumber{ whole: Vec::new(), fraction: Vec::new(), negative: true,}, "-0"),
            (BigNumber{ whole: vec![Digits::Zero], fraction: Vec::new(), negative: false,}, "0"),
            (BigNumber{ whole: Vec::new(), fraction: vec![Digits::Zero, Digits::One], negative: false,}, "0.01"),
        ];
        for (bn, expected) in test_data {
            assert_eq!(format!("{bn}"), expected);
        }
    }
}
