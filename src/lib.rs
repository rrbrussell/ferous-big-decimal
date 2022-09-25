// Temporary during development.
#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
enum Digits {
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
    /// Adds rhs to self with any carry over ammount returned as an [Option].
    const fn add(self: Self, rhs: Digits) -> (Digits, Option<Digits>) {
        return ADDITION_MATRIX[self.to_usize()][rhs.to_usize()];
    }

    /// Subtracts rhs from self with any borrowed ammount returned as an [Option].
    const fn subtract(self: Self, rhs: Digits) -> (Digits, Option<Digits>) {
        return SUBTRACTION_MATRIX[self.to_usize()][rhs.to_usize()];
    }

    /// Multiplies self by rhs with any carry over amount returned as an [Option].
    const fn multiply(self: Self, rhs: Digits) -> (Digits, Option<Digits>) {
        return MULTIPLICATION_MATRIX[self.to_usize()][rhs.to_usize()];
    }

    const fn to_usize(self: Self) -> usize {
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
}

const ADDITION_MATRIX: [[(Digits, Option<Digits>); 10]; 10] = [
    [
        (Digits::Zero, None),
        (Digits::One, None),
        (Digits::Two, None),
        (Digits::Three, None),
        (Digits::Four, None),
        (Digits::Five, None),
        (Digits::Six, None),
        (Digits::Seven, None),
        (Digits::Eight, None),
        (Digits::Nine, None),
    ],
    [
        (Digits::One, None),
        (Digits::Two, None),
        (Digits::Three, None),
        (Digits::Four, None),
        (Digits::Five, None),
        (Digits::Six, None),
        (Digits::Seven, None),
        (Digits::Eight, None),
        (Digits::Nine, None),
        (Digits::Zero, Some(Digits::One)),
    ],
    [
        (Digits::Two, None),
        (Digits::Three, None),
        (Digits::Four, None),
        (Digits::Five, None),
        (Digits::Six, None),
        (Digits::Seven, None),
        (Digits::Eight, None),
        (Digits::Nine, None),
        (Digits::Zero, Some(Digits::One)),
        (Digits::One, Some(Digits::One)),
    ],
    [
        (Digits::Three, None),
        (Digits::Four, None),
        (Digits::Five, None),
        (Digits::Six, None),
        (Digits::Seven, None),
        (Digits::Eight, None),
        (Digits::Nine, None),
        (Digits::Zero, Some(Digits::One)),
        (Digits::One, Some(Digits::One)),
        (Digits::Two, Some(Digits::One)),
    ],
    [
        (Digits::Four, None),
        (Digits::Five, None),
        (Digits::Six, None),
        (Digits::Seven, None),
        (Digits::Eight, None),
        (Digits::Nine, None),
        (Digits::Zero, Some(Digits::One)),
        (Digits::One, Some(Digits::One)),
        (Digits::Two, Some(Digits::One)),
        (Digits::Three, Some(Digits::One)),
    ],
    [
        (Digits::Five, None),
        (Digits::Six, None),
        (Digits::Seven, None),
        (Digits::Eight, None),
        (Digits::Nine, None),
        (Digits::Zero, Some(Digits::One)),
        (Digits::One, Some(Digits::One)),
        (Digits::Two, Some(Digits::One)),
        (Digits::Three, Some(Digits::One)),
        (Digits::Four, Some(Digits::One)),
    ],
    [
        (Digits::Six, None),
        (Digits::Seven, None),
        (Digits::Eight, None),
        (Digits::Nine, None),
        (Digits::Zero, Some(Digits::One)),
        (Digits::One, Some(Digits::One)),
        (Digits::Two, Some(Digits::One)),
        (Digits::Three, Some(Digits::One)),
        (Digits::Four, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
    ],
    [
        (Digits::Seven, None),
        (Digits::Eight, None),
        (Digits::Nine, None),
        (Digits::Zero, Some(Digits::One)),
        (Digits::One, Some(Digits::One)),
        (Digits::Two, Some(Digits::One)),
        (Digits::Three, Some(Digits::One)),
        (Digits::Four, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
    ],
    [
        (Digits::Eight, None),
        (Digits::Nine, None),
        (Digits::Zero, Some(Digits::One)),
        (Digits::One, Some(Digits::One)),
        (Digits::Two, Some(Digits::One)),
        (Digits::Three, Some(Digits::One)),
        (Digits::Four, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
        (Digits::Seven, Some(Digits::One)),
    ],
    [
        (Digits::Nine, None),
        (Digits::Zero, Some(Digits::One)),
        (Digits::One, Some(Digits::One)),
        (Digits::Two, Some(Digits::One)),
        (Digits::Three, Some(Digits::One)),
        (Digits::Four, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
        (Digits::Seven, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
    ],
];

const SUBTRACTION_MATRIX: [[(Digits, Option<Digits>); 10]; 10] = [
    [
        (Digits::Zero, None),
        (Digits::Nine, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
        (Digits::Seven, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
        (Digits::Four, Some(Digits::One)),
        (Digits::Three, Some(Digits::One)),
        (Digits::Two, Some(Digits::One)),
        (Digits::One, Some(Digits::One)),
    ],
    [
        (Digits::One, None),
        (Digits::Zero, None),
        (Digits::Nine, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
        (Digits::Seven, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
        (Digits::Four, Some(Digits::One)),
        (Digits::Three, Some(Digits::One)),
        (Digits::Two, Some(Digits::One)),
    ],
    [
        (Digits::Two, None),
        (Digits::One, None),
        (Digits::Zero, None),
        (Digits::Nine, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
        (Digits::Seven, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
        (Digits::Four, Some(Digits::One)),
        (Digits::Three, Some(Digits::One)),
    ],
    [
        (Digits::Three, None),
        (Digits::Two, None),
        (Digits::One, None),
        (Digits::Zero, None),
        (Digits::Nine, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
        (Digits::Seven, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
        (Digits::Four, Some(Digits::One)),
    ],
    [
        (Digits::Four, None),
        (Digits::Three, None),
        (Digits::Two, None),
        (Digits::One, None),
        (Digits::Zero, None),
        (Digits::Nine, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
        (Digits::Seven, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
    ],
    [
        (Digits::Five, None),
        (Digits::Four, None),
        (Digits::Three, None),
        (Digits::Two, None),
        (Digits::One, None),
        (Digits::Zero, None),
        (Digits::Nine, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
        (Digits::Seven, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
    ],
    [
        (Digits::Six, None),
        (Digits::Five, None),
        (Digits::Four, None),
        (Digits::Three, None),
        (Digits::Two, None),
        (Digits::One, None),
        (Digits::Zero, None),
        (Digits::Nine, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
        (Digits::Seven, Some(Digits::One)),
    ],
    [
        (Digits::Seven, None),
        (Digits::Six, None),
        (Digits::Five, None),
        (Digits::Four, None),
        (Digits::Three, None),
        (Digits::Two, None),
        (Digits::One, None),
        (Digits::Zero, None),
        (Digits::Nine, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
    ],
    [
        (Digits::Eight, None),
        (Digits::Seven, None),
        (Digits::Six, None),
        (Digits::Five, None),
        (Digits::Four, None),
        (Digits::Three, None),
        (Digits::Two, None),
        (Digits::One, None),
        (Digits::Zero, None),
        (Digits::Nine, Some(Digits::One)),
    ],
    [
        (Digits::Nine, None),
        (Digits::Eight, None),
        (Digits::Seven, None),
        (Digits::Six, None),
        (Digits::Five, None),
        (Digits::Four, None),
        (Digits::Three, None),
        (Digits::Two, None),
        (Digits::One, None),
        (Digits::Zero, None),
    ],
];

const MULTIPLICATION_MATRIX: [[(Digits, Option<Digits>); 10]; 10] = [
    [
        (Digits::Zero, None),
        (Digits::Zero, None),
        (Digits::Zero, None),
        (Digits::Zero, None),
        (Digits::Zero, None),
        (Digits::Zero, None),
        (Digits::Zero, None),
        (Digits::Zero, None),
        (Digits::Zero, None),
        (Digits::Zero, None),
    ],
    [
        (Digits::Zero, None),
        (Digits::One, None),
        (Digits::Two, None),
        (Digits::Three, None),
        (Digits::Four, None),
        (Digits::Five, None),
        (Digits::Six, None),
        (Digits::Seven, None),
        (Digits::Eight, None),
        (Digits::Nine, None),
    ],
    [
        (Digits::Zero, None),
        (Digits::Two, None),
        (Digits::Four, None),
        (Digits::Six, None),
        (Digits::Eight, None),
        (Digits::Zero, Some(Digits::One)),
        (Digits::Two, Some(Digits::One)),
        (Digits::Four, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
    ],
    [
        (Digits::Zero, None),
        (Digits::Three, None),
        (Digits::Six, None),
        (Digits::Nine, None),
        (Digits::Two, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
        (Digits::One, Some(Digits::Two)),
        (Digits::Four, Some(Digits::Two)),
        (Digits::Seven, Some(Digits::Two)),
    ],
    [
        (Digits::Zero, None),
        (Digits::Four, None),
        (Digits::Eight, None),
        (Digits::Two, Some(Digits::One)),
        (Digits::Six, Some(Digits::One)),
        (Digits::Zero, Some(Digits::Two)),
        (Digits::Four, Some(Digits::Two)),
        (Digits::Eight, Some(Digits::Two)),
        (Digits::Two, Some(Digits::Three)),
        (Digits::Six, Some(Digits::Three)),
    ],
    [
        (Digits::Zero, None),
        (Digits::Five, None),
        (Digits::Zero, Some(Digits::One)),
        (Digits::Five, Some(Digits::One)),
        (Digits::Zero, Some(Digits::Two)),
        (Digits::Five, Some(Digits::Two)),
        (Digits::Zero, Some(Digits::Three)),
        (Digits::Five, Some(Digits::Three)),
        (Digits::Zero, Some(Digits::Four)),
        (Digits::Five, Some(Digits::Four)),
    ],
    [
        (Digits::Zero, None),
        (Digits::Six, None),
        (Digits::Two, Some(Digits::One)),
        (Digits::Eight, Some(Digits::One)),
        (Digits::Four, Some(Digits::Two)),
        (Digits::Zero, Some(Digits::Three)),
        (Digits::Six, Some(Digits::Three)),
        (Digits::Two, Some(Digits::Four)),
        (Digits::Eight, Some(Digits::Four)),
        (Digits::Four, Some(Digits::Five)),
    ],
    [
        (Digits::Zero, None),
        (Digits::Seven, None),
        (Digits::Four, Some(Digits::One)),
        (Digits::One, Some(Digits::Two)),
        (Digits::Eight, Some(Digits::Two)),
        (Digits::Five, Some(Digits::Three)),
        (Digits::Two, Some(Digits::Four)),
        (Digits::Nine, Some(Digits::Four)),
        (Digits::Six, Some(Digits::Five)),
        (Digits::Three, Some(Digits::Six)),
    ],
    [
        (Digits::Zero, None),
        (Digits::Eight, None),
        (Digits::Six, Some(Digits::One)),
        (Digits::Four, Some(Digits::Two)),
        (Digits::Two, Some(Digits::Three)),
        (Digits::Zero, Some(Digits::Four)),
        (Digits::Eight, Some(Digits::Four)),
        (Digits::Six, Some(Digits::Five)),
        (Digits::Four, Some(Digits::Six)),
        (Digits::Two, Some(Digits::Seven)),
    ],
    [
        (Digits::Zero, None),
        (Digits::Nine, None),
        (Digits::Eight, Some(Digits::One)),
        (Digits::Seven, Some(Digits::Two)),
        (Digits::Six, Some(Digits::Three)),
        (Digits::Five, Some(Digits::Four)),
        (Digits::Four, Some(Digits::Five)),
        (Digits::Three, Some(Digits::Six)),
        (Digits::Two, Some(Digits::Seven)),
        (Digits::One, Some(Digits::Eight)),
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_addition() {
        let test_data = [
            (
                Digits::Five,
                Digits::Five,
                (Digits::Zero, Some(Digits::One)),
            ),
            (Digits::Two, Digits::Four, (Digits::Six, None)),
            (Digits::Two, Digits::Two, (Digits::Four, None)),
            (Digits::Six, Digits::Four, (Digits::Zero, Some(Digits::One))),
            (
                Digits::Seven,
                Digits::Six,
                (Digits::Three, Some(Digits::One)),
            ),
            (Digits::Six, Digits::Zero, (Digits::Six, None)),
            (Digits::Two, Digits::Six, (Digits::Eight, None)),
            (
                Digits::Seven,
                Digits::Eight,
                (Digits::Five, Some(Digits::One)),
            ),
            (Digits::Zero, Digits::Zero, (Digits::Zero, None)),
            (Digits::Four, Digits::Zero, (Digits::Four, None)),
        ];

        for (left, right, expected) in test_data {
            assert_eq!(left.add(right), expected);
        }
        assert_eq!(
            Digits::Four.add(Digits::Seven),
            (Digits::One, Some(Digits::One))
        );
    }

    #[test]
    fn test_digit_subtraction() {
        let test_data = [
            (Digits::Nine, Digits::Six, (Digits::Three, None)),
            (Digits::Six, Digits::Zero, (Digits::Six, None)),
            (
                Digits::One,
                Digits::Seven,
                (Digits::Four, Some(Digits::One)),
            ),
            (
                Digits::Zero,
                Digits::Five,
                (Digits::Five, Some(Digits::One)),
            ),
            (Digits::Five, Digits::Four, (Digits::One, None)),
            (
                Digits::Seven,
                Digits::Eight,
                (Digits::Nine, Some(Digits::One)),
            ),
            (Digits::Six, Digits::Zero, (Digits::Six, None)),
            (Digits::Zero, Digits::Nine, (Digits::One, Some(Digits::One))),
            (
                Digits::Zero,
                Digits::Seven,
                (Digits::Three, Some(Digits::One)),
            ),
            (Digits::Nine, Digits::Two, (Digits::Seven, None)),
        ];
        for (left, right, expected) in test_data {
            assert_eq!(left.subtract(right), expected);
        }
        assert_eq!(
            Digits::Four.add(Digits::Seven),
            (Digits::One, Some(Digits::One))
        );
    }
}
