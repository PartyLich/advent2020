//! Solutions to 2020 day 22 problems
//! --- Day 22: Crab Combat ---
use std::collections::VecDeque;
use std::num::ParseIntError;

/// parse a deck from a str
fn parse(input: &str) -> Result<VecDeque<usize>, ParseIntError> {
    input.lines().skip(1).map(|line| line.parse()).collect()
}

/// returns a deck's score
fn get_score(deck: &[usize]) -> usize {
    let size = deck.len();

    deck.iter()
        .enumerate()
        .fold(0, |acc, (idx, value)| acc + (value * (size - idx)))
}

/// returns the winning score from a game of 'Combat'
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scoring() {
        let msg = "should calculate a deck's score";

        let deck = vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1];

        let expected = 306;
        let actual = get_score(&deck);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should calculate the winning player's score";
        let expected = 306;
        let actual = one("input/22-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
