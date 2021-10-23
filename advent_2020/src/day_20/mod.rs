//! Solutions to 2020 day 20
//! --- Day 20: Jurassic Jigsaw ---

/// return true if two lists match or are mirrors of each other
fn compare(a: &[char], b: &[char]) -> bool {
    // match
    a.iter().zip(b.iter()).all(|(a, b)| a.eq(b)) ||
        // match flipped
        a.iter().zip(b.iter().rev()).all(|(a, b)| a.eq(b))
}

/// returns the product of the four corner tile ids
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn comparison() {
        let msg = "should return true if lists are equal";

        let a = vec!['.', '.', '.', '#', '#', '#', '.', '#', '.', '.'];
        let b = vec!['.', '.', '#', '.', '#', '#', '#', '.', '.', '.'];
        let actual = compare(&a, &b);
        assert!(actual, "{}", msg);

        let a = vec!['.', '.', '.', '#', '#', '#', '.', '#', '.', '.'];
        let b = vec!['.', '.', '.', '#', '#', '#', '.', '#', '.', '.'];
        let actual = compare(&a, &b);
        assert!(actual, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return product of the four corner tile ids";
        let expected = 20899048083289;
        let actual = one("input/20-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
