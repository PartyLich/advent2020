//! Solutions to 2020 day 23 problems part 2
//! --- Day 23: Crab Cups ---
use std::collections::HashMap;

use super::*;

// we care about the indexes of items that have been acted on, but don't want to store or
// manipulate anything that isnt touched
type Cups = HashMap<usize, usize>;

/// (cup state, current cup label, max label, left shifts)
type State = (Cups, usize, usize, usize);

/// parse cup state from str
// fn parse(input: &str) -> Result<Cups, String> {
fn parse(input: &str) -> Result<State, String> {
    let mut first = None;
    let mut max = 1;
    let cups = input
        .trim()
        .char_indices()
        .map(|(idx, ch)| {
            let cup = ch
                .to_digit(10)
                .map(|d| (d as usize, idx))
                .ok_or(format!("Failed to parse digit {}", ch))?;

            if first.is_none() {
                first = Some(cup.0);
            }

            if cup.0 > max {
                max = cup.0;
            }

            Ok(cup)
        })
        .collect::<Result<Cups, String>>()?;

    Ok((cups, first.unwrap(), max, 0))
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
