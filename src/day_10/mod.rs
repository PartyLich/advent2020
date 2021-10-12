//! Solutions to 2020 day 10
//! --- Day 10: Adapter Array ---

/// returns he number of 1-jolt differences multiplied by the number of 3-jolt
/// differences
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of 1-jolt differences multiplied by the number of 3-jolt differences";
        let expected = 35;
        let actual = one("input/10-t.txt");
        assert_eq!(actual, expected, "{}", msg);

        let expected = 10 * 22;
        let actual = one("input/10-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
