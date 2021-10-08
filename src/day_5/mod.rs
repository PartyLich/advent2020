//! Solutions to 2020 day 5 problems
//! --- Day 5: Binary Boarding ---
use std::convert::TryFrom;

/// Binary space partitioning instruction
#[derive(Debug, Clone, Copy)]
enum Half {
    Upper,
    Lower,
}

impl TryFrom<char> for Half {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'F' | 'L' => Ok(Self::Lower),
            'B' | 'R' => Ok(Self::Upper),
            _ => Err(format!("Invalid character encountered: {}", value)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_pass() {
        let msg = "should deserialize boarding pass";
        let actual = "FBFBBFFRLR".parse::<BoardingPass>().unwrap();
        assert_eq!(actual.seat_id(), 357, "{}", msg);
        assert_eq!(actual.row, 44, "{}", msg);
        assert_eq!(actual.col, 5, "{}", msg);

        let actual = "BFFFBBFRRR".parse::<BoardingPass>().unwrap();
        assert_eq!(actual.seat_id(), 567, "{}", msg);
        assert_eq!(actual.row, 70, "{}", msg);
        assert_eq!(actual.col, 7, "{}", msg);

        let actual = "BBFFBBFRLL".parse::<BoardingPass>().unwrap();
        assert_eq!(actual.seat_id(), 820, "{}", msg);
        assert_eq!(actual.row, 102, "{}", msg);
        assert_eq!(actual.col, 4, "{}", msg);

        let actual = "FFFBBBFRRR".parse::<BoardingPass>().unwrap();
        assert_eq!(actual.seat_id(), 119, "{}", msg);
        assert_eq!(actual.row, 14, "{}", msg);
        assert_eq!(actual.col, 7, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return the highest seat ID in a list of boarding passes";
        let expected = 820;
        let actual = one("input/5-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
