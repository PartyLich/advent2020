//! Solutions to 2020 day 20
//! --- Day 20: Jurassic Jigsaw ---

/// returns the product of the four corner tile ids
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return product of the four corner tile ids";
        let expected = 20899048083289;
        let actual = one("input/20-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
