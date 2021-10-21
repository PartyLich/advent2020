//! Solutions to 2020 day 19 problems part 2
//! --- Day 19: Monster Messages ---
use super::*;

/// return the count of messages that match rule 0
pub fn two(file_path: &str) -> usize {
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
    fn part_two() {
        let msg = "should count the number of messages that match rule 0";
        let expected = 12;
        let actual = two("input/19-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
