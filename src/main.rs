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
    show!("13-1", "Bus Id * Wait time", "13-1", day_13::one);
    show!("13-2", "Earliest Timestamp", "13-1", day_13::two);
    show!("14-1", "Sum of mem values", "14-1", day_14::one);
    show!("14-2", "Sum of mem values", "14-1", day_14::two);
    show!("15-1", "2020th number spoken", "15-1", day_15::one);
    show!("15-2", "30_000_000th number spoken", "15-1", day_15::two);
    show!("16-1", "Ticket scanning error rate", "16-1", day_16::one);
    show!("16-2", "Product of departure fields", "16-1", day_16::two);
    show!("17-1", "Active cubes", "17-1", day_17::one);
    show!("17-2", "Active cubes", "17-1", day_17::two);
    show!("18-1", "Sum of expressions", "18-1", day_18::one);
}
