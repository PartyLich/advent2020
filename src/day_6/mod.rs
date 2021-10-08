//! Solutions to 2020 day 6
//! --- Day 6: Custom Customs ---

pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should sum the unique yes answers for each group";
        let expected = 11;
        let actual = one("input/6-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
