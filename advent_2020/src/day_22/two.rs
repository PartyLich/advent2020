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

    /// play rounds until the game is complete
    pub fn resolve(mut self) -> GameResult {
        loop {
            match self {
                Self::InProgress {
                    decks,
                    previous_decks,
                } => {
                    // if there was a previous round in this game that had exactly the same cards in
                    // the same order in the same players' decks, the game instantly ends in a win
                    // for player 1.
                    if previous_decks.0.contains(&decks.0) || previous_decks.1.contains(&decks.1) {
                        return (1, decks.0);
                    }

                    self = play_round(previous_decks, decks);
                }
                Self::Complete(result) => {
                    return result;
                }
            }
        }
    }
}

fn play_round(
    mut previous_decks: (HashSet<Deck>, HashSet<Deck>),
    (mut deck1, mut deck2): (Deck, Deck),
) -> Game {
    todo!();
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
