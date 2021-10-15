//! Solutions to 2020 day 14 part two
//! --- Day 14: Docking Data ---

/// returns the sum of the values in memory after executing a the supplied initialization program
pub fn two(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should sum the values in memory";
        let expected = 208;
        let actual = two("input/14-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
