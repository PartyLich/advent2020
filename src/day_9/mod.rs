//! Solutions to 2020 day 6
//! --- Day 9: Encoding Error ---
use std::num::ParseIntError;

use crate::day_1::read_file;

// reads a newline separated 'series of numbers" (size/range not specified in the problem :( ) from
// a &str
fn read_str(serialized: &str) -> Result<Vec<usize>, ParseIntError> {
    todo!()
}

/// Validate an eXchange-Masking Addition System (XMAS) encoded sequence.
///
/// Result is `Ok(())` if the series is valid.
/// `Err(usize)` contains the first invalid number encountered
fn validate(preamble_len: usize, series: &[usize]) -> Result<(), usize> {
    todo!()
}

/// find the first number which is not the sum of two of the preamble numbers before it
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_series() {
        let msg = "should find the first number which is not the sum of two of the preamble numbers before it";
        let expected = 127;

        let file_content = read_file("input/9-t.txt");
        let series = read_str(&file_content).unwrap();
        let actual = validate(5, &series).unwrap_err();

        assert_eq!(actual, expected, "{}", msg);
    }
}
