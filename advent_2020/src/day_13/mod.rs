//! Solutions to 2020 day 13
//! --- Day 13: Shuttle Search ---
use std::str::FromStr;

use crate::day_1::read_file;

/// parse a string of comma separated values into a `Vec` of `T`, discarding failures
pub fn parse_csv_lossy<T>(text: &str) -> Vec<T>
where
    T: FromStr,
{
    text.split(',')
        .filter_map(|text| T::from_str(text.trim()).ok())
        .collect()
}

/// parse a string of comma separated values into a `Vec` of `Option<T>`
fn parse_csv<T>(text: &str) -> Vec<Option<T>>
where
    T: FromStr,
{
    text.split(',')
        .map(|text| T::from_str(text.trim()).ok())
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

/// find least common multiple
fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }

    let min = a.min(b);
    let max = a.max(b);
    if max % min == 0 {
        return max;
    }

    let mut lcm = max;
    while lcm % min != 0 {
        lcm += max;
    }

    lcm
}

fn find_timestamp(schedule_str: &str) -> usize {
    let bus_schedules = parse_csv::<usize>(schedule_str)
        .iter()
        .enumerate()
        .filter_map(|(idx, bus_id)| bus_id.and_then(|bus_id| Some((idx, bus_id))))
        .collect::<Vec<_>>();

    let (_, first) = bus_schedules.first().unwrap();

    let mut step = *first;
    let mut t_zero = *first;

    for (bus_idx, bus_id) in bus_schedules.iter().skip(1) {
        while (t_zero + bus_idx) % *bus_id != 0 {
            t_zero += step;
        }

        step = lcm(step, *bus_id);
    }

    t_zero
}

/// return the earliest timestamp such that all of the listed bus IDs depart at offsets
/// matching their positions in the list
pub fn two(file_path: &str) -> usize {
    let file_content = read_file(file_path);
    let (_, bus_schedules) = file_content
        .split_once("\n")
        .expect("Unable to parse notes");

    find_timestamp(bus_schedules)
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

    #[test]
    fn least_common_multiple() {
        let msg = "should return the least common multiple";
        let expected = 90;
        let actual = lcm(18, 30);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 84;
        let actual = lcm(21, 28);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 140;
        let actual = lcm(14, 20);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn finds_timestamp() {
        let msg = "should return the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list";
        let expected = 3417;
        let actual = find_timestamp("17,x,13,19");
        assert_eq!(actual, expected, "{}", msg);

        let expected = 754018;
        let actual = find_timestamp("67,7,59,61");
        assert_eq!(actual, expected, "{}", msg);

        let expected = 1261476;
        let actual = find_timestamp("67,7,x,59,61");
        assert_eq!(actual, expected, "{}", msg);

        let expected = 1202161486;
        let actual = find_timestamp("1789,37,47,1889");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the earliest timestamp such that all of the listed bus IDs depart at offsets matching their positions in the list";
        let expected = 1068781;
        let actual = two("input/13-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
