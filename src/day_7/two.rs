//! Solutions to 2020 day 7
//! --- Day 7: Handy Haversacks ---
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::day_1::read_file;

/// Luggage nesting graph child node
#[derive(Debug, PartialEq)]
struct Child {
    name: String,
    count: usize,
}

/// Airline luggage nesting rule
#[derive(Debug, PartialEq)]
struct Rule {
    /// bag/rule identifier
    pub name: String,
    /// children adjacency list
    pub children: Vec<Child>,
}

/// Parse [`Rule`]s from a single line &str
fn parse_rule(rule_str: &str) -> Rule {
    lazy_static! {
        static ref RULE_RE: Regex =
            Regex::new(r#"(?P<quant>\d+)? ?(?P<bag>\w+ \w+) bags?,?"#).unwrap();
    }

    let mut parent = Rule {
        name: Default::default(),
        children: vec![],
    };
    RULE_RE.captures_iter(rule_str).for_each(|cap| {
        let bag = cap.name("bag").expect("No bag found for rule").as_str();
        if bag == "no other" {
            return;
        }

        match cap.name("quant") {
            Some(quant) => {
                // child
                parent.children.push(Child {
                    name: bag.into(),
                    count: quant
                        .as_str()
                        .parse()
                        .expect("Failed to parse bag quantity"),
                });
            }
            None => {
                // parent
                parent.name = bag.into();
            }
        }
    });

    parent
}

/// Parse a map of [`Rules`] from a file at the provided path
fn parse_rule_map(file_path: &str) -> HashMap<String, Rule> {
    read_file(file_path)
        .lines()
        .map(parse_rule)
        .map(|rule| (rule.name.clone(), rule))
        .collect()
}

/// count all children for the given root [`Rule`] and rule map
fn count_children(rule_map: &HashMap<String, Rule>, rule: &Rule) -> usize {
    if rule.children.is_empty() {
        return 0;
    }

    rule.children
        .iter()
        .map(|child| {
            let count = child.count;
            let child_value = rule_map
                .get(&child.name)
                .map(|rule| count_children(rule_map, rule))
                .expect("Invalid rule map: Expected rule was not found");

            count + (child_value * count)
        })
        .sum()
}

/// count the number of bags descendant of a shiny gold root bag
pub fn two(file_path: &str) -> usize {
    let rule_map = parse_rule_map(file_path);
    let shiny_gold_rule = rule_map.get("shiny gold").unwrap();
    count_children(&rule_map, shiny_gold_rule)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_rule() {
        let msg = "should parse the rules in a line of text";
        let rule_str = "shiny gold bags contain 2 dark red bags.";
        let expected = Rule {
            name: "shiny gold".to_string(),
            children: vec![Child {
                name: "dark red".to_string(),
                count: 2,
            }],
        };
        let actual = parse_rule(rule_str);
        assert!(actual.eq(&expected), "{}", msg);

        let rule_str = "faded blue bags contain no other bags.";
        let expected = Rule {
            name: "faded blue".to_string(),
            children: vec![],
        };
        let actual = parse_rule(rule_str);
        assert!(actual.eq(&expected), "{}", msg);
    }

    #[test]
    fn counts_children() {
        let msg = "should count the number of contained bags";
        let rule_map = parse_rule_map("input/7-t2.txt");

        let rule = rule_map.get("dark blue").unwrap();
        let expected = 2;
        let actual = count_children(&rule_map, rule);
        assert_eq!(actual, expected, "{}", msg);

        let rule = rule_map.get("dark green").unwrap();
        let expected = 6;
        let actual = count_children(&rule_map, rule);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should calc how many children a shiny gold root has";
        let expected = 126;
        let actual = two("input/7-t2.txt");
        assert_eq!(actual, expected, "{}", msg);

        let expected = 32;
        let actual = two("input/7-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
