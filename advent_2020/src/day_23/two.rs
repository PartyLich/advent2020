//! Solutions to 2020 day 23 problems part 2
//! --- Day 23: Crab Cups ---
use std::collections::HashMap;

use super::*;

// we care about labels. this item, and the label that follow it. A linked list (ish)
type Cups = HashMap<usize, usize>;

/// (cup state, current cup label)
type State = (Cups, usize);

/// parse cup state from str
fn parse(input: &str) -> Result<State, String> {
    let mut first = None;
    let mut cups = input
        .trim()
        .char_indices()
        .rev()
        .scan(10, |next_cup, (idx, ch)| {
            let label = ch.to_digit(10).map(|d| d as usize).unwrap();
            let result = Some((label, *next_cup));
            *next_cup = label;

            if idx == 0 {
                first = Some(label);
            }

            result
        })
        .collect::<Cups>();
    cups.insert(1_000_000, first.unwrap());

    Ok((cups, first.unwrap()))
}

/// execute a single step of the game and return the new state
fn step((mut cups, current_label): State) -> State {
    let removed_0 = *cups.entry(current_label).or_insert(current_label + 1);
    let removed_1 = *cups.entry(removed_0).or_insert(removed_0 + 1);
    let removed_2 = *cups.entry(removed_1).or_insert(removed_1 + 1);
    let removed = [removed_0, removed_1, removed_2];

    // destination cup: the cup with a label equal to the current cup's label minus one.
    // If the destination label is not in the current set,
    //   - subtract one until a label is found OR
    //   - if the value goes below the lowest value on any cup's label, select the highest value
    //     label instead.
    let mut dest_label = current_label - 1;
    if dest_label < 1 {
        dest_label = 1_000_000;
    }
    while removed.contains(&dest_label) {
        dest_label -= 1;

        if dest_label < 1 {
            dest_label = 1_000_000;
        }
    }

    // update links
    let tail = *cups.get(&dest_label).unwrap();
    let after_removed = *cups.entry(removed_2).or_insert(removed_2 + 1);
    cups.entry(current_label)
        .and_modify(|next| *next = after_removed);
    cups.entry(dest_label).and_modify(|next| *next = removed_0);
    cups.entry(removed_2).and_modify(|next| *next = tail);

    // find the next current cup
    let next = *cups.get(&current_label).unwrap();

    (cups, next)
}

/// returns the product of the two cup labels immediately clockwise of cup 1 after ten million
/// steps
pub fn two(file_path: &str) -> usize {
    const STEPS: usize = 10_000_000;
    let input = read_file(file_path);
    let mut state = parse(&input).expect("Failed to parse initial cup state");

    for _ in 0..STEPS {
        state = step(state);
    }

    let one = *state.0.get(&1).unwrap();
    let two = *state.0.get(&one).unwrap();
    one * two
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
