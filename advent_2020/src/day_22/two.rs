//! Solutions to 2020 day 22 problems part two
//! --- Day 22: Crab Combat ---

/// returns the winning score from a game of 'Combat'
pub fn two(file_path: &str) -> usize {
    todo!();
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
