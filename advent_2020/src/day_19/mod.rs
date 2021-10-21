//! Solutions to 2020 day 19 problems
//! --- Day 19: Monster Messages ---
use parser::{
    and_then, between, keep_first, keep_second, one_or_more, optional, or_else, p_char,
    parse_lowercase, parse_number, sequence, Parser,
};

use crate::day_1::read_file;

mod two;
pub use two::two;

type Rule<'a> = Parser<'a, String>;

/// split block of rule strings into sorted list of (index, rule) pairs
fn format_rule_strs(input: &str) -> Result<Vec<(usize, &str)>, String> {
    let mut rule_strs: Vec<_> = input
        .lines()
        .map(|line| {
            let (idx, rule_str) = line
                .split_once(": ")
                .ok_or_else(|| format!("Unable to parse rule '{}'", line))?;
            let idx = idx
                .parse::<usize>()
                .map_err(|_| format!("Unable to parse rule '{}'", line))?;
            Ok((idx, rule_str))
        })
        .collect::<Result<_, String>>()?;
    // sort by rule index
    rule_strs.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    Ok(rule_strs)
}

/// parse a lowercase alpha char between quotes
fn quoted_char<'a>() -> Parser<'a, char> {
    let double_quote = p_char('"');
    between(double_quote.clone(), parse_lowercase(), double_quote)
}

/// parse rule lookup indices with optional alternate indexes
fn lookup_rule<'a>() -> Parser<'a, (Vec<usize>, Option<Vec<usize>>)> {
    // the humble space character
    let space = p_char(' ');
    // number possibly followed by a space
    let integer = parse_number::<usize>();
    let index = keep_first(integer, optional(space.clone()));
    // multiple indices
    let index_set = one_or_more(index);
    // second set of rule indices
    let alternate = keep_second(
        optional(and_then(p_char('|'), space.clone())),
        index_set.clone(),
    );

    and_then(index_set.clone(), optional(alternate))
}

fn parse_rules<'a>(rule_strs: &str) -> Result<Vec<Rule<'a>>, String> {
    let rule_strs = format_rule_strs(rule_strs)?;
    let mut rules: Vec<Option<Rule>> = vec![None; rule_strs.len()];

    fn helper<'a>(
        rule_strs: &[(usize, &str)],
        rules: &mut [Option<Rule<'a>>],
        idx: usize,
    ) -> Rule<'a> {
        match &rules[idx] {
            Some(p) => p.clone(),
            None => {
                if let Ok((_remaining, character)) = quoted_char().parse(rule_strs[idx].1) {
                    let rule = p_char(character).map(String::from);
                    rules[idx] = Some(rule.clone());
                    return rule;
                }
                if let Ok((_remaining, (indices, alt_indices))) =
                    lookup_rule().parse(rule_strs[idx].1)
                {
                    let first = indices
                        .iter()
                        .map(|idx| helper(rule_strs, rules, *idx))
                        .collect::<Vec<_>>();
                    let first = sequence(&first).map(|strings| strings.join(""));

                    if let Some(indices) = alt_indices {
                        let second = indices
                            .iter()
                            .map(|idx| helper(rule_strs, rules, *idx))
                            .collect::<Vec<_>>();
                        let second = sequence(&second).map(|strings| strings.join(""));
                        // combine parsers
                        return or_else(first, second);
                    }
                    return first;
                }
                panic!("Unrecognized rule format");
            }
        }
    }

    // parse all rules
    for idx in 0..rule_strs.len() {
        if rules[idx].is_some() {
            // rule already processed
            continue;
        }

        let rule = helper(&rule_strs, &mut rules, idx);
        rules[idx] = Some(rule);
    }

    rules
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "Failed to parse all rules".to_string())
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
