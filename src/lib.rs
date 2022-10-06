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
use iterators::DecimalsDescending;
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
    pub fn are_same_sign(self: &Self, other: &Self) -> bool {
        return self.negative == other.negative;
    }
    pub fn is_negative(self: &Self) -> bool {
        return self.negative;
    }
    pub fn is_positive(self: &Self) -> bool {
        return !self.negative;
    }
    pub fn negate(self: &mut Self) {
        self.negative = !self.negative;
    }
    pub fn zero() -> BigNumber {
        return BigNumber {
            integer: vec![Digits::Zero],
            decimal: vec![Digits::Zero],
            negative: false,
        };
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
                    return BigNumber::zero();
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

        let order: Ordering = lhs.cmp(&rhs);
        // Subtracting equal quantities from each other is defined to be zero.
        if order == Ordering::Equal {
            return BigNumber::zero();
        }

        let length: usize = cmp::max(lhs.decimal.len(), rhs.decimal.len());
        let mut results_decimal: Vec<Digits> = Vec::with_capacity(length);
        let mut carry: Digits = Digits::Zero;
        let mut temp: Digits;

        if lhs.are_same_sign(&rhs) {
            let da: DecimalsAscending = DecimalsAscending::new(&lhs.decimal, &rhs.decimal);
            for (x, y) in da {
                (temp, carry) = Digits::complement(x).fused_addition(y, carry);
                results_decimal.push(temp);
            }
            results_decimal.reverse();
            for x in results_decimal.iter_mut() {
                *x = Digits::complement(*x);
            }

            let length: usize = cmp::max(lhs.integer.len(), rhs.integer.len());
            let mut results_integer: Vec<Digits> = Vec::with_capacity(length);
            if length == 0 {
                results_integer.push(Digits::complement(carry));
            } else {
                let ia: IntegersAscending = IntegersAscending::new(&lhs.integer, &rhs.decimal);
                for (x, y) in ia {
                    (temp, carry) = Digits::complement(x).fused_addition(y, carry);
                    results_integer.push(temp);
                }
                results_integer.reverse();
                for x in results_integer.iter_mut() {
                    *x = Digits::complement(*x);
                }
            }
            return BigNumber {
                integer: results_integer,
                decimal: results_decimal,
                negative: lhs.negative,
            };
        }
        todo!();
    }
}

impl Ord for BigNumber {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let lhs = &self;
        // Negative numbers are always less than positive numbers.
        if lhs.is_negative() && rhs.is_positive() {
            return Ordering::Less;
        }
        if lhs.is_positive() && rhs.is_negative() {
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
                let mut order: Ordering = Ordering::Equal;
                let mut dd: DecimalsDescending =
                    DecimalsDescending::new(&lhs.decimal, &rhs.decimal);
                while order == Ordering::Equal {
                    let next = dd.next();
                    if next.is_none() {
                        break;
                    } else {
                        let (x, y) = next.unwrap();
                        order = x.cmp(&y);
                    }
                }
                if lhs.is_positive() {
                    match order {
                        Ordering::Equal => return Ordering::Equal,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                    };
                } else {
                    // Negative numbers
                    match order {
                        Ordering::Equal => return Ordering::Equal,
                        Ordering::Greater => return Ordering::Less,
                        Ordering::Less => return Ordering::Greater,
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
