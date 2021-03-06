//! Solutions to 2020 day 6
//! --- Day 9: Encoding Error ---
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::day_1::read_file;

/// reads a newline separated 'series of numbers" (size/range not specified in the problem :( ) from
/// a &str
pub fn parse_numbers(serialized: &str) -> Result<Vec<usize>, ParseIntError> {
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
    parse_numbers(&file_content).unwrap()
}

/// find the first number which is not the sum of two of the preamble numbers before it
pub fn one(file_path: &str) -> usize {
    const PREAMBLE: usize = 25;
    let series = series_from_file(file_path);

    validate(PREAMBLE, &series).unwrap_err()
}

/// find a contiguous set of at least two numbers in the series which sum to the invalid number
/// Return the sum of the smallest and largest number in this contiguous range;
fn find_weakness(invalid: usize, series: &[usize]) -> usize {
    let (mut start, mut end) = (0, 1);
    while end < series.len() {
        let sum: usize = series[start..=end].iter().sum();
        if sum == invalid {
            let max = series[start..=end].iter().max().unwrap();
            let min = series[start..=end].iter().min().unwrap();

            return min + max;
        }

        if sum < invalid {
            end += 1;
        } else {
            start += 1;
        }
    }

    panic!("Failed to find a solution")
}

/// find a contiguous set of at least two numbers in your list which sum to the invalid number
/// Return the sum of the smallest and largest number in this contiguous range;
pub fn two(file_path: &str) -> usize {
    const PREAMBLE: usize = 25;
    let series = series_from_file(file_path);
    let invalid = validate(PREAMBLE, &series).unwrap_err();

    find_weakness(invalid, &series)
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

    #[test]
    fn finds_weakness() {
        let msg = "should find the sum of the min and max values in the contiguous set that sum to the invalid number";
        let expected = 62;

        let series = series_from_file("input/9-t.txt");
        let invalid = validate(5, &series).unwrap_err();
        let actual = find_weakness(invalid, &series);

        assert_eq!(actual, expected, "{}", msg);
    }
}
