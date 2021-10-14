use std::time::Instant;

use advent_2020::*;

macro_rules! show {
    ($day: literal, $text: literal, $file: literal, $fn: path) => {
        let start = Instant::now();
        let result = $fn(&format!("./input/{}.txt", $file));
        let dur = start.elapsed();
        println!("Day {}:\n\t{}: {} ({:?})", $day, $text, result, dur,);
    };
}

fn main() {
    day_1::one("./input/1-1.txt");
    day_1::two("./input/1-1.txt");
    show!("2-1", "Valid passwords", "2-1", day_2::one);
    show!("2-2", "Valid passwords", "2-1", day_2::two);
    show!("3-1", "Trees encountered", "3-1", day_3::one);
    show!("3-2", "Trees encountered product", "3-1", day_3::two);
    show!("4-1", "Valid passports", "4-1", day_4::one);
    show!("4-2", "Valid passports", "4-1", day_4::two);
    show!("5-1", "Highest seat ID", "5-1", day_5::one);
    show!("5-2", "Missing seat ID", "5-1", day_5::two);
    show!("6-1", "Answer counts", "6-1", day_6::one);
    show!("6-2", "Answer counts", "6-1", day_6::two);
    show!("7-1", "Valid shiny gold container count", "7-1", day_7::one);
    show!("7-2", "shiny gold descendant count", "7-1", day_7::two);
    show!("8-1", "Last accumulator", "8-1", day_8::one);
    show!("8-2", "Last accumulator", "8-1", day_8::two);
    show!("9-1", "First invalid number", "9-1", day_9::one);
    show!("9-2", "Encryption weakness", "9-1", day_9::two);
    show!("10-1", "Joltage count product", "10-1", day_10::one);
    show!("10-2", "Distinct adapter arrangements", "10-1", day_10::two);
    show!("11-1", "Occupied Seats", "11-1", day_11::one);
    show!("11-2", "Occupied Seats", "11-1", day_11::two);
    show!("12-1", "Manhattan distance", "12-1", day_12::one);
    show!("12-2", "Manhattan distance", "12-1", day_12::two);
}
