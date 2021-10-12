//! Solutions to 2020 day 10
//! --- Day 10: Adapter Array ---
use std::collections::HashMap;

use crate::{day_1::read_file, day_9::parse_numbers};

/// count of 1, 2, and 3 jolt differences
#[derive(Debug, Default)]
struct Differences(pub usize, pub usize, pub usize);

/// returns the number of 1, 2, and 3 jolt differences, given a sorted list of joltage ratings
fn count_differences(series: &[usize]) -> Differences {
    series
        .iter()
        .skip(1)
        .enumerate()
        .fold(Differences::default(), |mut acc, (idx, next)| {
            let diff = next - series[idx];
            match diff {
                1 => {
                    acc.0 += 1;
                }
                2 => {
                    acc.1 += 1;
                }
                3 => {
                    acc.2 += 1;
                }
                _ => panic!("Joltage difference limit exceeded"),
            }

            acc
        })
}

/// returns he number of 1-jolt differences multiplied by the number of 3-jolt
/// differences
pub fn one(file_path: &str) -> usize {
    const PORT_JOLTAGE: usize = 0;

    // get adapter data
    let input = read_file(file_path);
    let mut adapters = parse_numbers(&input).unwrap();
    adapters.sort_unstable();
    // add port joltage to head of list
    let mut joltages = vec![PORT_JOLTAGE];
    joltages.append(&mut adapters);

    let differences = count_differences(&joltages);
    // add 1 to the 3 joltage difference count for the device's difference
    differences.0 * (differences.2 + 1)
}

/// returns true if diff is within the acceptable joltage difference range
fn valid_difference(diff: usize) -> bool {
    (0..=3).contains(&diff)
}

/// returns the number of valid adapter options for the first item in the provided list
fn count_options(adapters: &[usize]) -> usize {
    adapters
        .get(0)
        .map(|first_joltage| {
            (1..=3usize)
                .take_while(|i| {
                    // next adapter exists and is within the acceptable joltage range
                    adapters
                        .get(*i)
                        .and_then(|next_joltage| next_joltage.checked_sub(*first_joltage))
                        .and_then(|diff| valid_difference(diff).then(|| {}))
                        .is_some()
                })
                .count()
                .max(1)
        })
        .unwrap_or(0)
}

/// traverse a weird tree (ie I didnt store it in a sane tree/graph structure, so its kinda weird
/// to traverse) and count some things
fn traverse(series: &[usize]) -> usize {
    if series.is_empty() {
        return 0;
    }

    let mut cache = HashMap::new();

    // recursive fn with caching to avoid recalculating branches
    fn helper(cache: &mut HashMap<usize, usize>, series: &[usize]) -> usize {
        if let Some(val) = cache.get(&series[0]) {
            return *val;
        }

        let len = series.len();
        let mut i = 0;
        let mut options = count_options(&series[i..]);
        while options < 2 && i < len {
            i += 1;
            options = count_options(&series[i..]);
        }

        let result: usize = (1..=options)
            .map(|offset| helper(cache, &series[(i + offset)..]))
            .sum::<usize>()
            // return 1 if there are no options (ie we've reached a leaf)
            .max(1);
        cache.insert(series[0], result);

        result
    }

    helper(&mut cache, series)
}

/// returns the total number of distinct ways you can arrange the adapters to connect the charging
/// outlet to your device
pub fn two(file_path: &str) -> usize {
    const PORT_JOLTAGE: usize = 0;

    // get adapter data
    let input = read_file(file_path);
    let mut adapters = parse_numbers(&input).unwrap();
    adapters.sort_unstable();
    // add port joltage to head of list
    let mut joltages = vec![PORT_JOLTAGE];
    joltages.append(&mut adapters);

    traverse(&joltages)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should return the number of 1-jolt differences multiplied by the number of 3-jolt differences";
        let expected = 35;
        let actual = one("input/10-t.txt");
        assert_eq!(actual, expected, "{}", msg);

        let expected = 10 * 22;
        let actual = one("input/10-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn counts_options() {
        let msg = "should count the valid adapter options";
        let expected = 2;
        let data = vec![10, 11, 12, 15, 16, 19];
        let actual = count_options(&data);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 1;
        let data = vec![0, 3, 4];
        let actual = count_options(&data);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 1;
        let data = vec![12];
        let actual = count_options(&data);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device";
        let expected = 8;
        let actual = two("input/10-t.txt");
        assert_eq!(actual, expected, "{}", msg);

        let expected = 19208;
        let actual = two("input/10-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
