//! Solutions to 2020 day 4 problems
//! --- Day 4: Passport Processing ---
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::day_1::read_file;

/// a key value pair
type KeyValue<'a> = (&'a str, &'a str);

/// Passport data
#[derive(Debug, PartialEq)]
struct Passport(HashMap<String, String>);

impl TryFrom<Vec<KeyValue<'_>>> for Passport {
    type Error = &'static str;

    fn try_from(list: Vec<KeyValue<'_>>) -> Result<Self, Self::Error> {
        if list.len() < 7 {
            return Err("Too many absent fields");
        }
        let mut map = HashMap::new();
        for (key, value) in &list {
            map.insert(key.to_string(), value.to_string());
        }

        if list.len() == 7 && map.get("cid").is_some() {
            return Err("Too many absent fields");
        }

        Ok(Passport(map))
    }
}

/// attempt to deserialize [`Passport`]s from the supplied file path
fn read_passports(file_path: &str) -> Vec<Result<Passport, &'static str>> {
    read_file(file_path)
        // double newline between passport entries
        .split("\n\n")
        // create list of key:value pairs
        .map(|passport_str| {
            passport_str
                .split_whitespace()
                .map(|pair| pair.split_once(":").expect("Unable to find delimiter ':'"))
                .collect::<Vec<_>>()
        })
        .map(TryFrom::try_from)
        .collect()
}

// Count the number of valid passports - those that have all required fields.
// Treat cid as optional.
pub fn one(file_path: &str) -> usize {
    read_passports(file_path)
        .drain(..)
        .filter_map(Result::ok)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should count the number of valid passports, with optional cid";
        let expected = 2;
        let actual = one("input/4-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
