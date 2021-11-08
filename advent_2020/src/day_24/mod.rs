//! Solutions to 2020 day 24 problems
//! --- Day 24: Lobby Layout ---
use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;

use parser::three::three::{choice, one_or_more, p_char};

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

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w = p_char('w').map(|_| Direction(-1, 0));
        let nw = p_char('n').and_then(w.clone()).map(|_| Direction(0, -1));
        let sw = p_char('s').and_then(w.clone()).map(|_| Direction(-1, 1));
        let e = p_char('e').map(|_| Direction(1, 0));
        let ne = p_char('n').and_then(e.clone()).map(|_| Direction(1, -1));
        let se = p_char('s').and_then(e.clone()).map(|_| Direction(0, 1));
        let all = choice([nw, ne, sw, se, w, e]);

        let parser = one_or_more(all).with_label("Instruction List".to_string());

        match parser.parse(s) {
            Ok((_input, value)) => Ok(value
                .into_iter()
                .fold(Direction(0, 0), |acc, next| acc + next)),
            Err(err) => Err(format!("{}", err)),
        }
    }
}

/// assemble tile layout from a list of instructions
fn assemble(instructions: Vec<Direction>) -> HashMap<Direction, bool> {
    instructions
        .into_iter()
        .fold(Default::default(), |mut acc, next| {
            let tile = acc.entry(next).or_insert(false);
            *tile = !*tile;

            acc
        })
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
    fn parsing() {
        let msg = "should parse a tile flip instruction";
        let expected = Direction(0, 0);
        let actual: Direction = "nwwswee".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = Direction(-3, 2);
        let actual: Direction = "sesenwnenenewseeswwswswwnenewsewsw".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should count the number black tiles";
        let expected = 10;
        let actual = one("input/24-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
