//! Solutions to 2020 day 6
//! --- Day 9: Encoding Error ---
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::day_1::read_file;

// reads a newline separated 'series of numbers" (size/range not specified in the problem :( ) from
// a &str
fn read_str(serialized: &str) -> Result<Vec<usize>, ParseIntError> {
    serialized
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<_>, _>>()
}

/// Validate an eXchange-Masking Addition System (XMAS) encoded sequence.
///
/// Result is `Ok(())` if the series is valid.
/// `Err(usize)` contains the first invalid number encountered
fn validate(preamble_len: usize, series: &[usize]) -> Result<(), usize> {
    let mut preamble: VecDeque<_> = series.iter().take(preamble_len).collect();

    // check each value after the preamble
    for entry in series.iter().skip(preamble_len) {
        // find two values in the preamble that sum to the current series value
        let valid = preamble.iter().find(|a| {
            entry
                .checked_sub(***a)
                .map(|target| preamble.contains(&&target))
                .unwrap_or(false)
        });
        if valid.is_none() {
            return Err(*entry);
        }

        // valid entry. advance the preamble
        preamble.push_back(entry);
        preamble.pop_front();
    }

    Ok(())
}

/// read a series of numbers from a file. panic on errors
fn series_from_file(file_path: &str) -> Vec<usize> {
    let file_content = read_file(file_path);
    read_str(&file_content).unwrap()
}

/// find the first number which is not the sum of two of the preamble numbers before it
pub fn one(file_path: &str) -> usize {
    const PREAMBLE: usize = 25;
    let series = series_from_file(file_path);

    validate(PREAMBLE, &series).unwrap_err()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_series() {
        let msg = "should find the first number which is not the sum of two of the preamble numbers before it";
        let expected = 127;

        let series = series_from_file("input/9-t.txt");
        let actual = validate(5, &series).unwrap_err();

        assert_eq!(actual, expected, "{}", msg);
    }
}
