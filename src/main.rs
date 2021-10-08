use advent_2020::*;

macro_rules! show {
    ($day: literal, $text: literal, $file: literal, $fn: path) => {
        println!(
            "Day {}:\n\t{}: {}",
            $day,
            $text,
            $fn(&format!("./input/{}.txt", $file))
        );
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
}
