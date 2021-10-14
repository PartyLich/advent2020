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

fn find_timestamp(schedule_str: &str) -> usize {
    let bus_schedules = parse_csv::<usize>(schedule_str);

    let (max_idx, max) = bus_schedules
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap();
    let max = max.unwrap();
    println!("max {:?}", max);
    let first = bus_schedules.first().unwrap().unwrap();


    let mut t = max;
    let mut t_zero = t - max_idx;
    while (t_zero % first) != 0
        || bus_schedules
            .iter()
            .enumerate()
            .skip(1)
            .filter_map(|(idx, active)| {
                active.map(|id| {
                    println!("\t{} % {} == {} - {}", t_zero, id, id, idx);
                    println!("\t{} == {}", (t_zero) % id, id - idx);
                    (t_zero % id) == (id - idx)
                })
            })
            .any(|valid| !valid)
    {
        println!("t zero {}", t_zero);

        t += max;
        t_zero = t - max_idx;
    }
    println!("\t{} % {} == {}", t_zero, first, t_zero % first);

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
