//! Solutions to 2020 day 21 problems
//! --- Day 21: Allergen Assessment ---

/// return count of allergen free ingredient appearances
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should count of allergen free ingredient appearances";
        let expected = 5;
        let actual = one("input/21-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
