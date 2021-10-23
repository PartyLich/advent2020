//! Solutions to 2020 day 20 part 2
//! --- Day 20: Jurassic Jigsaw ---

/// returns count of '#' chars that are not part of a sea monster
pub fn two(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should return count of '#' chars that are not part of a sea monster";
        let expected = 273;
        let actual = two("input/20-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
