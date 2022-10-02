#![warn(dead_code)]

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
                let x = self.second.get(self.second.len() - self.current_power - 1);
                if x == None {
                    return None;
                }
                seconds_digit = *x.unwrap();
                self.current_power += 1;
                return Some((firsts_digit, seconds_digit));
            }
            Smaller::Second => {
                if self.current_power < self.second.len() {
                    seconds_digit = *self
                        .second
                        .get(self.first.len() - self.current_power - 1)
                        .unwrap_or_default();
                } else {
                    seconds_digit = Digits::Zero;
                }
                let x = self.first.get(self.first.len() - self.current_power - 1);
                if x == None {
                    return None;
                }
                firsts_digit = *x.unwrap();
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
