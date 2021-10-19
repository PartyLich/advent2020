//! Solutions to 2020 day 18 problems
//! --- Day 18: Operation Order ---

/// return the sum of the expressions on each line
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should sum the result of each line";
        let expected = 26_457;
        let actual = one("input/18-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
