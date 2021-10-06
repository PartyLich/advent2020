use advent_2020::*;

macro_rules! show {
    ($day: literal, $text: literal, $fn: path) => {
        println!(
            "Day {}:\n\t{}: {}",
            $day,
            $text,
            $fn(&format!("./input/{}.txt", $day))
        );
    };
}

fn main() {
    day_1::one("./input/1-1.txt");
    day_1::two("./input/1-1.txt");
    println!(
        "Day 2-1:\n\tValid passwords: {}",
        day_2::one("./input/2-1.txt")
    );
    println!(
        "Day 2-2:\n\tValid passwords: {}",
        day_2::two("./input/2-1.txt")
    );
    show!("3-1", "Trees encountered", day_3::one);
    show!("3-1", "Trees encountered product", day_3::two);
}
