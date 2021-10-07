//! Solutions to 2020 day 4 problems
//! --- Day 4: Passport Processing ---
use crate::day_1::read_file;

// Count the number of valid passports - those that have all required fields.
// Treat cid as optional.
pub fn one(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should count the number of valid passports, with optional cid";
        let expected = 2;
        let actual = one("input/4-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
