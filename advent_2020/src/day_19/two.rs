//! Solutions to 2020 day 19 problems part 2
//! --- Day 19: Monster Messages ---
use std::collections::HashMap;

use super::*;

/// parse rules 42 and 31, return a map containing them (and any necessary ancestors)
fn parse_rules<'a>(rule_strs: &str) -> Result<HashMap<usize, Rule<'a>>, String> {
    let rule_strs = format_rule_strs(rule_strs)?
        .into_iter()
        .collect::<HashMap<usize, &str>>();
    let mut rules: HashMap<usize, Rule> = HashMap::new();

    fn helper<'a>(
        rule_strs: &HashMap<usize, &str>,
        rules: &mut HashMap<usize, Rule<'a>>,
        idx: usize,
    ) -> Rule<'a> {
        match rules.get(&idx) {
            Some(p) => p.clone(),
            None => {
                if let Ok((_remaining, character)) =
                    quoted_char().parse(rule_strs.get(&idx).unwrap())
                {
                    let rule = p_char(character).map(String::from);
                    rules.insert(idx, rule.clone());
                    return rule;
                }
                if let Ok((_remaining, (indices, alt_indices))) =
                    lookup_rule().parse(rule_strs.get(&idx).unwrap())
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

    let rule_42 = helper(&rule_strs, &mut rules, 42);
    rules.insert(42, rule_42.clone());
    let rule_31 = helper(&rule_strs, &mut rules, 31);
    rules.insert(31, rule_31.clone());

    Ok(rules)
}

/// manually evaluates rule zero, returning Some if the message meets the Rule, None otherwise
// we now have loops, but only in rule 8 and 11,
// and only in the alternate section
// rule 8: 42 | 42 8
//    one or more rule 42
// rule 11: 42 31 | 42 11 31
//    one or more 42 and then  one or more 31
//    eg 42 11 31
//       42 (42 11 31) 31
//       42 42 42 31 31 31
fn meets_rule<'a>(rules: HashMap<usize, Rule<'a>>) -> impl Fn(&'a str) -> Option<&str> {
    move |message| {
        let rule_42 = rules.get(&42).unwrap().clone();
        let rule_31 = rules.get(&31).unwrap().clone();

        // check for first 42, ie base rule 8. exit if no match
        let (mut remaining, _result) = rule_42.parse(message).ok()?;
        let mut count_42 = 1;
        // parse alternate 8 branch, ie many 42
        while let Ok((next, _result)) = rule_42.parse(remaining) {
            remaining = next;
            count_42 += 1;
        }

        // check valid 11. the previous loop consumed our initial 42,
        let (mut remaining, _result) = rule_31.parse(remaining).ok()?;
        let mut count_31 = 1;

        // base rule 11
        if remaining.is_empty() && (count_42 > count_31) {
            return Some(message);
        }

        // alt rule 11
        while let Ok((next, _result)) = rule_31.parse(remaining) {
            remaining = next;
            count_31 += 1;
        }

        if remaining.is_empty() && (count_42 > count_31) {
            Some(message)
        } else {
            None
        }
    }
}

/// return the count of messages that match rule 0
pub fn two(file_path: &str) -> usize {
    let input = read_file(file_path);
    let (rules_str, messages_str) = input.split_once("\n\n").expect("Unable to parse input");
    let rules = parse_rules(rules_str).expect("Unable to parse rules");

    messages_str.lines().filter_map(meets_rule(rules)).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_rules() {
        let msg = "should parse rule 42 and 31";

        let input = read_file("input/19-t2.txt");
        let (rules_str, _) = input.split_once("\n\n").expect("Unable to parse input");
        let rules = parse_rules(rules_str).expect("Unable to parse rules");
        let rule_42 = rules.get(&42).unwrap().clone();
        let rule_31 = rules.get(&31).unwrap().clone();

        let expected = ("bbaabaabba", "bbabb".to_string());
        let actual = rule_42.parse("bbabbbbaabaabba").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("aabba", "bbaab".to_string());
        let actual = rule_42.parse("bbaabaabba").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("", "aabba".to_string());
        let actual = rule_31.parse("aabba").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("Cbaabbbbaab", "bbaba".to_string());
        let actual = rule_31.parse("bbabaCbaabbbbaab").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = ("Cbaabbbbaab", "ababb".to_string());
        let actual = rule_31.parse("ababbCbaabbbbaab").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should count the number of messages that match rule 0";
        let expected = 12;
        let actual = two("input/19-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
