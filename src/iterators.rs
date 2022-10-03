#![warn(dead_code)]

// External imports
use std::cmp::max;
use std::cmp::min;

// Inter crate imports
use super::Digits;

#[derive(Debug, Default)]
enum Smaller {
    First,
    Second,
    #[default]
    Niether,
}

pub struct IntegersAscending<'a> {
    first: &'a Vec<Digits>,
    second: &'a Vec<Digits>,
    current_power: usize,
    smaller: Smaller,
}

impl<'a> IntegersAscending<'a> {
    pub fn new(first: &'a Vec<Digits>, second: &'a Vec<Digits>) -> IntegersAscending<'a> {
        let mut smaller: Smaller = Smaller::Niether;
        if first.len() < second.len() {
            smaller = Smaller::First;
        }
        if first.len() > second.len() {
            smaller = Smaller::Second;
        }
        return IntegersAscending {
            first,
            second,
            current_power: 0,
            smaller,
        };
    }
}

impl<'a> Iterator for IntegersAscending<'a> {
    type Item = (Digits, Digits);

    fn next(&mut self) -> Option<Self::Item> {
        let firsts_digit: Digits;
        let seconds_digit: Digits;

        if self.first.len() == 0 && self.second.len() == 0 {
            return None;
        }

        match self.smaller {
            Smaller::First => {
                if self.current_power < self.first.len() {
                    firsts_digit = *self
                        .first
                        .get(self.first.len() - self.current_power - 1)
                        .unwrap_or_default();
                } else {
                    firsts_digit = Digits::Zero;
                }
                if self.current_power < self.second.len() {
                    seconds_digit = *self
                        .second
                        .get(self.second.len() - self.current_power - 1)
                        .unwrap_or_default();
                } else {
                    return None;
                }
                self.current_power += 1;
                return Some((firsts_digit, seconds_digit));
            }
            Smaller::Second => {
                if self.current_power < self.second.len() {
                    seconds_digit = *self
                        .second
                        .get(self.second.len() - self.current_power - 1)
                        .unwrap_or_default();
                } else {
                    seconds_digit = Digits::Zero;
                }
                if self.current_power < self.first.len() {
                    firsts_digit = *self
                        .first
                        .get(self.first.len() - self.current_power - 1)
                        .unwrap_or_default();
                } else {
                    return None;
                }
                self.current_power += 1;
                return Some((firsts_digit, seconds_digit));
            }
            Smaller::Niether => {
                if self.current_power < self.first.len() {
                    firsts_digit = *self
                        .first
                        .get(self.first.len() - self.current_power - 1)
                        .unwrap_or_default();
                    seconds_digit = *self
                        .second
                        .get(self.second.len() - self.current_power - 1)
                        .unwrap_or_default();
                    self.current_power += 1;
                    return Some((firsts_digit, seconds_digit));
                } else {
                    return None;
                }
            }
        }
    }
}

pub struct DecimalsAscending<'a> {
    first: &'a Vec<Digits>,
    second: &'a Vec<Digits>,
    current_power: usize,
    smaller: Smaller,
}

impl<'a> DecimalsAscending<'a> {
    pub fn new(first: &'a Vec<Digits>, second: &'a Vec<Digits>) -> DecimalsAscending<'a> {
        let mut smaller = Smaller::Niether;
        if first.len() < second.len() {
            smaller = Smaller::First;
        }
        if first.len() > second.len() {
            smaller = Smaller::Second;
        }
        return DecimalsAscending {
            first,
            second,
            current_power: 0,
            smaller,
        };
    }
}

impl<'a> Iterator for DecimalsAscending<'a> {
    type Item = (Digits, Digits);

    fn next(&mut self) -> Option<Self::Item> {
        let firsts_digit: Digits;
        let seconds_digit: Digits;
        // lms stands for largest minus smallest.
        let lms: usize =
            max(self.first.len(), self.second.len()) - min(self.first.len(), self.second.len());

        if self.first.len() == 0 && self.second.len() == 0 {
            return None;
        }
        match self.smaller {
            Smaller::First => {
                if self.current_power < self.second.len() {
                    seconds_digit = *self
                        .second
                        .get(self.second.len() - self.current_power - 1)
                        .unwrap_or_default();
                } else {
                    return None;
                }
                if self.current_power < lms {
                    firsts_digit = Digits::Zero;
                } else {
                    firsts_digit = *self
                        .first
                        .get(self.first.len() - (self.current_power - lms) - 1)
                        .unwrap_or_default();
                }
                self.current_power += 1;
                return Some((firsts_digit, seconds_digit));
            }
            Smaller::Second => {
                if self.current_power < self.first.len() {
                    firsts_digit = *self
                        .first
                        .get(self.first.len() - self.current_power - 1)
                        .unwrap_or_default();
                } else {
                    return None;
                }
                if self.current_power < lms {
                    seconds_digit = Digits::Zero;
                } else {
                    seconds_digit = *self
                        .second
                        .get(self.second.len() - (self.current_power - lms) - 1)
                        .unwrap_or_default();
                }
                self.current_power += 1;
                return Some((firsts_digit, seconds_digit));
            }
            Smaller::Niether => {
                if self.current_power < self.first.len() {
                    firsts_digit = *self
                        .first
                        .get(self.first.len() - self.current_power - 1)
                        .unwrap_or_default();
                    seconds_digit = *self
                        .second
                        .get(self.second.len() - self.current_power - 1)
                        .unwrap_or_default();
                    self.current_power += 1;
                    return Some((firsts_digit, seconds_digit));
                } else {
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_decimals_ascending() {
        let test_data = [
            (Vec::new(), Vec::new(), vec![None]),
            (
                vec![Digits::Three, Digits::Six],
                vec![Digits::Seven, Digits::One],
                vec![
                    Some((Digits::Six, Digits::One)),
                    Some((Digits::Three, Digits::Seven)),
                    None,
                ],
            ),
            (
                vec![Digits::Three],
                vec![Digits::Seven, Digits::One],
                vec![
                    Some((Digits::Zero, Digits::One)),
                    Some((Digits::Three, Digits::Seven)),
                    None,
                ],
            ),
            (
                vec![Digits::Three, Digits::Six],
                vec![Digits::Seven],
                vec![
                    Some((Digits::Six, Digits::Zero)),
                    Some((Digits::Three, Digits::Seven)),
                    None,
                ],
            ),
        ];
        for (first, second, results) in test_data {
            let mut counter: usize = 0;
            let mut da: DecimalsAscending = DecimalsAscending::new(&first, &second);
            while counter < results.len() {
                assert_eq!(da.next(), results[counter]);
                counter += 1;
            }
        }
    }

    #[test]
    fn test_integer_ascending() {
        let test_data = [
            (
                vec![Digits::Three, Digits::Six],
                vec![Digits::Seven, Digits::One],
                vec![
                    Some((Digits::Six, Digits::One)),
                    Some((Digits::Three, Digits::Seven)),
                    None,
                ],
            ),
            (
                vec![Digits::Three],
                vec![Digits::Seven, Digits::One],
                vec![
                    Some((Digits::Three, Digits::One)),
                    Some((Digits::Zero, Digits::Seven)),
                    None,
                ],
            ),
            (
                vec![Digits::Seven, Digits::One],
                vec![Digits::Three],
                vec![
                    Some((Digits::One, Digits::Three)),
                    Some((Digits::Seven, Digits::Zero)),
                    None,
                ],
            ),
            (Vec::new(), Vec::new(), vec![None]),
        ];
        for (first, second, results) in test_data {
            let mut counter: usize = 0;
            let mut da: IntegersAscending = IntegersAscending::new(&first, &second);
            while counter < results.len() {
                assert_eq!(da.next(), results[counter]);
                counter += 1;
            }
        }
    }
}
