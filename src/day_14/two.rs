//! Solutions to 2020 day 14 part two
//! --- Day 14: Docking Data ---

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


/// returns the sum of the values in memory after executing a the supplied initialization program
pub fn two(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should sum the values in memory";
        let expected = 208;
        let actual = two("input/14-t2.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
