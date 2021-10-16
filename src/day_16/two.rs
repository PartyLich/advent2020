//! Solutions to 2020 day 16 part 2
//! --- Day 16: Ticket Translation ---
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

///  return the product of the six fields that start with the word "departure".
pub fn two(file_path: &str) -> usize {
    todo!()
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
}
