//! Solutions to 2020 day 24 problems
//! --- Day 24: Lobby Layout ---
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::str::FromStr;

use parser::three::lib::{choice, one_or_more, p_char};

use crate::day_1::read_file;

/// Hexagonal tile neighbor direction
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

/// count active neighbors for a given position
fn count_neighbors(map: &HashSet<Direction>, pos: Direction) -> usize {
    let neighbors = [
        // w
        pos + Direction(-1, 0),
        // nw
        pos + Direction(0, -1),
        // sw
        pos + Direction(-1, 1),
        // e
        pos + Direction(1, 0),
        // ne
        pos + Direction(1, -1),
        // se
        pos + Direction(0, 1),
    ];

    let mut count = 0;
    for neighbor in neighbors {
        if map.contains(&neighbor) {
            count += 1;
        }
    }

    count
}

/// update a tile set according to the daily rules
fn flip_tiles(map: HashSet<Direction>) -> HashSet<Direction> {
    let mut result = HashSet::new();

    fn helper(map: &HashSet<Direction>, pos: Direction, is_black: bool) -> Option<Direction> {
        let count = count_neighbors(map, pos);
        // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped
        // to white.
        if (is_black && (1..=2).contains(&count) )
            // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to
            // black.
        || count == 2
        {
            Some(pos)
        } else {
            None
        }
    }

    for pos in map.iter() {
        let neighbors = [
            // self
            *pos,
            // w
            *pos + Direction(-1, 0),
            // nw
            *pos + Direction(0, -1),
            // sw
            *pos + Direction(-1, 1),
            // e
            *pos + Direction(1, 0),
            // ne
            *pos + Direction(1, -1),
            // se
            *pos + Direction(0, 1),
        ];

        let active = neighbors
            .iter()
            .filter_map(|pos| helper(&map, *pos, map.contains(pos)));
        result.extend(active);
    }

    result
}

/// returns the number of black tiles after 100 days of flips
pub fn two(file_path: &str) -> usize {
    let input = read_file(file_path);
    let instructions = input
        .lines()
        .map(Direction::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut tile_map: HashSet<Direction> = assemble(instructions)
        .into_iter()
        .filter_map(|(pos, is_black)| if is_black { Some(pos) } else { None })
        .collect();

    for _ in 0..100 {
        tile_map = flip_tiles(tile_map);
    }

    tile_map.len()
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

    #[test]
    fn part_two() {
        let msg = "should count the number black tiles";
        let expected = 2208;
        let actual = two("input/24-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
