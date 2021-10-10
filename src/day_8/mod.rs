//! Solutions to 2020 day 8
//! --- Day 8: Handheld Halting ---
use std::str::FromStr;

/// simple computer operating instruction
#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    /// increases or decreases the accumulator by the value given in the argument.
    ///
    /// For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After
    /// an acc instruction, the instruction immediately below it is executed next.
    Acc(isize),
    /// jumps to a new instruction relative to itself.
    ///
    /// The next instruction to execute is found using the argument as an offset from the jmp
    /// instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to
    /// the instruction immediately below it, and jmp -20
    /// would cause the instruction 20 lines above to be executed next.
    Jmp(isize),
    /// No OPeration - it does nothing.
    ///
    /// The instruction immediately below it is executed next.
    Nop(),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (instruction, argument) = value
            .split_once(" ")
            .ok_or("Instruction parse failure: missing separator")?;
        let argument: isize = argument
            .parse()
            .map_err(|err| format!("failed to parse argument '{}': {:?}", argument, err))?;

        match instruction {
            "acc" => Ok(Self::Acc(argument)),
            "jmp" => Ok(Self::Jmp(argument)),
            "nop" => Ok(Self::Nop()),
            _ => Err(format!("Unrecognized instruction: {}", instruction)),
        }
    }
}


/// return the accumulator value before looping
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_instruction() {
        let msg = "should parse an Instruction from a valid str";
        let expected = Instruction::Nop();
        let actual: Instruction = "nop +0".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = Instruction::Jmp(-3);
        let actual: Instruction = "jmp -3".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let expected = Instruction::Acc(-99);
        let actual: Instruction = "acc -99".parse().unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return the accumulator value before looping";
        let expected = 5;
        let actual = one("input/8-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
