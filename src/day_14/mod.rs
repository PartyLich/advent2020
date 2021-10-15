//! Solutions to 2020 day 14
//! --- Day 14: Docking Data ---
use std::collections::HashMap;
use std::str::FromStr;

use crate::day_1::read_file;

/// Operation to perform for a single bit of a bitmask
#[derive(Clone, Debug, PartialEq)]
enum MaskOp {
    // set a one bit at the contained offset
    One(usize),
    // set a zero bit at the contained offset
    Zero(usize),
}

// parse a [`MaskOp`] from a str
fn parse_op_str(value: &str, offset: usize) -> Result<MaskOp, String> {
    let result = match value {
        "0" => MaskOp::Zero(offset),
        "1" => MaskOp::One(offset),
        _ => {
            return Err(format!("Parse failure: invalid character '{}'", value));
        }
    };

    Ok(result)
}

type Mask = Vec<MaskOp>;

// parse a [`Mask`] from a str
fn parse_mask(mask_str: &str) -> Mask {
    let mask_len = mask_str.len();
    mask_str
        .split("")
        .enumerate()
        .filter_map(|(idx, op_str)| {
            (!op_str.is_empty())
                .then(|| {})
                .and_then(|_| parse_op_str(op_str, mask_len - idx).ok())
        })
        .collect()
}

// apply a bit mask to a number and return the result
fn apply_mask(mask: &[MaskOp], value: usize) -> usize {
    const ONE: usize = 1;
    mask.iter().fold(value, |acc, op| match op {
        MaskOp::Zero(offset) => acc & !(ONE << offset),
        MaskOp::One(offset) => acc | (ONE << offset),
    })
}

type MemOp = (usize, usize);

// parse a [memory operation](MemOp) from a str
fn parse_mem_op(mem_str: &str) -> MemOp {
    let (mem_idx, value) = mem_str
        .split_once(" = ")
        .expect("Failed to parse instruction");
    let value = value.parse().expect("Failed to parse memory value");
    let (_, mem_idx) = mem_idx.split_at(4);
    let (mem_idx, _) = mem_idx.split_at(mem_idx.len() - 1);
    let mem_idx = mem_idx.parse().expect("Failed to parse memory index");

    (mem_idx, value)
}

/// Initialization program instruction
#[derive(Debug)]
enum Instruction {
    Memory(MemOp),
    Mask(Mask),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (instr_type, value) = line
            .split_once(" = ")
            .ok_or("Failed to parse instruction")?;
        if instr_type.starts_with("mas") {
            return Ok(Instruction::Mask(parse_mask(value)));
        }
        if instr_type.starts_with("mem") {
            return Ok(Instruction::Memory(parse_mem_op(line)));
        }

        Err("Failed to parse instruction".to_string())
    }
}

/// returns the sum of the values in memory after executing a the supplied initialization program
pub fn one(file_path: &str) -> usize {
    let serialized = read_file(file_path);
    let mut mask: Mask = vec![];

    serialized
        .lines()
        .map(|line| line.parse().expect("Failed to parse instruction"))
        // may not need all of the values in a contiguous array, so just map used indexes and values
        .fold(HashMap::new(), |mut memory, instruction| {
            match instruction {
                Instruction::Mask(new_mask) => {
                    mask = new_mask.to_vec();
                }
                Instruction::Memory(mem_op) => {
                    memory.insert(mem_op.0, apply_mask(&mask, mem_op.1));
                }
            }

            memory
        })
        .values()
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should sum the values in memory";
        let expected = 165;
        let actual = one("input/14-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
