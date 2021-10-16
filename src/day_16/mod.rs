//! Solutions to 2020 day 16
//! --- Day 16: Ticket Translation ---
use std::ops::RangeInclusive;

use crate::day_1::read_file;
use crate::day_13::parse_csv_lossy;

type Ticket = Vec<u32>;

/// parse a set of comma-separated values into a list of Tickets
fn parse_tickets(input: &str) -> Vec<Ticket> {
    input.lines().skip(1).map(parse_csv_lossy).collect()
}

type Field = (String, Vec<RangeInclusive<u32>>);

/// parse list of fields that exist somewhere on the ticket
//
// rules for ticket fields specify a list of fields that exist somewhere on the ticket and the valid
// ranges of values for each field.
fn parse_fields(input: &str) -> Vec<Field> {
    input
        .lines()
        .map(|line| {
            let (name, ranges) = line
                .split_once(":")
                .unwrap_or_else(|| panic!("Failed to parse range '{}'", line));
            let ranges = ranges
                .split(" or ")
                .map(|range| {
                    let (min, max) = range
                        .trim()
                        .split_once("-")
                        .unwrap_or_else(|| panic!("Failed to parse range '{}'", line));
                    let min = min.parse::<u32>().unwrap();
                    let max = max.parse::<u32>().unwrap();

                    min..=max
                })
                .collect::<Vec<_>>();
            (name.to_string(), ranges)
        })
        .collect()
}

struct TicketInfo {
    /// rules for ticket fields
    fields: Vec<Field>,
    /// the numbers on your ticket
    own_ticket: Ticket,
    /// the numbers on other nearby tickets
    nearby_tickets: Vec<Ticket>,
}

fn parse_input(input: &str) -> TicketInfo {
    let parts: Vec<_> = input.split("\n\n").collect();
    let fields = parse_fields(parts[0]);
    let own_ticket = parse_tickets(parts[1]).first().unwrap().to_vec();
    let nearby_tickets = parse_tickets(parts[2]);

    TicketInfo {
        fields,
        own_ticket,
        nearby_tickets,
    }
}

/// return the invalid values (values that are not in any of the specified ranges) in a list of
/// tickets
fn get_nearby_errors(fields: &[RangeInclusive<u32>], tickets: &[Ticket]) -> Vec<u32> {
    tickets
        .iter()
        .flat_map(|ticket| {
            ticket.iter().filter_map(|value| {
                if fields.iter().any(|range| range.contains(value)) {
                    return None;
                }
                Some(*value)
            })
        })
        .collect()
}

/// return the sum of the invalid values on nearby tickets, ignoring your own
pub fn one(file_path: &str) -> u32 {
    let contents = read_file(file_path);
    // parse file
    let input = parse_input(&contents);
    let fields = input
        .fields
        .iter()
        // discard unused data
        .flat_map(|(_name, ranges)| ranges)
        .cloned()
        .collect::<Vec<_>>();

    // error check nearby tickets
    get_nearby_errors(&fields, &input.nearby_tickets)
        .iter()
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg =
            "should return the sum of the invalid values on nearby tickets, ignoring your own";
        let expected = 71;
        let actual = one("input/16-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
