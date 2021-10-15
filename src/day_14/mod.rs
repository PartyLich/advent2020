//! Solutions to 2020 day 14
//! --- Day 14: Docking Data ---

/// returns the sum of the values in memory after executing a the supplied initialization program
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should sum the values in memory";
        let expected = 165;
        let actual = one("input/14-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
