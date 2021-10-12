//! Solutions to 2020 day 10
//! --- Day 11: Seating System ---

/// return the count of occupied seats once the system has stagnated
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the count of occupied seats once the system has stagnated";
        let expected = 37;
        let actual = one("input/11-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
