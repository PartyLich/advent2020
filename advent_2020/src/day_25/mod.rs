//! Solutions to 2020 day 24 problems
//! --- Day 25: Combo Breaker ---
use crate::day_1::read_file;

/// transform a subject number
fn transform(subject: usize, value: Option<usize>) -> usize {
    let mut result = value.unwrap_or(1);

    // - Set the value to itself multiplied by the subject number.
    result *= subject;
    // - Set the value to the remainder after dividing the value by 20201227.
    result % 20201227
}

/// returns a fn that calculates loop size for a given target value, using the provided subject num
fn size_finder(subject: usize) -> impl FnMut(usize) -> usize {
    let mut cache = vec![];

    move |key| {
        let mut value = 1;

        for i in 0.. {
            if let Some(cached) = cache.get(i) {
                value = *cached;
            } else {
                value = transform(subject, Some(value));
                cache.push(value);
            }

            if value == key {
                return i + 1;
            }
        }

        panic!("Finished an infinite loop");
    }
}

/// returns the encryption key
pub fn one(file_path: &str) -> usize {
    let input = read_file(file_path);
    let mut pkey_iter = input
        .lines()
        .map(|line| usize::from_str_radix(line, 10).unwrap());
    let (card_pkey, door_pkey) = (pkey_iter.next().unwrap(), pkey_iter.next().unwrap());

    let mut loop_size = size_finder(7);
    let card_loop_size = loop_size(card_pkey);
    let door_loop_size = loop_size(door_pkey);

    // start with the value 1.
    // Then, a number of times called the loop size, transform the value
    let mut encryption_key = 1;
    if card_loop_size < door_loop_size {
        for _ in 0..card_loop_size {
            encryption_key = transform(door_pkey, Some(encryption_key));
        }
    } else {
        for _ in 0..door_loop_size {
            encryption_key = transform(card_pkey, Some(encryption_key));
        }
    }

    encryption_key
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transform_subject() {
        let msg = "should transform a subject number according to the encrpytion rules";
        let expected = 5764801;
        let actual = {
            let mut result = transform(7, None);
            for _ in 0..7 {
                result = transform(7, Some(result));
            }
            result
        };
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn find_loop_size() {
        let msg = "should calculate the loop size for a given key";
        let expected = 8;
        let mut loop_size = size_finder(7);
        let actual = loop_size(5764801);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 11;
        let mut loop_size = size_finder(7);
        let actual = loop_size(17807724);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return the encryption key";
        let expected = 14897079;
        let actual = one("input/25-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
