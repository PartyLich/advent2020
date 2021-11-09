//! Solutions to 2020 day 24 problems
//! --- Day 25: Combo Breaker ---

/// returns the encryption key
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the encryption key";
        let expected = 14897079;
        let actual = one("input/25-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
