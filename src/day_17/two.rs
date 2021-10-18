//! Solutions to 2020 day 17 part 2
//! --- Day 17: Conway Cubes ---

/// Count the number of cubes in the active state after the sixth cycle
pub fn two(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should return the number of cubes in the active state after the sixth cycle";
        let expected = 848;
        let actual = two("input/17-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
