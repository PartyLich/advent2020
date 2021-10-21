//! Solutions to 2020 day 19 problems
//! --- Day 19: Monster Messages ---
use parser::Parser;

use crate::day_1::read_file;

type Rule<'a> = Parser<'a, String>;


fn parse_rules<'a>(rule_strs: &str) -> Result<Vec<Rule<'a>>, String> {
    todo!();
}

/// return Some if the message meets the Rule, None otherwise
fn meets_rule<'a>(rule: Rule<'a>) -> impl Fn(&'a str) -> Option<&str> {
    move |message| {
        let (remaining, _result) = rule.parse(message).ok()?;
        if !remaining.is_empty() {
            return None;
        }
        Some(message)
    }
}

/// return the count of messages that match rule 0
pub fn one(file_path: &str) -> usize {
    let input = read_file(file_path);
    let (rules_str, messages_str) = input.split_once("\n\n").expect("Unable to parse input");
    let rules = parse_rules(rules_str).expect("Unable to parse rules");
    messages_str
        .lines()
        .filter_map(meets_rule(rules[0].clone()))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should count the number of messages that match rule 0";
        let expected = 2;
        let actual = one("input/19-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
