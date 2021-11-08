//! Solutions to 2020 day 24 problems
//! --- Day 24: Lobby Layout ---
use std::collections::HashMap;
use std::ops::Add;

use crate::day_1::read_file;

/// Hexagonal tile neighbor direction
#[derive(Debug, Eq, Hash, PartialEq)]
struct Direction(isize, isize);

impl Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

/// assemble tile layout from a list of instructions
fn assemble(instructions: Vec<Direction>) -> HashMap<Direction, bool> {
    todo!()
}

/// returns the number of black tiles after executing flip instructions
pub fn one(file_path: &str) -> usize {
    let input = read_file(file_path);
    let instructions = input
        .lines()
        .map(Direction::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assemble(instructions)
        .values()
        .filter(|tile| **tile)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should count the number black tiles";
        let expected = 10;
        let actual = one("input/24-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
