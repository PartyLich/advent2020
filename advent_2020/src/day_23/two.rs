//! Solutions to 2020 day 23 problems part 2
//! --- Day 23: Crab Cups ---
use std::collections::HashMap;

use super::*;

// we care about the indexes of items that have been acted on, but don't want to store or
// manipulate anything that isnt touched
type Cups = HashMap<usize, usize>;

/// parse cup state from str
fn parse(input: &str) -> Result<Cups, String> {
    input
        .trim()
        .char_indices()
        .map(|(idx, ch)| {
            ch.to_digit(10)
                .map(|d| (d as usize, idx))
                .ok_or(format!("Failed to parse digit {}", ch))
        })
        .collect()
}

/// returns the product of the two cup labels immediately clockwise of cup 1 after ten million
/// steps
pub fn two(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should return the product of the two cup labels immediately clockwise of cup 1 after ten million steps";
        let expected = 149245887792;
        let actual = two("input/23-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
