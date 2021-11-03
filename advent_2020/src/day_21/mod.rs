//! Solutions to 2020 day 21 problems
//! --- Day 21: Allergen Assessment ---
use std::collections::HashMap;

/// a food ingredient
#[derive(Debug, Default)]
struct Ingredient<'a> {
    /// number of foods this ingredient appears in
    appearances: usize,
    /// max times an allergen is associated with this ingredient
    max: (&'a str, usize),
    /// map of allergens associated with this ingredient
    allergens: HashMap<&'a str, usize>,
}

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
