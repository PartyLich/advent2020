//! Solutions to 2020 day 23 problems part 2
//! --- Day 23: Crab Cups ---

use super::*;

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
