//! Solutions to 2020 day 7
//! --- Day 7: Handy Haversacks ---

/// given a file containing luggage rules, returns the number of root bags have shiny gold leaf nodes
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should calc how many root bags have shiny gold leaf nodes";
        let expected = 4;
        let actual = one("input/7-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
