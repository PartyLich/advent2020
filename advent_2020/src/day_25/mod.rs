//! Solutions to 2020 day 24 problems
//! --- Day 25: Combo Breaker ---

/// transform a subject number
fn transform(subject: usize, value: Option<usize>) -> usize {
    let mut result = value.unwrap_or(1);

    // - Set the value to itself multiplied by the subject number.
    result *= subject;
    // - Set the value to the remainder after dividing the value by 20201227.
    result % 20201227
}

/// returns the encryption key
pub fn one(file_path: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transform_subject() {
        let msg = "should transform a subject number according to the encrpytion rules";
        let expected = 5764801;
        let actual = {
            let mut result = transform(7, None);
            for _ in 0..7 {
                result = transform(7, Some(result));
            }
            result
        };
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return the encryption key";
        let expected = 14897079;
        let actual = one("input/25-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
