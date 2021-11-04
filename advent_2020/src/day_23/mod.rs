//! Solutions to 2020 day 23 problems
//! --- Day 23: Crab Cups ---

/// should return the ordered cup labels after cup 1 following 100 steps
pub fn one(file_path: &str) -> String {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the ordered cup labels after cup 1 following 100 steps";
        let expected = 67384529.to_string();
        let actual = one("input/23-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
