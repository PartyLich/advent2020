//! Solutions to 2020 day 10
//! --- Day 10: Adapter Array ---
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
}
