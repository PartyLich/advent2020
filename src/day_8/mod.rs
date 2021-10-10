//! Solutions to 2020 day 8
//! --- Day 8: Handheld Halting ---

/// return the accumulator value before looping
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the accumulator value before looping";
        let expected = 5;
        let actual = one("input/8-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
