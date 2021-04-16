//! Solutions to 2020 day 1 problems
use std::{fs, path::Path};

pub fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let display = path.display();

    fs::read_to_string(&path).unwrap_or_else(|why| panic!("couldnt open {}: {}", display, why))
}

/// find the two entries that sum to 2020 and then multiply those two numbers together
pub fn one(file_path: &str) -> i32 {
    const TARGET: i32 = 2020;
    let mut expenses = read_file(file_path)
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    // TODO: avoid this sort overhead with a less-bad algo
    expenses.sort();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let expected = 514579;
        let actual = one("./input/1-t.txt");

        assert_eq!(actual, expected);
    }
}
