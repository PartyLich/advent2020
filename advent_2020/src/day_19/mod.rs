//! Solutions to 2020 day 19 problems
//! --- Day 19: Monster Messages ---
use parser::{
    and_then, between, keep_first, keep_second, one_or_more, optional, p_char, parse_lowercase,
    parse_number, Parser,
};

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
    fn parsers() {
        let msg = "should parse rule patterns";

        let space = p_char(' ');
        let integer = parse_number::<usize>();
        let index = keep_first(integer, optional(space.clone()));
        let index_set = one_or_more(index);

        let double_quote = p_char('"');
        let quoted_char = between(double_quote.clone(), parse_lowercase(), double_quote);

        let expected = ("", 'a');
        let actual = quoted_char.parse("\"a\"").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("", vec![4, 1, 25]);
        let actual = index_set.parse("4 1 25").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("| 3 2", vec![30, 2]);
        let actual = index_set.parse("30 2 | 3 2").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let alternate = keep_second(
            optional(and_then(p_char('|'), space.clone())),
            index_set.clone(),
        );
        let lookup_rule = and_then(index_set, optional(alternate));
        let expected = ("", (vec![30, 2], Some(vec![3, 20])));
        let actual = lookup_rule.parse("30 2 | 3 20").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should count the number of messages that match rule 0";
        let expected = 2;
        let actual = one("input/19-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
