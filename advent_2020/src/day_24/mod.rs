//! Solutions to 2020 day 24 problems
//! --- Day 24: Lobby Layout ---

/// returns the number of black tiles after executing flip instructions
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should count the number black tiles";
        let expected = 10;
        let actual = one("input/24-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
