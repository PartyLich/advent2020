//! Solutions to 2020 day 12
//! --- Day 12: Rain Risk ---
use std::str::FromStr;

use crate::day_1::read_file;

/// Ship navigation instruction
#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (instruction, argument) = value.split_at(1);
        let argument: u32 = argument.parse().map_err(|err| {
            format!(
                "failed to parse '{}'\n\targument '{}': {:?}",
                value, argument, err
            )
        })?;

        match instruction {
            "N" => Ok(Self::North(argument)),
            "S" => Ok(Self::South(argument)),
            "E" => Ok(Self::East(argument)),
            "W" => Ok(Self::West(argument)),
            "L" => Ok(Self::Left(argument)),
            "R" => Ok(Self::Right(argument)),
            "F" => Ok(Self::Forward(argument)),
            _ => Err(format!("Unrecognized instruction: {}", instruction)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn change_heading(&self, degrees: u32) -> Self {
        // given the problem statement, assume turns are restricted to cardinal directions (thus
        // multiples of 90 degrees)
        match degrees {
            90 => match self {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
            180 => match self {
                Direction::North => Direction::South,
                Direction::South => Direction::North,
                Direction::East => Direction::West,
                Direction::West => Direction::East,
            },
            270 => match self {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
            _ => *self,
        }
    }
}

/// reads a newline separated list of [`Instruction`]s from a &str
fn deserialize(serialized: &str) -> Result<Vec<Instruction>, String> {
    serialized
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<_>, _>>()
}

/// return the manhattan distance from the start position
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deser_instruction() {
        let msg = "should deserialize an instruction list";
        let expected = vec![
            Instruction::Forward(10),
            Instruction::North(3),
            Instruction::Forward(7),
            Instruction::Right(90),
            Instruction::Forward(11),
        ];
        let input = read_file("input/12-t.txt");
        let actual = deserialize(&input).unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return the manhattan distance from the start position";
        let expected = 25;
        let actual = one("input/12-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
    // }
}
