//! Solutions to 2020 day 15
//!
use std::collections::HashMap;

use crate::day_1::read_file;
use crate::day_13::parse_csv_lossy;

fn step(initial: Vec<u32>, steps: u32) -> u32 {
    let mut map: HashMap<u32, (u32, u32)> = HashMap::new();
    let mut last: u32 = 0;
    initial.iter().enumerate().for_each(|(idx, num)| {
        let turn = idx as u32 + 1;
        map.insert(*num, (turn, turn));
        last = *num;
    });

    let next = initial.len() as u32 + 1;
    let mut next_num = 0;
    for step in next..=steps {
        let previous_turns = *map.get(&last).unwrap_or(&(step, step));
        // next number to speak is the difference between the turn number when it was last spoken
        // and the turn number of the time it was most recently spoken before then
        next_num = previous_turns.1 - previous_turns.0;

        // update entry for this turn's number
        let prev = map.get(&next_num).copied().unwrap_or((step, step));
        map.insert(next_num, (prev.1, step));

        last = next_num;
    }

    next_num
}

/// return the 2020th number spoken
pub fn one(file_path: &str) -> u32 {
    const STEPS: u32 = 2020;
    let content = read_file(file_path);
    let numbers: Vec<u32> = parse_csv_lossy(&content);
    step(numbers, STEPS)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the 2020th number spoken";
        let expected = 436;
        let actual = step(vec![0, 3, 6], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 1;
        let actual = step(vec![1, 3, 2], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 10;
        let actual = step(vec![2, 1, 3], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 27;
        let actual = step(vec![1, 2, 3], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 78;
        let actual = step(vec![2, 3, 1], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 438;
        let actual = step(vec![3, 2, 1], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 1836;
        let actual = step(vec![3, 1, 2], 2020);
        assert_eq!(actual, expected, "{}", msg);
    }
}
