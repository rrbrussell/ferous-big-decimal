// Temporary during development.
#![allow(dead_code)]

// External imports
use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::Sub;

// Internal module declarations and imports.
mod digits;
mod iterators;
use digits::Digits;
use iterators::DecimalsAscending;
use iterators::IntegersAscending;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MathErrors {
    DivisionByZero,
}

pub struct BigNumber {
    integer: Vec<Digits>,
    decimal: Vec<Digits>,
    negative: bool,
}

impl BigNumber {
    pub fn is_negative(self: &Self) -> bool {
        return self.negative;
    }
    pub fn is_positive(self: &Self) -> bool {
        return !self.negative;
    }
    pub fn negate(self: &mut Self) {
        self.negative = !self.negative;
    }
}

impl Clone for BigNumber {
    fn clone(&self) -> Self {
        let mut bn = BigNumber {
            integer: Vec::<Digits>::with_capacity(self.integer.len()),
            decimal: Vec::<Digits>::with_capacity(self.decimal.len()),
            negative: self.negative,
        };
        bn.integer.extend(self.integer.iter());
        bn.decimal.extend(self.decimal.iter());
        return bn;
    }
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
            integer: jim,
            decimal: Vec::new(),
            negative: false,
        };
    }
}

impl Add for BigNumber {
    type Output = Self;

    fn add(self: Self, rhs: Self) -> Self {
        let lhs = &self;
        let length = cmp::max(lhs.decimal.len(), rhs.decimal.len());
        let mut result_decimal: Vec<Digits> = Vec::with_capacity(length);
        let mut carry = Digits::Zero;
        let mut temp: Digits;

        if lhs.negative == rhs.negative {
            if length != 0 {
                let da = DecimalsAscending::new(&lhs.decimal, &rhs.decimal);
                for (x, y) in da {
                    (temp, carry) = x.fused_addition(y, carry);
                    result_decimal.push(temp);
                }
                result_decimal.reverse();
            }

            let length = cmp::max(lhs.integer.len(), rhs.integer.len());
            let mut result_integer: Vec<Digits> = Vec::with_capacity(length);
            if length == 0 {
                result_integer.push(carry);
            } else {
                let ia = IntegersAscending::new(&lhs.integer, &rhs.integer);
                for (x, y) in ia {
                    (temp, carry) = x.fused_addition(y, carry);
                    result_integer.push(temp);
                }
                result_integer.reverse();
            }
            return BigNumber {
                integer: result_integer,
                decimal: result_decimal,
                negative: lhs.negative,
            };
        } else {
            match lhs.cmp(&rhs) {
                Ordering::Equal => {
                    return BigNumber {
                        integer: Vec::new(),
                        decimal: Vec::new(),
                        negative: false,
                    }
                }
                Ordering::Greater => {
                    let mut rhsc = rhs.clone();
                    rhsc.negate();
                    let mut result = lhs.clone().sub(rhsc);
                    result.negative = lhs.negative;
                    return result;
                }
                Ordering::Less => {
                    let mut lhsc = lhs.clone();
                    lhsc.negate();
                    let mut result = rhs.clone().sub(lhsc);
                    result.negative = rhs.negative;
                    return result;
                }
            }
        }
    }
}

impl Sub for BigNumber {
    type Output = Self;

    fn sub(self: Self, rhs: Self) -> Self {
        let lhs: &BigNumber = &self;
        let length: usize = cmp::max(lhs.decimal.len(), rhs.decimal.len());
        let mut results_decimal: Vec<Digits> = Vec::with_capacity(length);
        let mut carry: Digits = Digits::Zero;
        let mut temp: Digits;

        if lhs.negative == rhs.negative {
            if length != 0 {
                let da: DecimalsAscending = DecimalsAscending::new(&lhs.decimal, &rhs.decimal);
                for (x, y) in da {
                    (temp, carry) = x.fused_subtraction(y, carry);
                    results_decimal.push(temp);
                }
                results_decimal.reverse();
            }

            let length: usize = cmp::max(lhs.integer.len(), rhs.integer.len());
            let mut results_integer: Vec<Digits> = Vec::with_capacity(length);
            if length == 0 {
                results_integer.push(carry)
            }
        } else {
        }
        todo!();
    }
}

impl Ord for BigNumber {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let lhs = &self;
        // Negative numbers are always less than positive numbers.
        if lhs.negative && (rhs.negative == false) {
            return Ordering::Less;
        }
        if rhs.negative && (lhs.negative == false) {
            return Ordering::Greater;
        }
        // We now know that lhs and rhs have the same sign.
        // Whichever side has more integer digits is larger.
        if lhs.integer.len() > rhs.integer.len() {
            return Ordering::Greater;
        }
        if rhs.integer.len() > rhs.integer.len() {
            return Ordering::Less;
        }
        // We now know that lhs and rhs have the same number of integer digits.
        let x = lhs.integer.cmp(&rhs.integer);
        match x {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {
                // Okay now we have to use the decimal portion for comparison.
                // First check if the lengths of the decimal portion are equal.
                if lhs.decimal.len() == rhs.decimal.len() {
                    return lhs.decimal.cmp(&rhs.decimal);
                }
                // Now, who has the shorter length decimal portion.
                if lhs.decimal.len() > rhs.decimal.len() {
                    // lhs has a longer decimal portion.
                    let x = &lhs.decimal[(0..rhs.decimal.len())];
                    let x_compared = x.cmp(&rhs.decimal);
                    if !lhs.negative {
                        // positive numbers
                        match x_compared {
                            Ordering::Equal => return Ordering::Greater,
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Less => return Ordering::Less,
                        };
                    } else {
                        // Negative numbers
                        match x_compared {
                            Ordering::Equal => return Ordering::Less,
                            Ordering::Greater => return Ordering::Less,
                            Ordering::Less => return Ordering::Greater,
                        }
                    }
                } else {
                    // rhs has a longer decimal portion.
                    let x = &rhs.decimal[(0..lhs.decimal.len())];
                    let x_compared = lhs.decimal.cmp(&Vec::from(x));
                    if !lhs.negative {
                        // positive numbers
                        match x_compared {
                            Ordering::Equal => return Ordering::Less,
                            Ordering::Greater => return Ordering::Less,
                            Ordering::Less => return Ordering::Greater,
                        };
                    } else {
                        // Negative numbers
                        match x_compared {
                            Ordering::Equal => return Ordering::Greater,
                            Ordering::Greater => return Ordering::Greater,
                            Ordering::Less => return Ordering::Less,
                        }
                    }
                }
            }
        }
    }
}

impl PartialOrd for BigNumber {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        return Some(self.cmp(rhs));
    }
}

impl PartialEq for BigNumber {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => return true,
            Ordering::Greater => return false,
            Ordering::Less => return false,
        }
    }
}

impl Eq for BigNumber {}

impl Display for BigNumber {
    fn fmt(self: &Self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if self.negative {
            write!(formatter, "-")?;
        };
        let mut whole = String::with_capacity(self.integer.len());
        for x in self.integer.iter() {
            whole.push(x.to_char());
        }
        if self.decimal.len() > 0 {
            let mut fraction = String::with_capacity(self.decimal.len());
            for x in self.decimal.iter() {
                fraction.push(x.to_char());
            }
            if self.integer.len() == 0 {
                return write!(formatter, "0.{fraction}");
            } else {
                return write!(formatter, "{whole}.{fraction}");
            }
        }
        if self.integer.len() == 0 {
            return write!(formatter, "0");
        };
        return write!(formatter, "{whole}");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_big_number_display() {
        let test_data = [
            (
                BigNumber {
                    integer: Vec::new(),
                    decimal: Vec::new(),
                    negative: false,
                },
                "0",
            ),
            (
                BigNumber {
                    integer: Vec::new(),
                    decimal: Vec::new(),
                    negative: true,
                },
                "-0",
            ),
            (
                BigNumber {
                    integer: vec![Digits::Zero],
                    decimal: Vec::new(),
                    negative: false,
                },
                "0",
            ),
            (
                BigNumber {
                    integer: Vec::new(),
                    decimal: vec![Digits::Zero, Digits::One],
                    negative: false,
                },
                "0.01",
            ),
        ];
        for (bn, expected) in test_data {
            assert_eq!(format!("{bn}"), expected);
        }
    }

    #[test]
    fn test_big_number_cmp() {
        let test_data = [
            (
                (
                    BigNumber {
                        integer: vec![Digits::Three, Digits::Zero],
                        decimal: Vec::new(),
                        negative: true,
                    },
                    BigNumber {
                        integer: vec![Digits::Three],
                        decimal: Vec::new(),
                        negative: false,
                    },
                ),
                Ordering::Less,
            ),
            (
                (
                    BigNumber {
                        integer: vec![Digits::Three, Digits::Zero],
                        decimal: Vec::new(),
                        negative: false,
                    },
                    BigNumber {
                        integer: vec![Digits::Three],
                        decimal: Vec::new(),
                        negative: true,
                    },
                ),
                Ordering::Greater,
            ),
            (
                (
                    BigNumber {
                        integer: vec![Digits::Three, Digits::Zero],
                        decimal: Vec::new(),
                        negative: false,
                    },
                    BigNumber {
                        integer: vec![Digits::Three],
                        decimal: Vec::new(),
                        negative: false,
                    },
                ),
                Ordering::Greater,
            ),
            (
                (
                    BigNumber {
                        integer: vec![Digits::Three],
                        decimal: Vec::new(),
                        negative: false,
                    },
                    BigNumber {
                        integer: vec![Digits::Three, Digits::Zero],
                        decimal: Vec::new(),
                        negative: false,
                    },
                ),
                Ordering::Less,
            ),
        ];
        for ((left, right), expected) in test_data {
            assert_eq!(left.cmp(&right), expected);
        }
    }
}
