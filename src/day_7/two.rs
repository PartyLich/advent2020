//! Solutions to 2020 day 7
//! --- Day 7: Handy Haversacks ---

/// count the number of bags descendant of a shiny gold root bag
pub fn two(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should calc how many children a shiny gold root has";
        let expected = 126;
        let actual = two("input/7-t2.txt");
        assert_eq!(actual, expected, "{}", msg);

        let expected = 32;
        let actual = two("input/7-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
