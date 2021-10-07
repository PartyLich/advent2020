//! Solutions to 2020 day 4, part 2
//! --- Day 4: Passport Processing ---
use lazy_static::lazy_static;
use regex::Regex;

use super::{read_file, HashMap, KeyValue, Passport};

/// Vec<KeyValue> to Result<Passport> conversion with part two rules
///
/// It would go in the appropriate trait impl, but I did not want it to interfere with the part one
/// impl
fn try_from(list: Vec<KeyValue<'_>>) -> Result<Passport, &'static str> {
    if list.len() < 7 {
        return Err("Too many absent fields");
    }
    let mut map = HashMap::new();
    for (key, value) in &list {
        match *key {
            "byr" => {
                if BirthYear::new(value).is_none() {
                    return Err("Invalid field encountered");
                }
            }
            "iyr" => {
                if IssueYear::new(value).is_none() {
                    return Err("Invalid field encountered");
                }
            }
            "eyr" => {
                if ExpirationYear::new(value).is_none() {
                    return Err("Invalid field encountered");
                }
            }
            "hgt" => {
                if Height::new(value).is_none() {
                    return Err("Invalid field encountered");
                }
            }
            "hcl" => {
                if HairColor::new(value).is_none() {
                    return Err("Invalid field encountered");
                }
            }
            "ecl" => {
                if EyeColor::new(value).is_none() {
                    return Err("Invalid field encountered");
                }
            }
            "pid" => {
                if PassportID::new(value).is_none() {
                    return Err("Invalid field encountered");
                }
            }
            "cid" => {}
            _ => {}
        }
        map.insert(key.to_string(), value.to_string());
    }

    if map.len() == 7 && map.get("cid").is_some() {
        return Err("Too many absent fields");
    }

    Ok(Passport(map))
}

/// parse a year within the provided bounds
fn parse_year(value: &str, min: usize, max: usize) -> Option<usize> {
    if value.len() != 4 {
        return None;
    }

    value.parse::<usize>().ok().and_then(|year| {
        if year < min || year > max {
            return None;
        }
        Some(year)
    })
}

#[derive(Debug, PartialEq)]
struct BirthYear(usize);
impl BirthYear {
    pub fn new(value: &str) -> Option<Self> {
        const MIN: usize = 1920;
        const MAX: usize = 2002;
        parse_year(value, MIN, MAX).map(Self)
    }
}

#[derive(Debug, PartialEq)]
struct IssueYear(usize);
impl IssueYear {
    pub fn new(value: &str) -> Option<Self> {
        const MIN: usize = 2010;
        const MAX: usize = 2020;
        parse_year(value, MIN, MAX).map(Self)
    }
}

#[derive(Debug, PartialEq)]
struct ExpirationYear(usize);
impl ExpirationYear {
    pub fn new(value: &str) -> Option<Self> {
        const MIN: usize = 2020;
        const MAX: usize = 2030;
        parse_year(value, MIN, MAX).map(Self)
    }
}

#[derive(Debug, PartialEq)]
struct Height();
impl Height {
    pub fn new(value: &str) -> Option<Self> {
        lazy_static! {
            static ref POLICY_RE: Regex =
                Regex::new(r#"^(?P<height>\d{2,3})(?P<unit>(cm|in))$"#).unwrap();
        }
        POLICY_RE
            .captures(value)
            .and_then(|captures| {
                let height = captures.name("height")?.as_str();
                let unit = captures.name("unit")?.as_str();
                Some((height, unit))
            })
            .and_then(|(height, unit)| match unit {
                "in" => {
                    let height = height.parse::<usize>().ok()?;
                    if height < 59 || height > 76 {
                        None
                    } else {
                        Some(Self())
                    }
                }
                "cm" => {
                    let height = height.parse::<usize>().ok()?;
                    if height < 150 || height > 193 {
                        None
                    } else {
                        Some(Self())
                    }
                }
                _ => None,
            })
    }
}

#[derive(Debug, PartialEq)]
struct HairColor(String);
impl HairColor {
    pub fn new(value: &str) -> Option<Self> {
        lazy_static! {
            static ref POLICY_RE: Regex = Regex::new(r#"^#[0-9,a-f]{6}$"#).unwrap();
        }
        if POLICY_RE.is_match(value) {
            return Some(Self(value.to_string()));
        }

        None
    }
}

#[derive(Debug, PartialEq)]
struct EyeColor();
impl EyeColor {
    pub fn new(value: &str) -> Option<Self> {
        lazy_static! {
            static ref POLICY_RE: Regex =
                Regex::new(r#"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$"#).unwrap();
        }
        if POLICY_RE.is_match(value) {
            return Some(Self());
        }

        None
    }
}

#[derive(Debug, PartialEq)]
struct PassportID(String);
impl PassportID {
    pub fn new(value: &str) -> Option<Self> {
        lazy_static! {
            static ref POLICY_RE: Regex = Regex::new(r#"^\d{9}$"#).unwrap();
        }
        if POLICY_RE.is_match(value) {
            return Some(Self(value.to_string()));
        }

        None
    }
}

#[derive(Debug, PartialEq)]
struct CountryID();

pub fn two(file_path: &str) -> usize {
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
        .map(try_from)
        .filter_map(Result::ok)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn byr() {
        let msg = "should create a BirthYear";
        let expected = Some(BirthYear(2002));
        let actual = BirthYear::new("2002");
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a BirthYear";
        let expected = None;
        let actual = BirthYear::new("2003");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn hgt() {
        let msg = "should create a Height";
        let expected = Some(Height());
        let actual = Height::new("60in");
        assert_eq!(actual, expected, "{}", msg);
        let actual = Height::new("190cm");
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a Height";
        let expected = None;
        let actual = Height::new("190in");
        assert_eq!(actual, expected, "{}", msg);
        let actual = Height::new("190");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn hcl() {
        let msg = "should create a HairColor";
        let expected = Some(HairColor("#123abc".to_string()));
        let actual = HairColor::new("#123abc");
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a HairColor";
        let expected = None;
        let actual = HairColor::new("#123abz");
        assert_eq!(actual, expected, "{}", msg);
        let actual = HairColor::new("123abc");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn ecl() {
        let msg = "should create a EyeColor";
        let expected = Some(EyeColor());
        let actual = EyeColor::new("brn");
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a EyeColor";
        let expected = None;
        let actual = EyeColor::new("wat");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn pid() {
        let msg = "should create a PassportID";
        let expected = Some(PassportID("000000001".to_string()));
        let actual = PassportID::new("000000001");
        assert_eq!(actual, expected, "{}", msg);

        let msg = "should fail to create a PassportID";
        let expected = None;
        let actual = PassportID::new("0123456789");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should count the number of valid passports, with optional cid";
        let expected = 4;
        let actual = two("input/4-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
