//! Solutions to 2020 day 19 problems
//! --- Day 19: Monster Messages ---

/// return the count of messages that match rule 0
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should count the number of messages that match rule 0";
        let expected = 2;
        let actual = one("input/19-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
