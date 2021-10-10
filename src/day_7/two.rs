//! Solutions to 2020 day 7
//! --- Day 7: Handy Haversacks ---
use lazy_static::lazy_static;
use regex::Regex;

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

/// count the number of bags descendant of a shiny gold root bag
pub fn two(file_path: &str) -> usize {
    todo!()
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
