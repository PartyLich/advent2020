//! Solutions to 2020 day 13
//! --- Day 13: Shuttle Search ---

/// return ID of the earliest bus you can take to the airport multiplied by the number of minutes
/// you'll need to wait for that bus
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait";
        let expected = 295;
        let actual = one("input/13-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
