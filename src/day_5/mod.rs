//! Solutions to 2020 day 5 problems
//! --- Day 5: Binary Boarding ---
use std::convert::TryFrom;
use std::str::FromStr;

use crate::day_1::read_file;

/// Total seating rows in the aircraft
const ROWS: u8 = 128;

/// Total seating columns in the aircraft
const COLS: u8 = 8;

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

/// return resulting index pair from processing a [binary space partitioning instruction](Half)
/// with the provided min and max index
fn partition(pair: (u8, u8), value: &Half) -> (u8, u8) {
    let (start, end) = pair;
    let mid = (start + end) / 2;
    match value {
        Half::Lower => (start, mid),
        Half::Upper => (mid + 1, end),
    }
}

/// North Pole airline flight boarding pass
#[derive(Debug, Clone, Copy)]
struct BoardingPass {
    /// exactly one of the 128 rows on the plane (numbered 0 through 127)
    row: u8,
    /// exactly one of the 8 columns of seats on the plane (numbered 0 through 7).
    col: u8,
}

impl BoardingPass {
    pub fn seat_id(&self) -> usize {
        (self.row as usize * 8 + self.col as usize).into()
    }
}

impl FromStr for BoardingPass {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 10 {
            return Err(format!(
                "Unable to parse BoardingPass. Expected length 10, actual {}",
                value.len()
            ));
        }

        // parse partitioning intructions
        let instructions: Vec<Half> = value
            .chars()
            .map(|val| Half::try_from(val))
            .collect::<Result<Vec<_>, _>>()?;

        let (row_instructions, col_instructions) = instructions.split_at(7);
        let (row, _) = row_instructions.iter().fold((0, ROWS - 1), partition);
        let (col, _) = col_instructions.iter().fold((0, COLS - 1), partition);

        Ok(Self { row, col })
    }
}

/// Return highest seat ID in a list of serialized [`BoardingPass`]es
pub fn one(file_path: &str) -> usize {
    read_file(file_path)
        .lines()
        .map(|pass_str| {
            pass_str
                .parse::<BoardingPass>()
                .expect("Boarding pass parse failure")
        })
        .max_by_key(|pass| pass.seat_id())
        .map(|pass| pass.seat_id())
        .unwrap()
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
