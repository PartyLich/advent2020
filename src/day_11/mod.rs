//! Solutions to 2020 day 10
//! --- Day 11: Seating System ---
use std::iter::Sum;

use crate::day_3::load_terrain;

mod two;

/// Map square types
#[derive(Debug, PartialEq, Clone, Copy)]
enum Seating {
    /// an occupied seat
    Full,
    /// an empty seat
    Open,
    /// a floor space with no proper seating
    Floor,
}

impl Seating {
    // return numeric value for this instance
    fn value(&self) -> usize {
        match self {
            Seating::Full => 1,
            Seating::Open => 0,
            Seating::Floor => 0,
        }
    }
}

impl From<char> for Seating {
    fn from(character: char) -> Self {
        match character {
            '#' => Self::Full,
            'L' => Self::Open,
            '.' => Self::Floor,
            _ => panic!("Invalid character in map data: '{}'", character),
        }
    }
}

impl<'a> Sum<&'a Seating> for usize {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Seating>,
    {
        iter.fold(0, |acc, next| acc + next.value())
    }
}

type Map<T> = Vec<Vec<T>>;

fn count_neighbors(map: &[Vec<Seating>], r: usize, c: usize) -> usize {
    let mut neighbors = 0usize;
    let left = c.checked_sub(1);
    let right = c.checked_add(1);
    let up = r.checked_sub(1);
    let down = r.checked_add(1);

    // count left
    neighbors += left
        .and_then(|left| map[r].get(left).map(Seating::value))
        .unwrap_or(0);
    // count right
    neighbors += right
        .and_then(|right| map[r].get(right).map(Seating::value))
        .unwrap_or(0);
    // count up
    neighbors += up
        .and_then(|up| map.get(up).and_then(|row| row.get(c).map(Seating::value)))
        .unwrap_or(0);
    // count down
    neighbors += down
        .and_then(|down| map.get(down).and_then(|row| row.get(c).map(Seating::value)))
        .unwrap_or(0);

    // diagonals
    if let Some(up) = up {
        // count up left
        neighbors += map
            .get(up)
            .and_then(|row| left.and_then(|left| row.get(left).map(Seating::value)))
            .unwrap_or(0);
        // count up right
        neighbors += map
            .get(up)
            .and_then(|row| right.and_then(|right| row.get(right).map(Seating::value)))
            .unwrap_or(0);
    }
    if let Some(down) = down {
        // count down left
        neighbors += map
            .get(down)
            .and_then(|row| left.and_then(|left| row.get(left).map(Seating::value)))
            .unwrap_or(0);
        // count down right
        neighbors += map
            .get(down)
            .and_then(|row| right.and_then(|right| row.get(right).map(Seating::value)))
            .unwrap_or(0);
    }

    neighbors
}

fn next(map: &[Vec<Seating>]) -> Option<Map<Seating>> {
    let mut result = map.to_vec();
    let map_height = map.len();
    let map_width = map[0].len();
    let mut changes = 0;

    for r in 0..map_height {
        for c in 0..map_width {
            match map[r][c] {
                Seating::Full => {
                    // If a seat is occupied and four or more seats adjacent to it are also occupied,
                    // the seat becomes empty.
                    if count_neighbors(map, r, c) >= 4 {
                        result[r][c] = Seating::Open;
                        changes += 1;
                    }
                }
                Seating::Open => {
                    // If a seat is empty and there are no occupied seats adjacent to it,
                    // the seat becomes occupied.
                    if count_neighbors(map, r, c) == 0 {
                        result[r][c] = Seating::Full;
                        changes += 1;
                    }
                }
                Seating::Floor => {}
            }
        }
    }

    if changes == 0 {
        return None;
    }
    Some(result)
}

/// return the count of occupied seats once the system has stagnated
pub fn one(file_path: &str) -> usize {
    let mut map: Map<Seating> = load_terrain(file_path);
    while let Some(next_map) = next(&map) {
        map = next_map;
    }

    map.iter()
        .fold(0, |acc, next| acc + next.iter().sum::<usize>())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the count of occupied seats once the system has stagnated";
        let expected = 37;
        let actual = one("input/11-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
