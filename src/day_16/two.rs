//! Solutions to 2020 day 16 part 2
//! --- Day 16: Ticket Translation ---
use std::collections::{HashMap, HashSet};

use crate::day_1::read_file;

use super::{parse_input, Field, Ticket};

/// return the subset of tickets that conform to the provided field rules
fn discard_invalid_tickets(fields: &[Field], tickets: &[Ticket]) -> Vec<Ticket> {
    tickets
        .iter()
        .filter(|ticket| {
            // all values match at least one field range
            ticket.iter().all(|value| {
                fields
                    .iter()
                    .any(|field| field.1.iter().any(|range| range.contains(value)))
            })
        })
        .cloned()
        .collect()
}

/// return a map of field names to their ticket data index
fn map_fields(fields: &[Field], tickets: &[Ticket]) -> HashMap<String, usize> {
    let mut cache: HashMap<u32, HashSet<usize>> = HashMap::new();

    let mut matching_rules = tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .map(|value| {
                    if let Some(set) = cache.get(value) {
                        return set.clone();
                    }
                    let set = fields
                        .iter()
                        .enumerate()
                        .filter(|(_idx, (_name, ranges))| {
                            ranges.iter().any(|range| range.contains(value))
                        })
                        .map(|(field_idx, _)| field_idx)
                        .collect::<HashSet<usize>>();

                    cache.insert(*value, set.clone());
                    set
                })
                .collect::<Vec<_>>()
        })
        .reduce(|mut acc, ticket| {
            for idx in 0..acc.len() {
                acc[idx] = acc[idx].intersection(&ticket[idx]).copied().collect();
            }
            acc
        })
        .unwrap();

    let mut visited = HashSet::new();
    for rule_list in matching_rules
        .iter()
        .filter(|rule_list| rule_list.len() == 1)
    {
        visited = visited.union(rule_list).copied().collect();
    }

    while matching_rules.iter().any(|rule_list| rule_list.len() > 1) {
        for rule_list in matching_rules
            .iter_mut()
            .filter(|rule_list| rule_list.len() > 1)
        {
            *rule_list = rule_list.difference(&visited).copied().collect();
            if rule_list.len() == 1 {
                visited = visited.union(rule_list).copied().collect();
            }
        }
    }

    matching_rules
        .iter()
        // all of our sets should have a single element at this point
        .map(|set| set.iter().take(1).collect::<Vec<_>>()[0])
        .enumerate()
        .fold(HashMap::new(), |mut map, (idx, field_idx)| {
            let name = fields[*field_idx].0.clone();
            map.insert(name, idx);
            map
        })
}

///  return the product of the six fields that start with the word "departure".
pub fn two(file_path: &str) -> usize {
    let contents = read_file(file_path);
    let input = parse_input(&contents);
    let nearby_tickets = discard_invalid_tickets(&input.fields, &input.nearby_tickets);
    let field_map = map_fields(&input.fields, &nearby_tickets);

    field_map
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, idx)| input.own_ticket[*idx] as usize)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn discards_invalid_tickets() {
        let msg = "should return the subset of tickets that conform to the provided field rules";
        let expected = vec![vec![7, 3, 47]];

        let contents = read_file("input/16-t.txt");
        let input = parse_input(&contents);
        let actual = discard_invalid_tickets(&input.fields, &input.nearby_tickets);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn maps_fields() {
        let msg = "should map of field names to their ticket data index";
        let mut expected = HashMap::new();
        expected.insert("class".to_string(), 1);
        expected.insert("row".to_string(), 0);
        expected.insert("seat".to_string(), 2);

        let contents = read_file("input/16-t2.txt");
        let input = parse_input(&contents);
        let nearby_tickets = vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]];
        let actual = map_fields(&input.fields, &nearby_tickets);
        assert_eq!(actual, expected, "{}", msg);
    }
}
