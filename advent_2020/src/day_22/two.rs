//! Solutions to 2020 day 22 problems part two
//! --- Day 22: Crab Combat ---
use std::collections::HashSet;

use super::Deck;

/// Completed game state, as a (winner index, winning deck) pair
type GameResult = (usize, Deck);

/// Recursive Combat game state
#[derive(Debug)]
enum Game {
    /// Game with rounds remaining to be played
    InProgress {
        decks: (Deck, Deck),
        previous_decks: (HashSet<Deck>, HashSet<Deck>),
    },
    /// Game that has been won
    Complete(GameResult),
}

impl Game {
    /// create a new game of Combat
    pub fn new(deck1: Deck, deck2: Deck) -> Self {
        Self::InProgress {
            decks: (deck1, deck2),
            previous_decks: Default::default(),
        }
    }
}

/// returns the winning score from a game of 'Combat'
pub fn two(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should calculate the winning player's score";
        let expected = 291;
        let actual = two("input/22-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
