// Temporary during development.
#![allow(dead_code)]

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
    decimal_point: u16,
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
            decimal_point: 0,
            negative: false,
        };
    }
}
