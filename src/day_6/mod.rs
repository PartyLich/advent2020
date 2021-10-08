//! Solutions to 2020 day 6
//! --- Day 6: Custom Customs ---
use std::convert::TryInto;
use std::ops::BitOr;
use std::str::FromStr;

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

pub fn one(file_path: &str) -> usize {
    todo!();
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
}
