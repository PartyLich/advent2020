//! Solutions to 2020 day 18 part 2
//! --- Day 18: Operation Order ---
use crate::day_1::read_file;

use super::{evaluate, Expression, Op, Operand};

// recursive [`Expression`] parsing helper
fn parse_expr(mut lhs: Option<Operand>, string: &str) -> Result<(&str, Operand), String> {
    todo!();
}

/// parse an [`Expression`] from a string
fn from_str(string: &str) -> Result<Expression, String> {
    let (_remaining, expr) = parse_expr(None, string)?;
    match expr {
        Operand::Expr(result) => Ok(result),
        _ => Err("Should be unreachable".to_string()),
    }
}

/// return the sum of the expressions on each line
pub fn two(file_path: &str) -> usize {
    let input = read_file(file_path);
    input
        .lines()
        .map(|line| from_str(line).unwrap())
        .map(evaluate)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses() {
        let msg = "should parse an expression";
        let rhs = Expression {
            lhs: Operand::Number(1).into(),
            rhs: Operand::Number(3).into(),
            op: Op::Add,
        };
        let expected = Expression {
            lhs: Operand::Number(2).into(),
            op: Op::Mult,
            rhs: Operand::Expr(rhs).into(),
        };
        let actual = from_str("2 * 1 + 3").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let lhs = Expression {
            lhs: Operand::Number(1).into(),
            rhs: Operand::Number(2).into(),
            op: Op::Add,
        };
        let rhs = Expression {
            lhs: Operand::Number(3).into(),
            rhs: Operand::Number(4).into(),
            op: Op::Add,
        };
        let expected = Expression {
            lhs: Operand::Expr(lhs).into(),
            op: Op::Mult,
            rhs: Operand::Expr(rhs).into(),
        };
        let actual = from_str("1 + 2 * 3 + 4)").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should sum the result of each line";
        let expected = 694_173;
        let actual = two("input/18-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
