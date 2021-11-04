//! Solutions to 2020 day 22 problems
//! --- Day 22: Crab Combat ---
use std::collections::VecDeque;
use std::num::ParseIntError;

use crate::day_1::read_file;

mod two;
pub use two::two;

/// A deck of playing cards
type Deck = VecDeque<usize>;

/// Combat game state
#[derive(Debug)]
enum Game {
    /// Game with rounds remaining to be played
    InProgress((Deck, Deck)),
    /// Game that has been won
    Complete(Deck),
}

impl Game {
    /// create a new game of Combat
    pub fn new(deck1: Deck, deck2: Deck) -> Self {
        Self::InProgress((deck1, deck2))
    }
}

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

/// play a round of 'Combat' with the two provided decks
fn play_round((mut deck1, mut deck2): (Deck, Deck)) -> Game {
    deck1.pop_front().map(|card1| {
        deck2.pop_front().map(|card2| {
            if card1 > card2 {
                deck1.extend([card1, card2]);
            } else {
                deck2.extend([card2, card1]);
            }
        })
    });

    if deck1.is_empty() {
        return Game::Complete(deck2);
    }
    if deck2.is_empty() {
        return Game::Complete(deck1);
    }
    Game::InProgress((deck1, deck2))
}

/// returns the winning score from a game of 'Combat'
pub fn one(file_path: &str) -> usize {
    let input = read_file(file_path);
    let mut decks: Vec<_> = input
        .split("\n\n")
        .map(parse)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input decks");

    let mut game = Game::new(decks.remove(0), decks.remove(0));
    loop {
        match game {
            Game::Complete(winning_deck) => {
                return get_score(&Vec::from(winning_deck));
            }
            Game::InProgress(next) => {
                game = play_round(next);
            }
        }
    }
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
