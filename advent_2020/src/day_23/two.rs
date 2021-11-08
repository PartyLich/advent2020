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

fn rotate_left(map: &mut Cups, units: usize, len: usize) {
    if map.is_empty() {
        return;
    }

    for (_label, idx) in map.iter_mut() {
        if *idx >= units {
            *idx -= units;
        } else if *idx < units {
            *idx = len - (units - *idx);
        }
    }
}

/// execute a single step of the game and return the new state
///
/// Next cup is always at index 0
fn step((mut state, current_label, mut max, mut shifts): State) -> State {
    let current_idx = *state.get(&current_label).unwrap_or(&(current_label - 1));

    if current_idx > 0 {
        rotate_left(&mut state, current_idx, 1_000_000);
        shifts += current_idx;
    }

    let mut removed: Vec<_> = state
        .iter()
        .filter(|(_label, idx)| (current_idx + 1..current_idx + 4).contains(idx))
        .map(|(label, _idx)| *label)
        .collect();
    while removed.len() < 3 {
        max += 1;
        let index = removed.len() + 1;
        state.insert(max, index);
        removed.push(max);
    }

    let remaining = state
        .iter()
        .filter(|(_label, idx)| !(current_idx + 1..current_idx + 4).contains(idx));

    // destination cup: the cup with a label equal to the current cup's label minus one.
    // If the destination label is not in the current set,
    //   - subtract one until a label is found OR
    //   - if the value goes below the lowest value on any cup's label, select the highest value label
    //     instead.
    let mut dest_label = current_label - 1;
    let mut dest = None;
    while dest_label > 0 && dest == None {
        dest = remaining
            .clone()
            .find(|(label, _idx)| **label == dest_label)
            .map(|(_label, idx)| idx);
        dest_label -= 1;
    }

    let dest = if dest.is_none() {
        let mut max_dest = 999_999 - shifts;
        while removed.contains(&&max_dest) {
            max_dest -= 1;
        }
        max_dest
    } else {
        *dest.unwrap()
    };

    // update indices somehow
    for (label, idx) in state.iter_mut() {
        if removed.contains(&label) {
            // insert
            *idx += dest - 3;
        } else {
            // shift
            if (4..=dest).contains(idx) {
                *idx -= 3;
            }
        }
    }

    rotate_left(&mut state, 1, 1_000_000);
    shifts += 1;

    // find next current cup
    let next = state
        .iter()
        .find_map(|(label, idx)| if *idx == 0 { Some(*label) } else { None })
        .unwrap_or_else(|| {
            max += 1;
            // insert
            state.insert(max, 0);
            max
        });

    (state, next, max, shifts)
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
