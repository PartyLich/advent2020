//! Solutions to 2020 day 11 part 2
//! --- Day 11: Seating System ---
use super::{Map, Seating};

/// return the count of occupied seats once the system has stagnated
pub fn two(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should return the count of occupied seats once the system has stagnated";
        let expected = 26;
        let actual = two("input/11-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
