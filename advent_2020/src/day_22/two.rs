//! Solutions to 2020 day 22 problems part two
//! --- Day 22: Crab Combat ---
use std::collections::HashSet;

use crate::day_1::read_file;

use super::{get_score, parse, Deck};

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

/// play a round of 'Recursive Combat' with the two provided decks
///
/// See [AoC 2020 Day 22 part 2](https://adventofcode.com/2020/day/22#part2) for game rules
fn play_round(
    mut previous_decks: (HashSet<Deck>, HashSet<Deck>),
    (mut deck1, mut deck2): (Deck, Deck),
) -> Game {
    previous_decks.0.insert(deck1.clone());
    previous_decks.1.insert(deck2.clone());

    // draw top cards
    deck1.pop_front().map(|card1| {
        deck2.pop_front().map(|card2| {
            if deck1.len() >= card1 && deck2.len() >= card2 {
                // the winner of the round is determined by playing a new sub-game
                let subdeck1 = deck1.iter().copied().take(card1).collect();
                let subdeck2 = deck2.iter().copied().take(card2).collect();
                let sub_game = Game::new(subdeck1, subdeck2);

                match sub_game.resolve() {
                    (1, _) => {
                        deck1.extend([card1, card2]);
                    }
                    (2, _) => {
                        deck2.extend([card2, card1]);
                    }
                    _ => panic!("Invalid game result"),
                }
            } else {
                // the winner of the round is the player with the higher-value card.
                if card1 > card2 {
                    deck1.extend([card1, card2]);
                } else {
                    deck2.extend([card2, card1]);
                }
            }
        })
    });

    if deck1.is_empty() {
        return Game::Complete((2, deck2));
    }
    if deck2.is_empty() {
        return Game::Complete((1, deck1));
    }

    Game::InProgress {
        decks: (deck1, deck2),
        previous_decks,
    }
}

/// returns the winning score from a game of 'Combat'
pub fn two(file_path: &str) -> usize {
    let input = read_file(file_path);
    let mut decks: Vec<_> = input
        .split("\n\n")
        .map(parse)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input decks");
    let game = Game::new(decks.remove(0), decks.remove(0));
    let (_winner, winning_deck) = game.resolve();

    get_score(&Vec::from(winning_deck))
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
