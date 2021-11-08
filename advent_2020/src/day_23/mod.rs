//! Solutions to 2020 day 23 problems
//! --- Day 23: Crab Cups ---
use crate::day_1::read_file;

type State = Vec<usize>;

/// parse cup state from str
fn parse(input: &str) -> Result<Vec<usize>, String> {
    input
        .trim()
        .chars()
        .map(|ch| {
            ch.to_digit(10)
                .map(|d| d as usize)
                .ok_or(format!("Failed to parse digit {}", ch))
        })
        .collect()
}

/// execute a single step of the game and return the new state
///
/// Next cup is always at index 0
fn step(mut state: State) -> State {
    let current = 0;
    let removed: Vec<_> = state.splice((current + 1)..(current + 4), []).collect();

    // destination cup: the cup with a label equal to the current cup's label minus one.
    // If the destination label is not in the current set,
    //   - subtract one until a label is found OR
    //   - if the value goes below the lowest value on any cup's label, select the highest value label
    //     instead.
    let mut dest_label = state[current] - 1;
    let mut dest = None;
    while dest_label > 0 && dest == None {
        dest = state
            .iter()
            .enumerate()
            .find(|(_, label)| **label == dest_label)
            .map(|(idx, _)| idx);
        dest_label -= 1;
    }

    if dest.is_none() {
        dest = state
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(idx, _)| idx);
    }
    let dest = dest.unwrap();
    state.splice(dest + 1..dest + 1, removed);

    // select a new current cup: the cup which is immediately clockwise of the current cup.
    state.rotate_left(1);

    state
}

/// Starting after the cup labeled 1, collects the other cups' labels clockwise into a single string
/// with no extra characters
fn format_result(mut state: State) -> String {
    let one_idx = state
        .iter()
        .enumerate()
        .find(|(_, label)| **label == 1)
        .map(|(idx, _)| idx)
        .unwrap();
    state.rotate_left(one_idx);

    state
        .into_iter()
        .skip(1)
        .map(|digit| digit.to_string())
        .collect()
}

/// should return the ordered cup labels after cup 1 following 100 steps
pub fn one(file_path: &str) -> String {
    const STEPS: usize = 100;
    let input = read_file(file_path);
    let mut cups = parse(&input).expect("Failed to parse initial cup state");

    for _ in 0..STEPS {
        cups = step(cups);
    }

    format_result(cups)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn steps() {
        let msg = "should return the ordered cup labels after a step";
        let initial = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let expected = vec![2, 8, 9, 1, 5, 4, 6, 7, 3];
        let actual = step(initial);
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should return the ordered cup labels after 10 steps";
        let initial = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let expected = vec![8, 3, 7, 4, 1, 9, 2, 6, 5];
        let mut actual = initial;
        for _ in 0..10 {
            actual = step(actual);
        }
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return the ordered cup labels after cup 1 following 100 steps";
        let expected = 67384529.to_string();
        let actual = one("input/23-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
