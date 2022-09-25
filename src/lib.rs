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
    fn add(self: Self, rhs: Digits) -> (Digits, Option<Digits>) {
        return ADDITION_MATRIX[self.to_usize()][rhs.to_usize()];
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
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
}
