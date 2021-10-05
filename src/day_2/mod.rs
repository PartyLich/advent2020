//! Solutions to 2020 day 2 problems
//!
//! Option and `unwrap()` used for simplicity. `Result`s and error handling would obviously be more
//! robust
use lazy_static::lazy_static;
use regex::Regex;

use crate::day_1::read_file;

/// Password policy interface. Provides behavior to validate compliance with this policy
trait PasswordPolicy {
    /// Returns true if the supplied password complies with this policy
    fn is_valid(&self, password: &str) -> bool;
}

/// returns the first [`char`] (unicode scalar value, not grapheme cluster) in the provided [`str`]
/// will panic on an empty str
fn first_char(ch: &str) -> char {
    ch.as_bytes()[0].into()
}

/// password policy indicates the lowest and highest number of times a given letter must appear for
/// the password to be valid.
#[derive(Debug, PartialEq)]
struct Policy {
    pub min: u32,
    pub max: u32,
    pub letter: char,
}

impl Policy {
    /// attempt to parse a [`Policy`] from the provided string
    pub fn new(policy_string: &str) -> Option<Self> {
        lazy_static! {
            static ref POLICY_RE: Regex =
                Regex::new(r#"(?P<min>\d*)-(?P<max>\d*)\s(?P<letter>\w)"#).unwrap();
        }

        POLICY_RE
            .captures(policy_string)
            .and_then(|captures| {
                // try to get named groups
                vec![
                    captures.name("min"),
                    captures.name("max"),
                    captures.name("letter"),
                ]
                .drain(0..)
                // stand-in for Sequence
                .collect::<Option<Vec<_>>>()
            })
            // convert to str
            .map(|matches| matches.iter().map(|s| s.as_str()).collect())
            // parse numbers
            .map(|matches: Vec<&str>| {
                let min = matches[0].parse::<u32>().expect("unable to parse min");
                let max = matches[1].parse::<u32>().expect("unable to parse max");
                let letter = matches[2];
                (min, max, letter).into()
            })
    }
}

impl From<(u32, u32, &str)> for Policy {
    fn from(other: (u32, u32, &str)) -> Self {
        Self {
            min: other.0,
            max: other.1,
            letter: first_char(other.2),
        }
    }
}

impl PasswordPolicy for Policy {
    fn is_valid(&self, password: &str) -> bool {
        let count = password.matches(self.letter).count();
        count >= self.min as usize && count <= self.max as usize
    }
}

#[derive(Debug, PartialEq)]
struct Password(String);

impl Password {
    /// return a new Password instance if the supplied password meets the Policy
    pub fn new(policy: &impl PasswordPolicy, password: &str) -> Option<Self> {
        if policy.is_valid(password) {
            return Some(Password(String::from(password)));
        }

        None
    }
}

/// Read a list of policy: password combinations and return how many passwords are valid according
/// to their policies
///
/// nb: does NOT handle bad data.
pub fn one(file_path: &str) -> usize {
    read_file(file_path)
        .lines()
        .filter_map(|line| {
            // split policy and password. panic on bad data
            let (policy, password) = line.split_once(':').expect("Unable to find delimiter ':'");
            let policy = Policy::new(policy).expect("Unable to parse valid Policy");
            Password::new(&policy, password)
        })
        .count()
}

/// Each policy describes two positions in the password, `first` and `second`
/// Exactly one of these positions must contain the given letter.
/// Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
#[derive(Debug, PartialEq)]
struct PolicyTwo {
    pub first: usize,
    pub second: usize,
    pub letter: char,
}

impl PolicyTwo {
    /// attempt to parse a [`PolicyTwo`] from the provided string
    pub fn new(policy_string: &str) -> Option<Self> {
        Policy::new(policy_string).map(From::from)
    }
}

impl From<Policy> for PolicyTwo {
    fn from(policy: Policy) -> Self {
        // Serialized policies are indexed with the first element as 1
        Self {
            first: policy.min as usize - 1,
            second: policy.max as usize - 1,
            letter: policy.letter,
        }
    }
}

impl PasswordPolicy for PolicyTwo {
    fn is_valid(&self, password: &str) -> bool {
        let first = password.get(self.first..=self.first).map(first_char);
        let second = password.get(self.second..=self.second).map(first_char);

        (first == Some(self.letter)) ^ (second == Some(self.letter))
    }
}

/// Read a list of policy: password combinations and return how many passwords are valid according
/// to their policies
///
/// nb: does NOT handle bad data.
pub fn two(file_path: &str) -> usize {
    read_file(file_path)
        .lines()
        .filter_map(|line| {
            // split policy and password. panic on bad data
            let (policy, password) = line.split_once(':').expect("Unable to find delimiter ':'");
            let policy = PolicyTwo::new(policy).expect("Unable to parse valid PolicyTwo");
            Password::new(&policy, password.trim())
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_policy() {
        let msg = "should create a Policy";
        let expected = Some(Policy {
            min: 1,
            max: 3,
            letter: 'a',
        });
        let actual = Policy::new("1-3 a");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn new_password() {
        let msg = "should create a Password";
        let expected = Some(Password(String::from("abcde")));
        let actual = Password::new(
            &Policy {
                min: 1,
                max: 3,
                letter: 'a',
            },
            "abcde",
        );
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a Password";
        let expected = None;
        let actual = Password::new(
            &Policy {
                min: 1,
                max: 3,
                letter: 'b',
            },
            "cdefg",
        );
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return 2";
        let expected = 2;
        let actual = one("./input/2-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn new_policy_two() {
        let msg = "should create a PolicyTwo";
        let expected = Some(PolicyTwo {
            first: 0,
            second: 2,
            letter: 'a',
        });
        let actual = PolicyTwo::new("1-3 a");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return 1";
        let expected = 1;
        let actual = two("./input/2-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
