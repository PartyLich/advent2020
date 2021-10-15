//! Solutions to 2020 day 15
//!

pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the 2020th number spoken";
        let expected = 436;
        let actual = step(vec![0, 3, 6], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 1;
        let actual = step(vec![1, 3, 2], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 10;
        let actual = step(vec![2, 1, 3], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 27;
        let actual = step(vec![1, 2, 3], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 78;
        let actual = step(vec![2, 3, 1], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 438;
        let actual = step(vec![3, 2, 1], 2020);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 1836;
        let actual = step(vec![3, 1, 2], 2020);
        assert_eq!(actual, expected, "{}", msg);
    }
}
