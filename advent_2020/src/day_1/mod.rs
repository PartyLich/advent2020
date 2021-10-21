//! Solutions to 2020 day 1 problems
use std::{fs, path::Path};

/// read the specified file at `file_path` into a `String`
///
/// Panic! on error
pub fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let display = path.display();

    fs::read_to_string(&path).unwrap_or_else(|why| panic!("couldnt open {}: {}", display, why))
}

/// summation target value
const TARGET: i32 = 2020;

/// find the two entries that sum to 2020 and then multiply those two numbers together
///
/// nb: does NOT handle bad data. assumes there is a valid answer
pub fn one(file_path: &str) -> i32 {
    let mut expenses = read_file(file_path)
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    // TODO: avoid this sort overhead with a less-bad algo
    expenses.sort_unstable();

    let mut s = 0;
    let mut e = expenses.len() - 1;
    let mut product = 0;
    while s != e {
        match expenses[s] + expenses[e] {
            sum if sum == TARGET => {
                product = expenses[s] * expenses[e];
                println!(
                    "day 1-1\n\ta: {}\n\tb: {}\n\tproduct: {}",
                    expenses[s], expenses[e], product
                );
                break;
            }
            sum if sum > TARGET => {
                e -= 1;
            }
            sum if sum < TARGET => {
                s += 1;
            }
            _ => {}
        }
    }

    product
}

/// find the three entries that sum to 2020 and then multiply those three numbers together
///
/// nb: does NOT handle bad data. assumes there is a valid answer; expect an infinite loop
/// otherwise
pub fn two(file_path: &str) -> i32 {
    let mut expenses = read_file(file_path)
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    // TODO: avoid this sort overhead with a less-bad algo
    expenses.sort_unstable();

    let mut ind = (0, 1, expenses.len() - 1);
    loop {
        match expenses[ind.0] + expenses[ind.1] + expenses[ind.2] {
            sum if sum == TARGET => {
                let product = expenses[ind.0] * expenses[ind.1] * expenses[ind.2];
                println!(
                    "day 1-2\n\ta: {}\n\tb: {}\n\tc: {}\n\tproduct: {}",
                    expenses[ind.0], expenses[ind.1], expenses[ind.2], product
                );
                break product;
            }
            sum if sum > TARGET => {
                ind.2 -= 1;
            }
            sum if sum < TARGET => {
                if (ind.1 - ind.0) > 1 {
                    // first pointer is a smaller increment, prefer moving it first
                    ind.0 += 1;
                } else {
                    // move second pointer if they are adjacent (ie a smaller increment is not
                    // possible)
                    ind.1 += 1;
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let expected = 514579;
        let actual = one("./input/1-t.txt");

        assert_eq!(actual, expected);
    }

    #[test]
    fn part_two() {
        let expected = 241861950;
        let actual = two("./input/1-t.txt");

        assert_eq!(actual, expected);
    }
}
