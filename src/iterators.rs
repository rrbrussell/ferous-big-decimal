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

pub struct IntegersByAscendingPower<'a> {
    first: &'a Vec<Digits>,
    second: &'a Vec<Digits>,
    current_power: usize,
    smaller: Smaller,
}

impl<'a> IntegersByAscendingPower<'a> {
    pub fn new(first: &'a Vec<Digits>, second: &'a Vec<Digits>) -> IntegersByAscendingPower<'a> {
        let mut smaller: Smaller = Smaller::Niether;
        if first.len() < second.len() {
            smaller = Smaller::First;
        }
        if first.len() > second.len() {
            smaller = Smaller::Second;
        }
        return IntegersByAscendingPower {
            first,
            second,
            current_power: 0,
            smaller,
        };
    }
}

impl<'a> Iterator for IntegersByAscendingPower<'a> {
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

pub struct IntegersByDescendingPower<'a> {
    first: std::slice::Iter<'a, Digits>,
    second: std::slice::Iter<'a, Digits>,
    current_power: usize,
    smaller: Smaller,
    smallers_length: usize,
}

impl<'a> IntegersByDescendingPower<'a> {
    pub fn new(first: &'a Vec<Digits>, second: &'a Vec<Digits>) -> IntegersByDescendingPower<'a> {
        let mut smaller: Smaller = Smaller::Niether;
        if first.len() < second.len() {
            smaller = Smaller::First;
        }
        if first.len() > second.len() {
            smaller = Smaller::Second;
        }
        return IntegersByDescendingPower {
            first: first.iter(),
            second: second.iter(),
            current_power: max(first.len(), second.len()),
            smaller,
            smallers_length: min(first.len(), second.len()),
        };
    }
}

impl<'a> Iterator for IntegersByDescendingPower<'a> {
    type Item = (Digits, Digits);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_power == 0 {
            return None;
        }

        match self.smaller {
            Smaller::First => {
                let firsts_result: Option<&Digits>;
                let seconds_result: Option<&Digits>;
                let firsts_digit: &Digits;
                let seconds_digit: &Digits;
                if self.current_power > self.smallers_length {
                    firsts_digit = &Digits::Zero;
                } else {
                    firsts_result = self.first.next();
                    firsts_digit = firsts_result.unwrap_or_default();
                }
                seconds_result = self.second.next();
                seconds_digit = seconds_result.unwrap_or_default();
                self.current_power -= 1;
                if seconds_result.is_none() {
                    return None;
                } else {
                    return Some((*firsts_digit, *seconds_digit));
                }
            }
            Smaller::Second => {
                let firsts_result: Option<&Digits>;
                let seconds_result: Option<&Digits>;
                let firsts_digit: &Digits;
                let seconds_digit: &Digits;
                if self.current_power > self.smallers_length {
                    seconds_digit = &Digits::Zero;
                } else {
                    seconds_result = self.second.next();
                    seconds_digit = seconds_result.unwrap_or_default();
                }
                firsts_result = self.first.next();
                firsts_digit = firsts_result.unwrap_or_default();
                self.current_power -= 1;
                if firsts_result.is_none() {
                    return None;
                } else {
                    return Some((*firsts_digit, *seconds_digit));
                }
            }
            Smaller::Niether => {
                let firsts_result: Option<&Digits> = self.first.next();
                let seconds_result: Option<&Digits> = self.second.next();
                let firsts_digit: &Digits = firsts_result.unwrap_or_default();
                let seconds_digit: &Digits = seconds_result.unwrap_or_default();
                if firsts_result.is_none() && seconds_result.is_none() {
                    return None;
                } else {
                    return Some((*firsts_digit, *seconds_digit));
                }
            }
        }
    }
}

pub struct DecimalsByAscendingPower<'a> {
    first: &'a Vec<Digits>,
    second: &'a Vec<Digits>,
    current_power: usize,
    smaller: Smaller,
}

impl<'a> DecimalsByAscendingPower<'a> {
    pub fn new(first: &'a Vec<Digits>, second: &'a Vec<Digits>) -> DecimalsByAscendingPower<'a> {
        let mut smaller = Smaller::Niether;
        if first.len() < second.len() {
            smaller = Smaller::First;
        }
        if first.len() > second.len() {
            smaller = Smaller::Second;
        }
        return DecimalsByAscendingPower {
            first,
            second,
            current_power: 0,
            smaller,
        };
    }
}

impl<'a> Iterator for DecimalsByAscendingPower<'a> {
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

pub struct DecimalsByDescendingPower<'a> {
    first: std::slice::Iter<'a, Digits>,
    second: std::slice::Iter<'a, Digits>,
}

impl<'a> DecimalsByDescendingPower<'a> {
    pub fn new(first: &'a Vec<Digits>, second: &'a Vec<Digits>) -> DecimalsByDescendingPower<'a> {
        return DecimalsByDescendingPower {
            first: first.iter(),
            second: second.iter(),
        };
    }
}

impl<'a> Iterator for DecimalsByDescendingPower<'a> {
    type Item = (Digits, Digits);

    fn next(&mut self) -> Option<Self::Item> {
        let firsts_result: Option<&Digits> = self.first.next();
        let seconds_result: Option<&Digits> = self.second.next();
        let firsts_digit: &Digits = firsts_result.unwrap_or_default();
        let seconds_digit: &Digits = seconds_result.unwrap_or_default();

        if firsts_result.is_none() && seconds_result.is_none() {
            return None;
        } else {
            return Some((*firsts_digit, *seconds_digit));
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
            let mut da: DecimalsByAscendingPower = DecimalsByAscendingPower::new(&first, &second);
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
            let mut da: IntegersByAscendingPower = IntegersByAscendingPower::new(&first, &second);
            while counter < results.len() {
                assert_eq!(da.next(), results[counter]);
                counter += 1;
            }
        }
    }
}
