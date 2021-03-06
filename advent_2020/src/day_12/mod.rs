//! Solutions to 2020 day 12
//! --- Day 12: Rain Risk ---
use std::collections::VecDeque;
use std::str::FromStr;

use crate::day_1::read_file;

/// Ship navigation instruction
#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Move(Direction, u32),
    Turn(u32),
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
            "N" => Ok(Self::Move(Direction::North, argument)),
            "S" => Ok(Self::Move(Direction::South, argument)),
            "E" => Ok(Self::Move(Direction::East, argument)),
            "W" => Ok(Self::Move(Direction::West, argument)),
            "L" => Ok(Self::Turn(360 - argument)),
            "R" => Ok(Self::Turn(argument)),
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

impl Default for Direction {
    fn default() -> Self {
        Self::East
    }
}

/// reads a newline separated list of [`Instruction`]s from a &str
fn deserialize(serialized: &str) -> Result<Vec<Instruction>, String> {
    serialized
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<_>, _>>()
}

type Position = (i32, i32);

/// returns a new coordinate resulting from moving the supplied coordinate in the specified
/// direction and amount
fn move_position(pos: Position, direction: Direction, value: u32) -> Position {
    let value = value as i32;
    match direction {
        Direction::North => (pos.0, pos.1 + value),
        Direction::South => (pos.0, pos.1 - value),
        Direction::East => (pos.0 + value, pos.1),
        Direction::West => (pos.0 - value, pos.1),
    }
}

/// returns a new coordinate calculated from rotating the supplied coordinate clockwise around the
/// origin by the specified number of degrees
fn rotate_position(pos: Position, degrees: u32) -> Position {
    // given the problem statement, assume turns are restricted to cardinal directions (thus
    // multiples of 90 degrees)
    match degrees {
        90 => (pos.1, -pos.0),
        180 => (-pos.0, -pos.1),
        270 => (-pos.1, pos.0),
        _ => pos,
    }
}

struct Nav {
    /// List of navigation instructions
    instructions: VecDeque<Instruction>,
    /// Current position
    position: Position,
    /// Current direction of travel
    direction: Direction,
}

impl Nav {
    pub fn with_instructions(instructions: &[Instruction]) -> Self {
        Self {
            position: (0, 0),
            direction: Direction::East,
            instructions: instructions.iter().copied().collect(),
        }
    }

    /// Advance state by processing next instruction
    fn next(&mut self) -> Option<()> {
        if let Some(instruction) = self.instructions.pop_front() {
            match instruction {
                Instruction::Move(direction, value) => {
                    self.position = move_position(self.position, direction, value);
                }
                Instruction::Turn(degrees) => {
                    self.direction = self.direction.change_heading(degrees);
                }
                Instruction::Forward(value) => {
                    self.position = move_position(self.position, self.direction, value);
                }
            }
            return Some(());
        }

        None
    }
}

impl Iterator for Nav {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

/// return the manhattan distance from the start position
pub fn one(file_path: &str) -> usize {
    let file_content = read_file(file_path);
    let instructions = deserialize(&file_content).unwrap();
    let mut nav = Nav::with_instructions(&instructions);

    while nav.next().is_some() {}
    let (x, y) = nav.position;
    (x.abs() + y.abs()) as usize
}

#[derive(Debug, PartialEq)]
struct Nav2 {
    /// Current position
    position: Position,
    /// navigation waypoint relative to current position
    waypoint: Position,
}

impl Default for Nav2 {
    fn default() -> Self {
        Self {
            position: Default::default(),
            waypoint: (10, 1),
        }
    }
}

/// return the next [`Nav`](Nav2) state given an initial state and an [`Instruction`]
fn process_instruction(state: Nav2, instruction: &Instruction) -> Nav2 {
    let mut next = Nav2 { ..state };
    match instruction {
        Instruction::Move(direction, value) => {
            // move the waypoint in the given direction by the given value
            next.waypoint = move_position(next.waypoint, *direction, *value);
        }
        Instruction::Turn(degrees) => {
            // rotate the waypoint around the ship
            next.waypoint = rotate_position(next.waypoint, *degrees);
        }
        Instruction::Forward(value) => {
            // moves the ship to the waypoint value times
            next.position.0 += next.waypoint.0 * (*value as i32);
            next.position.1 += next.waypoint.1 * (*value as i32);
        }
    }

    next
}

/// return the manhattan distance from the start position
// behavior on structs is notably less flexible than pure functions with struct parameters
pub fn two(file_path: &str) -> usize {
    let file_content = read_file(file_path);
    let instructions = deserialize(&file_content).unwrap();
    let nav = instructions
        .iter()
        .fold(Nav2::default(), process_instruction);
    let (x, y) = nav.position;

    (x.abs() + y.abs()) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deser_instruction() {
        let msg = "should deserialize an instruction list";
        let expected = vec![
            Instruction::Forward(10),
            Instruction::Move(Direction::North, 3),
            Instruction::Forward(7),
            Instruction::Turn(90),
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

    #[test]
    fn part_two() {
        let msg = "should return the manhattan distance from the start position";
        let expected = 286;
        let actual = two("input/12-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
