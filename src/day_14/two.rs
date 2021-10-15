//! Solutions to 2020 day 14 part two
//! --- Day 14: Docking Data ---
use std::collections::HashMap;
use std::str::FromStr;

use crate::day_1::read_file;

use super::{parse_mem_op, MemOp};

/// Operation to perform for a single bit of a bitmask
#[derive(Clone, Debug, PartialEq)]
enum MaskOp {
    // set a one bit at the contained offset
    One(usize),
    // set all possible values at the contained offset
    Float(usize),
}

// parse a [`MaskOp`] from a str
fn parse_op_str(value: &str, offset: usize) -> Result<MaskOp, String> {
    let result = match value {
        "X" => MaskOp::Float(offset),
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

// apply a bit mask to an address and return all resulting addresses
fn apply_mask(mask: &[MaskOp], address: usize) -> Vec<usize> {
    const ONE: usize = 1;
    let value = mask.iter().fold(address, |acc, op| match op {
        MaskOp::One(offset) => acc | (ONE << offset),
        _ => acc,
    });

    // recursive helper function to evaluate b tree branches
    fn helper(mask: &[MaskOp], address: usize) -> Vec<usize> {
        mask.iter()
            .enumerate()
            .find_map(|(idx, op)| match op {
                MaskOp::Float(offset) => {
                    // set all mem locations
                    let zero = address & !(ONE << offset);
                    let one = address | (ONE << offset);

                    // walk branches
                    let mut zeros = helper(&mask[(idx + 1)..], zero);
                    let mut ones = helper(&mask[(idx + 1)..], one);

                    zeros.append(&mut ones);
                    Some(zeros)
                }
                _ => None,
            })
            .unwrap_or_else(|| vec![address])
    }

    helper(mask, value)
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
pub fn two(file_path: &str) -> usize {
    let serialized = read_file(file_path);
    let mut mask: Mask = vec![];

    serialized
        .lines()
        .map(|line| line.parse().expect("Failed to parse instruction"))
        .fold(HashMap::new(), |mut memory, instruction| {
            match instruction {
                Instruction::Mask(new_mask) => {
                    mask = new_mask.to_vec();
                }
                Instruction::Memory(mem_op) => {
                    let addresses = apply_mask(&mask, mem_op.0);
                    addresses.into_iter().for_each(|address| {
                        memory.insert(address, mem_op.1);
                    });
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
    fn applies_mask() {
        let msg = "should return a list of addresses";
        let mask = parse_mask("000000000000000000000000000000X1001X");
        let expected = vec![26, 27, 58, 59];
        let actual = apply_mask(&mask, 42);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should sum the values in memory";
        let expected = 208;
        let actual = two("input/14-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
