//! Solutions to 2020 day 7
//! --- Day 7: Handy Haversacks ---
use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

use crate::day_1::read_file;

mod two;
pub use two::two;

/// Airline luggage nesting rule
#[derive(Debug, PartialEq)]
struct Rule {
    /// bag/rule identifier
    pub name: String,
    /// ancestor adjacency list
    pub parents: HashSet<String>,
}

/// Parse [`Rule`]s from a single line &str
fn parse_rule(rule_str: &str) -> Vec<Rule> {
    lazy_static! {
        static ref RULE_RE: Regex =
            Regex::new(r#"(?P<quant>\d+)? ?(?P<bag>\w+ \w+) bags?,?"#).unwrap();
    }

    let mut parent = String::default();
    RULE_RE
        .captures_iter(rule_str)
        .filter_map(|cap| {
            let bag = cap.name("bag").expect("No bag found for rule").as_str();
            if bag == "no other" {
                return None;
            }

            match cap.name("quant") {
                Some(_quant) => {
                    // child
                    Rule {
                        name: bag.into(),
                        parents: [parent.clone()].iter().cloned().collect(),
                    }
                }
                None => {
                    // parent
                    parent = bag.into();
                    Rule {
                        name: parent.clone(),
                        parents: HashSet::new(),
                    }
                }
            }
            .into()
        })
        .collect()
}

/// Parse a map of [`Rules`] from a file at the provided path
fn parse_rule_map(file_path: &str) -> HashMap<String, Rule> {
    let mut rule_map: HashMap<String, Rule> = HashMap::new();
    read_file(file_path)
        .lines()
        .map(parse_rule)
        .flatten()
        .for_each(|rule| {
            // get existing data
            let (name, old_parents) = rule_map.remove_entry(&rule.name).map_or_else(
                || (rule.name.clone(), HashSet::new()),
                |(name, rule)| (name, rule.parents),
            );
            // merge parent sets
            let parents = &old_parents | &rule.parents;
            rule_map.insert(name.clone(), Rule { name, parents });
        });

    rule_map
}

/// return the set of all ancestors for the given parents and rule map
fn get_ancestors(rule_map: &HashMap<String, Rule>, parents: &HashSet<String>) -> HashSet<String> {
    if parents.is_empty() {
        return HashSet::default();
    }

    let ancestors = parents
        .iter()
        .map(|name| {
            rule_map
                .get(name)
                .map(|rule| get_ancestors(rule_map, &rule.parents))
                .unwrap_or_else(HashSet::default)
        })
        .reduce(|a, b| &a | &b)
        .unwrap_or_else(HashSet::default);

    parents | &ancestors
}

/// given a file containing luggage rules, returns the number of root bags that have shiny gold leaf
/// nodes
pub fn one(file_path: &str) -> usize {
    let rule_map = parse_rule_map(file_path);
    let parents = &rule_map.get("shiny gold").unwrap().parents;
    get_ancestors(&rule_map, parents).len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_rule() {
        let msg = "should parse the rules in a line of text";
        let rule_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let expected = vec![
            Rule {
                name: "light red".to_string(),
                parents: [].iter().cloned().collect(),
            },
            Rule {
                name: "bright white".to_string(),
                parents: ["light red".to_string()].iter().cloned().collect(),
            },
            Rule {
                name: "muted yellow".to_string(),
                parents: ["light red".to_string()].iter().cloned().collect(),
            },
        ];
        let actual = parse_rule(rule_str);
        assert!(actual.eq(&expected), "{}", msg);

        let rule_str = "faded blue bags contain no other bags.";
        let expected = vec![Rule {
            name: "faded blue".to_string(),
            parents: [].iter().cloned().collect(),
        }];
        let actual = parse_rule(rule_str);
        assert!(actual.eq(&expected), "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should calc how many root bags have shiny gold leaf nodes";
        let expected = 4;
        let actual = one("input/7-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
