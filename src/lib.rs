// Temporary during development.
#![allow(dead_code)]

// Internal module declarations and imports.
mod digits;
use digits::Digits;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MathErrors {
    DivisionByZero,
}

struct BigNumber {
    whole: Vec<Digits>,
    fraction: Vec<Digits>,
    decimal_point: u16,
    negative: bool,
}
