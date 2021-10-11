//! Solutions to 2020 day 8
//! --- Day 8: Handheld Halting ---
use std::collections::HashSet;
use std::str::FromStr;

use crate::day_1::read_file;

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
    Nop(isize),
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
            "nop" => Ok(Self::Nop(argument)),
            _ => Err(format!("Unrecognized instruction: {}", instruction)),
        }
    }
}

/// a list of instructions that together comprise a program
type Program = Vec<Instruction>;

/// read a set of [`Instruction`]s from a file
fn read_program(file_path: &str) -> Program {
    read_file(file_path)
        .lines()
        .map(|instruction| instruction.parse().unwrap())
        .collect()
}

/// the state (registers?) of a simple computer system
#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct State {
    /// program counter
    pc: usize,
    /// accumulator
    acc: isize,
}

#[derive(Debug, PartialEq)]
enum Flag {
    Idle,
    Complete,
    Halted,
}

impl Default for Flag {
    fn default() -> Self {
        Flag::Idle
    }
}

/// a simple computer
#[derive(Debug, Default, PartialEq)]
struct Computer {
    state: State,
    /// a [Program] held in memory
    program: Program,
    /// previously visited memory locations
    visited: HashSet<usize>,
    /// flag set if program execution causes a halt
    flag: Flag,
}

impl Computer {
    /// Create an instance with the supplied program in memory
    pub fn with_program(program: Program) -> Self {
        Self {
            program,
            ..Default::default()
        }
    }

    // halt any further instruction execution
    fn halt(&mut self) {
        self.flag = Flag::Halted;
    }

    /// Execute the next instruction and return a copy of the next system state
    pub fn step(&mut self) -> Option<State> {
        if self.program.is_empty() || self.flag != Flag::Idle {
            return None;
        }
        // The program is supposed to terminate by attempting to execute an instruction immediately
        // after the last instruction in the file.
        if self.state.pc >= self.program.len() {
            self.flag = Flag::Complete;
            return None;
        }

        // update instruction visit list
        self.visited.insert(self.state.pc);
        // unsafe indexing. if it fails, its the other programmers fault, right?
        let instruction = self.program[self.state.pc];
        match instruction {
            Instruction::Acc(argument) => {
                let pc = self.state.pc + 1;
                if self.visited.contains(&pc) {
                    self.halt();
                    return None;
                }

                self.state = State {
                    pc,
                    acc: self.state.acc + argument,
                }
            }
            Instruction::Jmp(argument) => {
                // sketchy typecasting
                let pc = (self.state.pc as isize + argument) as usize;
                if self.visited.contains(&pc) {
                    self.halt();
                    return None;
                }

                self.state = State { pc, ..self.state }
            }
            Instruction::Nop(_) => {
                let pc = self.state.pc + 1;
                if self.visited.contains(&pc) {
                    self.halt();
                    return None;
                }

                self.state = State {
                    pc: self.state.pc + 1,
                    ..self.state
                }
            }
        }

        self.state.into()
    }

    /// Execute instructions until the computer halts, returning the last state
    pub fn run(&mut self) -> Option<State> {
        self.last()
    }
}

impl Iterator for Computer {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

/// return the accumulator value before repeating an instruction
pub fn one(file_path: &str) -> isize {
    let program = read_program(file_path);
    let mut computer = Computer::with_program(program);

    computer.run().map(|state| state.acc).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_instruction() {
        let msg = "should parse an Instruction from a valid str";
        let expected = Instruction::Nop(0);
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
