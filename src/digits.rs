#![warn(dead_code)]
use super::MathErrors;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Digits {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digits {
    /// Adds rhs to self with any carry over ammount returned.
    pub const fn addition(self: Self, rhs: Digits) -> (Digits, Digits) {
        return ADDITION_MATRIX[self.to_usize()][rhs.to_usize()];
    }

    /// Subtracts rhs from self with any borrowed ammount returned.
    pub const fn subtract(self: Self, rhs: Digits) -> (Digits, Digits) {
        return SUBTRACTION_MATRIX[self.to_usize()][rhs.to_usize()];
    }

    /// Multiplies self by rhs with any carry over amount returned.
    pub const fn multiply(self: Self, rhs: Digits) -> (Digits, Digits) {
        return MULTIPLICATION_MATRIX[self.to_usize()][rhs.to_usize()];
    }

    /// Divides self by rhs with any remainder returned.
    ///
    /// [MathErrors::DivisionByZero] is returned if self or rhs are 0.
    pub const fn divide(self: Self, rhs: Digits) -> Result<(Digits, Digits), MathErrors> {
        return DIVISION_MATRIX[self.to_usize()][rhs.to_usize()];
    }

    pub const fn to_usize(self: Self) -> usize {
        match self {
            Digits::Zero => return 0,
            Digits::One => return 1,
            Digits::Two => return 2,
            Digits::Three => return 3,
            Digits::Four => return 4,
            Digits::Five => return 5,
            Digits::Six => return 6,
            Digits::Seven => return 7,
            Digits::Eight => return 8,
            Digits::Nine => return 9,
        }
    }

    pub const fn to_char(self: Self) -> char {
        match self {
            Digits::Zero => {
                return '0';
            }
            Digits::One => {
                return '1';
            }
            Digits::Two => {
                return '2';
            }
            Digits::Three => {
                return '3';
            }
            Digits::Four => {
                return '4';
            }
            Digits::Five => {
                return '5';
            }
            Digits::Six => {
                return '6';
            }
            Digits::Seven => {
                return '7';
            }
            Digits::Eight => {
                return '8';
            }
            Digits::Nine => {
                return '9';
            }
        }
    }
}

impl From<u128> for Digits {
    fn from(input: u128) -> Self {
        if input == 0 {
            return Digits::Zero;
        }
        if input == 1 {
            return Digits::One;
        }
        if input == 2 {
            return Digits::Two;
        }
        if input == 3 {
            return Digits::Three;
        }
        if input == 4 {
            return Digits::Four;
        }
        if input == 5 {
            return Digits::Five;
        }
        if input == 6 {
            return Digits::Six;
        }
        if input == 7 {
            return Digits::Seven;
        }
        if input == 8 {
            return Digits::Eight;
        }
        return Digits::Nine;
    }
}

const ADDITION_MATRIX: [[(Digits, Digits); 10]; 10] = [
    [
        (Digits::Zero, Digits::Zero),
        (Digits::One, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Nine, Digits::Zero),
    ],
    [
        (Digits::One, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Nine, Digits::Zero),
        (Digits::Zero, Digits::One),
    ],
    [
        (Digits::Two, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Nine, Digits::Zero),
        (Digits::Zero, Digits::One),
        (Digits::One, Digits::One),
    ],
    [
        (Digits::Three, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Nine, Digits::Zero),
        (Digits::Zero, Digits::One),
        (Digits::One, Digits::One),
        (Digits::Two, Digits::One),
    ],
    [
        (Digits::Four, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Nine, Digits::Zero),
        (Digits::Zero, Digits::One),
        (Digits::One, Digits::One),
        (Digits::Two, Digits::One),
        (Digits::Three, Digits::One),
    ],
    [
        (Digits::Five, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Nine, Digits::Zero),
        (Digits::Zero, Digits::One),
        (Digits::One, Digits::One),
        (Digits::Two, Digits::One),
        (Digits::Three, Digits::One),
        (Digits::Four, Digits::One),
    ],
    [
        (Digits::Six, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Nine, Digits::Zero),
        (Digits::Zero, Digits::One),
        (Digits::One, Digits::One),
        (Digits::Two, Digits::One),
        (Digits::Three, Digits::One),
        (Digits::Four, Digits::One),
        (Digits::Five, Digits::One),
    ],
    [
        (Digits::Seven, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Nine, Digits::Zero),
        (Digits::Zero, Digits::One),
        (Digits::One, Digits::One),
        (Digits::Two, Digits::One),
        (Digits::Three, Digits::One),
        (Digits::Four, Digits::One),
        (Digits::Five, Digits::One),
        (Digits::Six, Digits::One),
    ],
    [
        (Digits::Eight, Digits::Zero),
        (Digits::Nine, Digits::Zero),
        (Digits::Zero, Digits::One),
        (Digits::One, Digits::One),
        (Digits::Two, Digits::One),
        (Digits::Three, Digits::One),
        (Digits::Four, Digits::One),
        (Digits::Five, Digits::One),
        (Digits::Six, Digits::One),
        (Digits::Seven, Digits::One),
    ],
    [
        (Digits::Nine, Digits::Zero),
        (Digits::Zero, Digits::One),
        (Digits::One, Digits::One),
        (Digits::Two, Digits::One),
        (Digits::Three, Digits::One),
        (Digits::Four, Digits::One),
        (Digits::Five, Digits::One),
        (Digits::Six, Digits::One),
        (Digits::Seven, Digits::One),
        (Digits::Eight, Digits::One),
    ],
];

const SUBTRACTION_MATRIX: [[(Digits, Digits); 10]; 10] = [
    [
        (Digits::Zero, Digits::Zero),
        (Digits::Nine, Digits::One),
        (Digits::Eight, Digits::One),
        (Digits::Seven, Digits::One),
        (Digits::Six, Digits::One),
        (Digits::Five, Digits::One),
        (Digits::Four, Digits::One),
        (Digits::Three, Digits::One),
        (Digits::Two, Digits::One),
        (Digits::One, Digits::One),
    ],
    [
        (Digits::One, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Nine, Digits::One),
        (Digits::Eight, Digits::One),
        (Digits::Seven, Digits::One),
        (Digits::Six, Digits::One),
        (Digits::Five, Digits::One),
        (Digits::Four, Digits::One),
        (Digits::Three, Digits::One),
        (Digits::Two, Digits::One),
    ],
    [
        (Digits::Two, Digits::Zero),
        (Digits::One, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Nine, Digits::One),
        (Digits::Eight, Digits::One),
        (Digits::Seven, Digits::One),
        (Digits::Six, Digits::One),
        (Digits::Five, Digits::One),
        (Digits::Four, Digits::One),
        (Digits::Three, Digits::One),
    ],
    [
        (Digits::Three, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::One, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Nine, Digits::One),
        (Digits::Eight, Digits::One),
        (Digits::Seven, Digits::One),
        (Digits::Six, Digits::One),
        (Digits::Five, Digits::One),
        (Digits::Four, Digits::One),
    ],
    [
        (Digits::Four, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::One, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Nine, Digits::One),
        (Digits::Eight, Digits::One),
        (Digits::Seven, Digits::One),
        (Digits::Six, Digits::One),
        (Digits::Five, Digits::One),
    ],
    [
        (Digits::Five, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::One, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Nine, Digits::One),
        (Digits::Eight, Digits::One),
        (Digits::Seven, Digits::One),
        (Digits::Six, Digits::One),
    ],
    [
        (Digits::Six, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::One, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Nine, Digits::One),
        (Digits::Eight, Digits::One),
        (Digits::Seven, Digits::One),
    ],
    [
        (Digits::Seven, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::One, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Nine, Digits::One),
        (Digits::Eight, Digits::One),
    ],
    [
        (Digits::Eight, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::One, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Nine, Digits::One),
    ],
    [
        (Digits::Nine, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::One, Digits::Zero),
        (Digits::Zero, Digits::Zero),
    ],
];

const MULTIPLICATION_MATRIX: [[(Digits, Digits); 10]; 10] = [
    [
        (Digits::Zero, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Zero, Digits::Zero),
        (Digits::Zero, Digits::Zero),
    ],
    [
        (Digits::Zero, Digits::Zero),
        (Digits::One, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Nine, Digits::Zero),
    ],
    [
        (Digits::Zero, Digits::Zero),
        (Digits::Two, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Zero, Digits::One),
        (Digits::Two, Digits::One),
        (Digits::Four, Digits::One),
        (Digits::Six, Digits::One),
        (Digits::Eight, Digits::One),
    ],
    [
        (Digits::Zero, Digits::Zero),
        (Digits::Three, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Nine, Digits::Zero),
        (Digits::Two, Digits::One),
        (Digits::Five, Digits::One),
        (Digits::Eight, Digits::One),
        (Digits::One, Digits::Two),
        (Digits::Four, Digits::Two),
        (Digits::Seven, Digits::Two),
    ],
    [
        (Digits::Zero, Digits::Zero),
        (Digits::Four, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Two, Digits::One),
        (Digits::Six, Digits::One),
        (Digits::Zero, Digits::Two),
        (Digits::Four, Digits::Two),
        (Digits::Eight, Digits::Two),
        (Digits::Two, Digits::Three),
        (Digits::Six, Digits::Three),
    ],
    [
        (Digits::Zero, Digits::Zero),
        (Digits::Five, Digits::Zero),
        (Digits::Zero, Digits::One),
        (Digits::Five, Digits::One),
        (Digits::Zero, Digits::Two),
        (Digits::Five, Digits::Two),
        (Digits::Zero, Digits::Three),
        (Digits::Five, Digits::Three),
        (Digits::Zero, Digits::Four),
        (Digits::Five, Digits::Four),
    ],
    [
        (Digits::Zero, Digits::Zero),
        (Digits::Six, Digits::Zero),
        (Digits::Two, Digits::One),
        (Digits::Eight, Digits::One),
        (Digits::Four, Digits::Two),
        (Digits::Zero, Digits::Three),
        (Digits::Six, Digits::Three),
        (Digits::Two, Digits::Four),
        (Digits::Eight, Digits::Four),
        (Digits::Four, Digits::Five),
    ],
    [
        (Digits::Zero, Digits::Zero),
        (Digits::Seven, Digits::Zero),
        (Digits::Four, Digits::One),
        (Digits::One, Digits::Two),
        (Digits::Eight, Digits::Two),
        (Digits::Five, Digits::Three),
        (Digits::Two, Digits::Four),
        (Digits::Nine, Digits::Four),
        (Digits::Six, Digits::Five),
        (Digits::Three, Digits::Six),
    ],
    [
        (Digits::Zero, Digits::Zero),
        (Digits::Eight, Digits::Zero),
        (Digits::Six, Digits::One),
        (Digits::Four, Digits::Two),
        (Digits::Two, Digits::Three),
        (Digits::Zero, Digits::Four),
        (Digits::Eight, Digits::Four),
        (Digits::Six, Digits::Five),
        (Digits::Four, Digits::Six),
        (Digits::Two, Digits::Seven),
    ],
    [
        (Digits::Zero, Digits::Zero),
        (Digits::Nine, Digits::Zero),
        (Digits::Eight, Digits::One),
        (Digits::Seven, Digits::Two),
        (Digits::Six, Digits::Three),
        (Digits::Five, Digits::Four),
        (Digits::Four, Digits::Five),
        (Digits::Three, Digits::Six),
        (Digits::Two, Digits::Seven),
        (Digits::One, Digits::Eight),
    ],
];

const DIVISION_MATRIX: [[Result<(Digits, Digits), MathErrors>; 10]; 10] = [
    [
        Err(MathErrors::DivisionByZero),
        Ok((Digits::Zero, Digits::One)),
        Ok((Digits::Zero, Digits::Two)),
        Ok((Digits::Zero, Digits::Three)),
        Ok((Digits::Zero, Digits::Four)),
        Ok((Digits::Zero, Digits::Five)),
        Ok((Digits::Zero, Digits::Six)),
        Ok((Digits::Zero, Digits::Seven)),
        Ok((Digits::Zero, Digits::Eight)),
        Ok((Digits::Zero, Digits::Nine)),
    ],
    [
        Err(MathErrors::DivisionByZero),
        Ok((Digits::One, Digits::Zero)),
        Ok((Digits::Zero, Digits::One)),
        Ok((Digits::Zero, Digits::One)),
        Ok((Digits::Zero, Digits::One)),
        Ok((Digits::Zero, Digits::One)),
        Ok((Digits::Zero, Digits::One)),
        Ok((Digits::Zero, Digits::One)),
        Ok((Digits::Zero, Digits::One)),
        Ok((Digits::Zero, Digits::One)),
    ],
    [
        Err(MathErrors::DivisionByZero),
        Ok((Digits::Two, Digits::Zero)),
        Ok((Digits::One, Digits::Zero)),
        Ok((Digits::Zero, Digits::Two)),
        Ok((Digits::Zero, Digits::Two)),
        Ok((Digits::Zero, Digits::Two)),
        Ok((Digits::Zero, Digits::Two)),
        Ok((Digits::Zero, Digits::Two)),
        Ok((Digits::Zero, Digits::Two)),
        Ok((Digits::Zero, Digits::Two)),
    ],
    [
        Err(MathErrors::DivisionByZero),
        Ok((Digits::Three, Digits::Zero)),
        Ok((Digits::One, Digits::One)),
        Ok((Digits::One, Digits::Zero)),
        Ok((Digits::Zero, Digits::Three)),
        Ok((Digits::Zero, Digits::Three)),
        Ok((Digits::Zero, Digits::Three)),
        Ok((Digits::Zero, Digits::Three)),
        Ok((Digits::Zero, Digits::Three)),
        Ok((Digits::Zero, Digits::Three)),
    ],
    [
        Err(MathErrors::DivisionByZero),
        Ok((Digits::Four, Digits::Zero)),
        Ok((Digits::Two, Digits::Zero)),
        Ok((Digits::One, Digits::One)),
        Ok((Digits::One, Digits::Zero)),
        Ok((Digits::Zero, Digits::Four)),
        Ok((Digits::Zero, Digits::Four)),
        Ok((Digits::Zero, Digits::Four)),
        Ok((Digits::Zero, Digits::Four)),
        Ok((Digits::Zero, Digits::Four)),
    ],
    [
        Err(MathErrors::DivisionByZero),
        Ok((Digits::Five, Digits::Zero)),
        Ok((Digits::Two, Digits::One)),
        Ok((Digits::One, Digits::Two)),
        Ok((Digits::One, Digits::One)),
        Ok((Digits::One, Digits::Zero)),
        Ok((Digits::Zero, Digits::Five)),
        Ok((Digits::Zero, Digits::Five)),
        Ok((Digits::Zero, Digits::Five)),
        Ok((Digits::Zero, Digits::Five)),
    ],
    [
        Err(MathErrors::DivisionByZero),
        Ok((Digits::Six, Digits::Zero)),
        Ok((Digits::Three, Digits::Zero)),
        Ok((Digits::Two, Digits::Zero)),
        Ok((Digits::One, Digits::Two)),
        Ok((Digits::One, Digits::One)),
        Ok((Digits::One, Digits::Zero)),
        Ok((Digits::Zero, Digits::Six)),
        Ok((Digits::Zero, Digits::Six)),
        Ok((Digits::Zero, Digits::Six)),
    ],
    [
        Err(MathErrors::DivisionByZero),
        Ok((Digits::Seven, Digits::Zero)),
        Ok((Digits::Three, Digits::One)),
        Ok((Digits::Two, Digits::One)),
        Ok((Digits::One, Digits::Three)),
        Ok((Digits::One, Digits::Two)),
        Ok((Digits::One, Digits::One)),
        Ok((Digits::One, Digits::Zero)),
        Ok((Digits::Zero, Digits::Seven)),
        Ok((Digits::Zero, Digits::Seven)),
    ],
    [
        Err(MathErrors::DivisionByZero),
        Ok((Digits::Eight, Digits::Zero)),
        Ok((Digits::Four, Digits::Zero)),
        Ok((Digits::Two, Digits::Two)),
        Ok((Digits::Two, Digits::Zero)),
        Ok((Digits::One, Digits::Three)),
        Ok((Digits::One, Digits::Two)),
        Ok((Digits::One, Digits::One)),
        Ok((Digits::One, Digits::Zero)),
        Ok((Digits::Zero, Digits::Eight)),
    ],
    [
        Err(MathErrors::DivisionByZero),
        Ok((Digits::Nine, Digits::Zero)),
        Ok((Digits::Four, Digits::One)),
        Ok((Digits::Three, Digits::Zero)),
        Ok((Digits::Two, Digits::One)),
        Ok((Digits::One, Digits::Four)),
        Ok((Digits::One, Digits::Three)),
        Ok((Digits::One, Digits::Two)),
        Ok((Digits::One, Digits::One)),
        Ok((Digits::One, Digits::Zero)),
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_addition() {
        let test_data = [
            (Digits::Five, Digits::Five, (Digits::Zero, Digits::One)),
            (Digits::Two, Digits::Four, (Digits::Six, Digits::Zero)),
            (Digits::Two, Digits::Two, (Digits::Four, Digits::Zero)),
            (Digits::Six, Digits::Four, (Digits::Zero, Digits::One)),
            (Digits::Seven, Digits::Six, (Digits::Three, Digits::One)),
            (Digits::Six, Digits::Zero, (Digits::Six, Digits::Zero)),
            (Digits::Two, Digits::Six, (Digits::Eight, Digits::Zero)),
            (Digits::Seven, Digits::Eight, (Digits::Five, Digits::One)),
            (Digits::Zero, Digits::Zero, (Digits::Zero, Digits::Zero)),
            (Digits::Four, Digits::Zero, (Digits::Four, Digits::Zero)),
        ];

        for (left, right, expected) in test_data {
            assert_eq!(left.addition(right), expected);
        }
    }

    #[test]
    fn test_digit_subtraction() {
        let test_data = [
            (Digits::Nine, Digits::Six, (Digits::Three, Digits::Zero)),
            (Digits::Six, Digits::Zero, (Digits::Six, Digits::Zero)),
            (Digits::One, Digits::Seven, (Digits::Four, Digits::One)),
            (Digits::Zero, Digits::Five, (Digits::Five, Digits::One)),
            (Digits::Five, Digits::Four, (Digits::One, Digits::Zero)),
            (Digits::Seven, Digits::Eight, (Digits::Nine, Digits::One)),
            (Digits::Six, Digits::Zero, (Digits::Six, Digits::Zero)),
            (Digits::Zero, Digits::Nine, (Digits::One, Digits::One)),
            (Digits::Zero, Digits::Seven, (Digits::Three, Digits::One)),
            (Digits::Nine, Digits::Two, (Digits::Seven, Digits::Zero)),
        ];
        for (left, right, expected) in test_data {
            assert_eq!(left.subtract(right), expected);
        }
    }

    #[test]
    fn test_digit_multiplication() {
        let test_data = [
            (Digits::Nine, Digits::Two, (Digits::Eight, Digits::One)),
            (Digits::Eight, Digits::Nine, (Digits::Two, Digits::Seven)),
            (Digits::One, Digits::Zero, (Digits::Zero, Digits::Zero)),
            (Digits::Six, Digits::Six, (Digits::Six, Digits::Three)),
            (Digits::Eight, Digits::Two, (Digits::Six, Digits::One)),
            (Digits::Zero, Digits::Nine, (Digits::Zero, Digits::Zero)),
            (Digits::Seven, Digits::Zero, (Digits::Zero, Digits::Zero)),
            (Digits::Two, Digits::Eight, (Digits::Six, Digits::One)),
            (Digits::Three, Digits::Four, (Digits::Two, Digits::One)),
            (Digits::Eight, Digits::Seven, (Digits::Six, Digits::Five)),
        ];
        for (left, right, expected) in test_data {
            assert_eq!(left.multiply(right), expected);
        }
    }

    #[test]
    fn test_digit_division() {
        let test_data = [
            (Digits::Nine, Digits::Five, Ok((Digits::One, Digits::Four))),
            (Digits::Six, Digits::Five, Ok((Digits::One, Digits::One))),
            (
                Digits::Seven,
                Digits::Nine,
                Ok((Digits::Zero, Digits::Seven)),
            ),
            (Digits::Zero, Digits::Six, Ok((Digits::Zero, Digits::Six))),
            (Digits::Six, Digits::Six, Ok((Digits::One, Digits::Zero))),
            (
                Digits::Four,
                Digits::Eight,
                Ok((Digits::Zero, Digits::Four)),
            ),
            (Digits::Five, Digits::Four, Ok((Digits::One, Digits::One))),
            (Digits::Nine, Digits::Zero, Err(MathErrors::DivisionByZero)),
            (Digits::Eight, Digits::Four, Ok((Digits::Two, Digits::Zero))),
            (Digits::One, Digits::Two, Ok((Digits::Zero, Digits::One))),
        ];
        for (left, right, expected) in test_data {
            assert_eq!(left.divide(right), expected);
        }
    }
}
