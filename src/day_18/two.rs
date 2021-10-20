//! Solutions to 2020 day 18 part 2
//! --- Day 18: Operation Order ---
use crate::day_1::read_file;

use super::{evaluate, Expression, Op, Operand};

// recursive [`Expression`] parsing helper
fn parse_expr(mut lhs: Option<Operand>, string: &str) -> Result<(&str, Operand), String> {
    if string.is_empty() {
        return lhs
            .map(|lhs| (string, lhs))
            .ok_or_else(|| "Missing LHS operand".to_string());
    }

    let mut rhs: Option<Operand> = None;
    let mut op: Option<Op> = None;
    for (idx, character) in string.char_indices() {
        if let (Some(lhs), Some(op), Some(rhs)) = (&lhs, &op, &rhs) {
            return parse_expr(
                Some(Operand::Expr(Expression {
                    lhs: lhs.clone().into(),
                    rhs: rhs.clone().into(),
                    op: op.clone(),
                })),
                &string[idx..],
            );
        }

        match character {
            ')' => {
                // end of expression
                return lhs
                    .map(|lhs| (&string[idx + 1..], lhs))
                    .ok_or_else(|| "Missing LHS operand".to_string());
            }
            '(' => {
                // new expression
                let (remaining, expr) = parse_expr(None, &string[idx + 1..])?;

                if lhs.is_none() {
                    return parse_expr(Some(expr), remaining);
                }
                if rhs.is_none() {
                    return parse_expr(
                        Some(Operand::Expr(Expression {
                            lhs: lhs.unwrap().into(),
                            rhs: expr.into(),
                            op: op.unwrap(),
                        })),
                        remaining,
                    );
                }
            }
            character if character.is_digit(10) => {
                // part of a number
                // NOTE: input contains only 0-9
                let num = character.to_digit(10).unwrap() as usize;
                if lhs.is_none() {
                    lhs = Some(Operand::Number(num));
                } else if rhs.is_none() {
                    rhs = Some(Operand::Number(num));
                }
            }
            '*' => {
                op = Some(Op::Mult);

                // new expression
                let (remaining, expr) = parse_expr(None, &string[idx + 1..])?;

                // complete expression
                return Ok((
                    remaining,
                    Operand::Expr(Expression {
                        lhs: lhs.unwrap().into(),
                        rhs: expr.into(),
                        op: op.unwrap(),
                    }),
                ));
            }
            '+' => {
                op = Some(Op::Add);
            }
            _ => {}
        }
    }

    match op {
        None => lhs
            .map(|lhs| ("", lhs))
            .ok_or_else(|| "Missing LHS operand".to_string()),
        Some(op) => Ok((
            "",
            Operand::Expr(Expression {
                lhs: lhs.unwrap().into(),
                rhs: rhs.unwrap().into(),
                op,
            }),
        )),
    }
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
