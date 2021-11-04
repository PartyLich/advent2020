//! Solutions to 2020 day 22 problems
//! --- Day 22: Crab Combat ---

/// returns the winning score from a game of 'Combat'
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should calculate the winning player's score";
        let expected = 306;
        let actual = one("input/22-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
