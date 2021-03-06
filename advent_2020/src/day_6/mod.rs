//! Solutions to 2020 day 6
//! --- Day 6: Custom Customs ---
use std::convert::TryInto;
use std::ops::{BitAnd, BitOr};
use std::str::FromStr;

use crate::day_1::read_file;

#[derive(Debug, Copy, Clone, PartialEq)]
struct AnswerFlags(u32);

impl AnswerFlags {
    /// returns the number of active flags
    fn len(&self) -> usize {
        let mut size = 0;

        for i in 0..26 {
            let a = self.0 & (1 << i);
            let b = a >> i;

            size += b;
        }

        size.try_into().unwrap()
    }
}

impl FromStr for AnswerFlags {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        /// ascii offset such that 'a' is 0
        const ALPHA_OFFSET: u32 = 97;

        let len = value.len();
        if len > 26 {
            return Err(format!("Parse failure: too many answers ({})", len));
        }

        let mut result = 0;
        for character in value.chars() {
            if (character as u32) < 97 || (character as u32) > 122 {
                return Err(format!("Parse failue: invalid character '{}'", character));
            }

            result |= 1 << (character as u32 - ALPHA_OFFSET);
        }

        Ok(AnswerFlags(result))
    }
}

impl BitOr for AnswerFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for AnswerFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

/// combine [`AnswerFlags`] returning flags that are active in either argument
fn any(a: AnswerFlags, b: AnswerFlags) -> AnswerFlags {
    a | b
}

/// combine [`AnswerFlags`] returning flags that are active in both argument
fn all(a: AnswerFlags, b: AnswerFlags) -> AnswerFlags {
    a & b
}

fn count_answers<F>(mut combinator: F, file_path: &str) -> usize
where
    F: FnOnce(AnswerFlags, AnswerFlags) -> AnswerFlags
        + FnMut(AnswerFlags, AnswerFlags) -> AnswerFlags,
{
    read_file(file_path)
        // double newline between entries
        .split("\n\n")
        // parse answers for group members
        .map(|answer_group| {
            answer_group
                .lines()
                .map(|answers| answers.parse::<AnswerFlags>())
                .collect::<Result<Vec<_>, _>>()
        })
        // combine answers for each group
        .map(|group| {
            group
                .unwrap()
                .drain(..)
                .reduce(|a, b| combinator(a, b))
                .expect("Received an empty list")
        })
        // count unique answers
        .map(|flags| flags.len())
        .sum()
}

/// count unique answers in each group, then sum the counts
pub fn one(file_path: &str) -> usize {
    count_answers(any, file_path)
}

/// count questions where everyone answered yes in each group, then sum the counts
pub fn two(file_path: &str) -> usize {
    count_answers(all, file_path)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn flag_len() {
        let msg = "should count the number of active flags";
        let expected = 3;
        let actual = AnswerFlags(0b1110).len();
        assert_eq!(actual, expected, "{}", msg);

        let actual = AnswerFlags(0b100110).len();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn flag_parse() {
        let msg = "should parse a valid str to AnswerFlags";
        let expected = AnswerFlags(0b0111);
        let actual = "abc".parse::<AnswerFlags>().unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should sum the unique yes answers for each group";
        let expected = 11;
        let actual = one("input/6-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should sum the yes answers for each group";
        let expected = 6;
        let actual = two("input/6-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
