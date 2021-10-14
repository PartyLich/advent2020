//! Solutions to 2020 day 13
//! --- Day 13: Shuttle Search ---
use std::str::FromStr;

use crate::day_1::read_file;

/// parse a string of comma separated values into a `Vec` of `T`, discarding failures
fn parse_csv_lossy<T>(text: &str) -> Vec<T>
where
    T: FromStr,
{
    text.split(',')
        .filter_map(|text| T::from_str(text.trim()).ok())
        .collect()
}

/// return ID of the earliest bus you can take to the airport multiplied by the number of minutes
/// you'll need to wait for that bus
pub fn one(file_path: &str) -> u32 {
    let file_content = read_file(file_path);
    let (departure, bus_schedules) = file_content
        .split_once("\n")
        .expect("Unable to parse notes");
    let departure: u32 = departure.parse().expect("Unable to parse departure time");
    let bus_schedules = parse_csv_lossy(bus_schedules);

    let mut bus_schedules = bus_schedules
        .iter()
        .map(|id| {
            let wait_time = id - (departure % id);
            (id, wait_time)
        })
        .collect::<Vec<_>>();
    bus_schedules.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    bus_schedules
        .iter()
        .take(1)
        .map(|(id, wait_time)| *id * *wait_time)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return ID of the earliest bus you can take to the airport multiplied by the number of minutes you'll need to wait";
        let expected = 295;
        let actual = one("input/13-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
