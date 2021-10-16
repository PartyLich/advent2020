//! Solutions to 2020 day 16
//! --- Day 16: Ticket Translation ---

// input: rules for ticket fields,
//     the numbers on your ticket,
//     and the numbers on other nearby tickets

// rules for ticket fields specify a list of fields that exist somewhere on the ticket and the valid ranges of values for each field.
//
// Each ticket is represented by a single line of comma-separated values.

/// return the sum of the invalid values on nearby tickets, ignoring your own
pub fn one(file_path: &str) -> usize {
    todo!()
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
