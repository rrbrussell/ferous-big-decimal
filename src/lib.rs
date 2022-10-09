// Temporary during development.
#![allow(dead_code)]

// External imports
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::Not;
use std::ops::Sub;
use std::str::FromStr;

// Internal module declarations and imports.
mod digits;
mod iterators;
use digits::Digits;
use iterators::DecimalsByAscendingPower;
use iterators::DecimalsByDescendingPower;
use iterators::IntegersByAscendingPower;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MathErrors {
    DivisionByZero,
    ParseError,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
enum Sign {
    Negative,
    #[default]
    Positive,
}

impl Sign {
    fn is_negative(self) -> bool {
        return self == Sign::Negative;
    }
    fn is_positive(self) -> bool {
        return self == Sign::Positive;
    }
}

impl Not for Sign {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Sign::Negative => return Sign::Positive,
            Sign::Positive => return Sign::Negative,
        }
    }
}

/// BigNumber is my attempt at an arbitrary precision mathematical type for
/// Rust.
///
/// This is not intended to be the most performant option available. It is
/// mostly for me to learn more about Rust and practice programming.
///
/// Internally the type uses an enum that represents the decimal digits.
/// The integer and decimal portions are kept in separate Vecs.
#[derive(Debug)]
pub struct BigNumber {
    integer: Vec<Digits>,
    decimal: Vec<Digits>,
    sign: Sign,
}

impl BigNumber {
    /// Adds x to y.
    fn add_helper(x: &BigNumber, y: &BigNumber) -> BigNumber {
        let length: usize = cmp::max(x.decimal.len(), y.decimal.len());
        let mut result_decimal: Vec<Digits> = Vec::with_capacity(length);
        let mut carry: Digits = Digits::Zero;
        let mut temp: Digits;
        if length != 0 {
            let da: DecimalsByAscendingPower =
                DecimalsByAscendingPower::new(&x.decimal, &y.decimal);
            for (temp_x, temp_y) in da {
                (temp, carry) = temp_x.fused_addition(temp_y, carry);
                result_decimal.push(temp);
            }
            result_decimal.reverse();
        }

        let length: usize = cmp::max(x.integer.len(), y.integer.len());
        let mut result_integer: Vec<Digits> = Vec::with_capacity(length);
        if length == 0 {
            result_integer.push(carry);
        } else {
            let ia: IntegersByAscendingPower =
                IntegersByAscendingPower::new(&x.integer, &y.integer);
            for (temp_x, temp_y) in ia {
                (temp, carry) = temp_x.fused_addition(temp_y, carry);
                result_integer.push(temp);
            }
            if carry != Digits::Zero {
                result_integer.push(carry);
            }
            result_integer.reverse();
        }
        return BigNumber {
            integer: result_integer,
            decimal: result_decimal,
            sign: x.sign,
        };
    }

    pub fn is_the_same_sign_as(self: &Self, other: &Self) -> bool {
        return self.sign == other.sign;
    }
    pub fn is_negative(self: &Self) -> bool {
        return self.sign == Sign::Negative;
    }
    pub fn is_positive(self: &Self) -> bool {
        return self.sign == Sign::Positive;
    }
    pub fn negate(self: &mut Self) {
        self.sign = !self.sign;
    }

    /// Cleans up the internal representation of a BigNumber.
    pub fn normalize(self: &mut Self) {
        todo!();
    }

    /// this performs Nines Complement addition of the subtrahend and the
    /// minuend.
    fn sub_helper(subtrahend: &BigNumber, minuend: &BigNumber) -> BigNumber {
        let length: usize = cmp::max(subtrahend.decimal.len(), minuend.decimal.len());
        let mut results_decimal: Vec<Digits> = Vec::with_capacity(length);
        let mut carry: Digits = Digits::Zero;
        let mut temp: Digits;

        let da: DecimalsByAscendingPower =
            DecimalsByAscendingPower::new(&subtrahend.decimal, &minuend.decimal);
        for (x, y) in da {
            (temp, carry) = Digits::complement(x).fused_addition(y, carry);
            results_decimal.push(temp);
        }
        results_decimal.reverse();
        for x in results_decimal.iter_mut() {
            *x = Digits::complement(*x);
        }

        let length: usize = cmp::max(subtrahend.integer.len(), minuend.integer.len());
        let mut results_integer: Vec<Digits> = Vec::with_capacity(length);
        if length == 0 {
            results_integer.push(Digits::complement(carry));
        } else {
            let ia: IntegersByAscendingPower =
                IntegersByAscendingPower::new(&subtrahend.integer, &minuend.decimal);
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
            sign: Sign::Positive,
        };
    }

    /// Returns a BigNumber that represents zero.
    pub fn zero() -> BigNumber {
        return BigNumber {
            integer: vec![Digits::Zero],
            decimal: vec![Digits::Zero],
            sign: Sign::Positive,
        };
    }
}

impl Clone for BigNumber {
    fn clone(&self) -> Self {
        let mut bn = BigNumber {
            integer: Vec::<Digits>::with_capacity(self.integer.len()),
            decimal: Vec::<Digits>::with_capacity(self.decimal.len()),
            sign: self.sign,
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
            sign: Sign::Positive,
        };
    }
}

impl FromStr for BigNumber {
    type Err = MathErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // the (|\.\d+) requires any s to have either an integer portion
            // or an integer and a decimal portion.
            static ref RE: Regex = Regex::new(r"^-?\d+(|\.\d+)$").unwrap();
        }
        if !RE.is_match(s) {
            return Err(MathErrors::ParseError);
        }

        let result_sign: Sign;
        if s.starts_with('-') {
            result_sign = Sign::Negative;
        } else {
            result_sign = Sign::Positive;
        }

        let mut result_integer: Vec<Digits> = Vec::with_capacity(s.len());
        let mut result_decimal: Vec<Digits> = Vec::with_capacity(s.len());
        match s.find('.') {
            None => {
                if result_sign.is_negative() {
                    for character in s[1..].chars() {
                        result_integer.push(Digits::from(character));
                    }
                } else {
                    for character in s.chars() {
                        result_integer.push(Digits::from(character));
                    }
                }
            }
            Some(periods_location) => {
                if result_sign.is_negative() {
                    for character in s[1..periods_location].chars() {
                        result_integer.push(Digits::from(character));
                    }
                } else {
                    for character in s[0..periods_location].chars() {
                        result_integer.push(Digits::from(character));
                    }
                }
                for character in s[(periods_location + 1)..].chars() {
                    result_decimal.push(Digits::from(character));
                }
            }
        }

        return Ok(BigNumber {
            integer: result_integer,
            decimal: result_decimal,
            sign: result_sign,
        });
    }
}

impl Add for BigNumber {
    type Output = Self;

    /// Adds rhs to self.
    ///
    /// Call [`BigNumber::normalize`] on both values first.
    ///
    /// Rule 1: If self and rhs are both of the same sign then add rhs to self.
    /// The result has the same sign as both self and rhs.
    ///
    /// Rule 2: If self and rhs are not of the same sign the subtract the
    /// smaller magnitude number from the larger magnitude number. The result
    /// has the sign of the largest magnitude number.
    fn add(self: Self, rhs: Self) -> Self {
        let lhs: &BigNumber = &self;

        if lhs.is_the_same_sign_as(&rhs) {
            return BigNumber::add_helper(lhs, &rhs);
        } else {
            match lhs.cmp(&rhs) {
                Ordering::Equal => {
                    return BigNumber::zero();
                }
                Ordering::Greater => {
                    let mut result: BigNumber = BigNumber::sub_helper(lhs, &rhs);
                    result.sign = lhs.sign;
                    return result;
                }
                Ordering::Less => {
                    let mut result: BigNumber = BigNumber::sub_helper(&rhs, lhs);
                    result.sign = rhs.sign;
                    return result;
                }
            }
        }
    }
}

impl Sub for BigNumber {
    type Output = Self;

    /// Subtracts rhs from self.
    ///
    /// Call [`BigNumber::normalize`] on both values first.
    ///
    /// Rule 1: If self and rhs are both positive then subtract rhs from self.
    /// The answer is negative if rhs is greater than self.
    ///
    /// Rule 2: If self is negative and rhs is positive then add rhs to self.
    /// The answer is negative.
    ///
    /// Rule 3: If self is negative and rhs is negative then subtract the
    /// smaller magnitude number from the larger magnitude number. The answer
    /// is negative if self is larger. The answer is positive is rhs is larger.
    fn sub(self: Self, rhs: Self) -> Self {
        let lhs: &BigNumber = &self;

        let order: Ordering = lhs.cmp(&rhs);
        // Subtracting equal quantities from each other is defined to be zero.
        if order == Ordering::Equal {
            return BigNumber::zero();
        }
        let mut result: BigNumber = BigNumber::zero();
        if lhs.is_positive() && rhs.is_positive() {
            result = BigNumber::sub_helper(lhs, &rhs);
            if order == Ordering::Less {
                result.sign = Sign::Negative;
            }
        }
        if lhs.is_negative() && rhs.is_positive() {
            result = BigNumber::add_helper(lhs, &rhs);
            result.sign = Sign::Negative;
        }
        if lhs.is_negative() && rhs.is_negative() {
            if order == Ordering::Greater {
                result = BigNumber::sub_helper(lhs, &rhs);
                result.sign = Sign::Negative;
            }
            if order == Ordering::Less {
                result = BigNumber::sub_helper(&rhs, lhs);
                result.sign = Sign::Positive;
            }
        }
        return result;
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
                let mut dd: DecimalsByDescendingPower =
                    DecimalsByDescendingPower::new(&lhs.decimal, &rhs.decimal);
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
        if self.is_negative() {
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
                    sign: Sign::Positive,
                },
                "0",
            ),
            (
                BigNumber {
                    integer: Vec::new(),
                    decimal: Vec::new(),
                    sign: Sign::Negative,
                },
                "-0",
            ),
            (
                BigNumber {
                    integer: vec![Digits::Zero],
                    decimal: Vec::new(),
                    sign: Sign::Positive,
                },
                "0",
            ),
            (
                BigNumber {
                    integer: Vec::new(),
                    decimal: vec![Digits::Zero, Digits::One],
                    sign: Sign::Positive,
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
                        sign: Sign::Negative,
                    },
                    BigNumber {
                        integer: vec![Digits::Three],
                        decimal: Vec::new(),
                        sign: Sign::Positive,
                    },
                ),
                Ordering::Less,
            ),
            (
                (
                    BigNumber {
                        integer: vec![Digits::Three, Digits::Zero],
                        decimal: Vec::new(),
                        sign: Sign::Positive,
                    },
                    BigNumber {
                        integer: vec![Digits::Three],
                        decimal: Vec::new(),
                        sign: Sign::Negative,
                    },
                ),
                Ordering::Greater,
            ),
            (
                (
                    BigNumber {
                        integer: vec![Digits::Three, Digits::Zero],
                        decimal: Vec::new(),
                        sign: Sign::Positive,
                    },
                    BigNumber {
                        integer: vec![Digits::Three],
                        decimal: Vec::new(),
                        sign: Sign::Positive,
                    },
                ),
                Ordering::Greater,
            ),
            (
                (
                    BigNumber {
                        integer: vec![Digits::Three],
                        decimal: Vec::new(),
                        sign: Sign::Positive,
                    },
                    BigNumber {
                        integer: vec![Digits::Three, Digits::Zero],
                        decimal: Vec::new(),
                        sign: Sign::Positive,
                    },
                ),
                Ordering::Less,
            ),
        ];
        for ((left, right), expected) in test_data {
            assert_eq!(left.cmp(&right), expected);
        }
    }

    #[test]
    fn test_big_number_add() {
        let test_data = [
            (
                (
                    BigNumber {
                        integer: vec![Digits::Three],
                        decimal: Vec::new(),
                        sign: Sign::Positive,
                    },
                    BigNumber {
                        integer: vec![Digits::Three],
                        decimal: Vec::new(),
                        sign: Sign::Positive,
                    },
                ),
                BigNumber {
                    integer: vec![Digits::Six],
                    decimal: Vec::new(),
                    sign: Sign::Positive,
                },
            ),
            (
                (
                    BigNumber {
                        integer: vec![Digits::Seven],
                        decimal: Vec::new(),
                        sign: Sign::Positive,
                    },
                    BigNumber {
                        integer: vec![Digits::Seven],
                        decimal: vec![Digits::Seven],
                        sign: Sign::Positive,
                    },
                ),
                BigNumber {
                    integer: vec![Digits::One, Digits::Four],
                    decimal: vec![Digits::Seven],
                    sign: Sign::Positive,
                },
            ),
            (
                (
                    BigNumber {
                        integer: vec![Digits::Seven],
                        decimal: vec![Digits::Nine],
                        sign: Sign::Positive,
                    },
                    BigNumber {
                        integer: Vec::new(),
                        decimal: vec![Digits::Nine],
                        sign: Sign::Positive,
                    },
                ),
                BigNumber {
                    integer: vec![Digits::Eight],
                    decimal: vec![Digits::Eight],
                    sign: Sign::Positive,
                },
            ),
        ];
        for ((left, right), expected) in test_data {
            assert_eq!(left.add(right), expected);
        }
    }

    #[test]
    fn test_big_number_validation_regex() {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^-?\d+(|\.\d+)$").unwrap();
        }
        let test_data = [
            (("0", true)),
            (("-0", true)),
            (("--0", false)),
            (("0.00", true)),
            ((" 1923.8855 ", false)),
            (("147.58374", true)),
            (("-.98", false)),
            (("0.", false)),
        ];
        for (example, expected) in test_data {
            assert_eq!(RE.is_match(example), expected);
        }
    }

    #[test]
    fn test_big_number_from_str() {
        let test_data = [
            (
                BigNumber::from_str("7.0").unwrap(),
                BigNumber {
                    integer: vec![Digits::Seven],
                    decimal: vec![Digits::Zero],
                    sign: Sign::Positive,
                },
            ),
            (
                BigNumber::from_str("-8.004").unwrap(),
                BigNumber {
                    integer: vec![Digits::Eight],
                    decimal: vec![Digits::Zero, Digits::Zero, Digits::Four],
                    sign: Sign::Negative,
                },
            ),
            (
                BigNumber::from_str("-11").unwrap(),
                BigNumber {
                    integer: vec![Digits::One, Digits::One],
                    decimal: Vec::new(),
                    sign: Sign::Negative,
                },
            ),
            (
                BigNumber::from_str("30").unwrap(),
                BigNumber {
                    integer: vec![Digits::Three, Digits::Zero],
                    decimal: Vec::new(),
                    sign: Sign::Positive,
                },
            ),
        ];
        for (example, expected) in test_data {
            assert_eq!(example, expected);
        }
    }
}
