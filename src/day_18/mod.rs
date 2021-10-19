//! Solutions to 2020 day 18 problems
//! --- Day 18: Operation Order ---
use crate::day_1::read_file;

/// Value that can be used with an [operation](Op)
#[derive(Debug, PartialEq, Clone)]
enum Operand {
    /// parenthesized sub expression
    Expr(Expression),
    /// number
    Number(u32),
}

/// Operation that can be applied to two numbers
#[derive(Debug, PartialEq, Clone)]
enum Op {
    /// addition
    Add,
    /// multiplication
    Mult,
}

impl Op {
    /// apply this operation to the supplied operands
    pub fn apply(&self, left: u32, right: u32) -> u32 {
        match self {
            Self::Mult => left * right,
            Self::Add => left + right,
        }
    }
}

/// [Operands](Operand) and [Operations](Op) that form a single value
#[derive(Debug, PartialEq, Clone)]
struct Expression {
    /// right hand side
    rhs: Box<Operand>,
    /// operation
    op: Op,
    /// left hand side
    lhs: Box<Operand>,
}

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
                let num = character.to_digit(10).unwrap();
                if lhs.is_none() {
                    lhs = Some(Operand::Number(num));
                } else if rhs.is_none() {
                    rhs = Some(Operand::Number(num));
                }
            }
            '*' => {
                op = Some(Op::Mult);
            }
            '+' => {
                op = Some(Op::Add);
            }
            _ => {}
        }
    }

    Ok((
        "",
        Operand::Expr(Expression {
            lhs: lhs.unwrap().into(),
            rhs: rhs.unwrap().into(),
            op: op.unwrap(),
        }),
    ))
}

/// parse an [`Expression`] from a string
fn from_str(string: &str) -> Result<Expression, String> {
    let (_remaining, expr) = parse_expr(None, string)?;
    match expr {
        Operand::Expr(result) => Ok(result),
        _ => Err("Should be unreachable".to_string()),
    }
}

/// Return the result of evaluating an expression
fn evaluate(expr: Expression) -> u32 {
    let left = match *expr.lhs {
        Operand::Number(num) => num,
        Operand::Expr(expression) => evaluate(expression),
    };
    let right = match *expr.rhs {
        Operand::Number(num) => num,
        Operand::Expr(expression) => evaluate(expression),
    };

    expr.op.apply(left, right)
}

/// return the sum of the expressions on each line
pub fn one(file_path: &str) -> u32 {
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
    fn ops_add() {
        let msg = "should sum the operands";
        let expected = 69;
        let actual = Op::Add.apply(33, 36);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn ops_mult() {
        let msg = "should multiply the operands";
        let expected = 69;
        let actual = Op::Mult.apply(23, 3);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn evaluates_expression() {
        let msg = "should return the value of the expression after evaluation";
        let expected = 7;
        let rhs = Expression {
            lhs: Operand::Number(2).into(),
            rhs: Operand::Number(3).into(),
            op: Op::Mult,
        };
        let expression = Expression {
            lhs: Operand::Number(1).into(),
            op: Op::Add,
            rhs: Operand::Expr(rhs).into(),
        };
        let actual = evaluate(expression);
        assert_eq!(actual, expected, "{}", msg);

        let expected = 21;
        let expression = Expression {
            lhs: Operand::Expr(Expression {
                lhs: Operand::Number(2).into(),
                rhs: Operand::Number(4).into(),
                op: Op::Add,
            })
            .into(),
            op: Op::Add,
            rhs: Operand::Expr(Expression {
                lhs: Operand::Number(6).into(),
                rhs: Operand::Number(9).into(),
                op: Op::Add,
            })
            .into(),
        };
        let actual = evaluate(expression);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn parses() {
        let msg = "should parse an expression";
        let expected = Expression {
            lhs: Operand::Number(1).into(),
            op: Op::Add,
            rhs: Operand::Number(3).into(),
        };
        let actual = from_str("1 + 3").unwrap();
        assert_eq!(actual, expected, "{}", msg);

        let rhs = Expression {
            lhs: Operand::Number(2).into(),
            rhs: Operand::Number(3).into(),
            op: Op::Mult,
        };
        let expected = Expression {
            lhs: Operand::Number(1).into(),
            op: Op::Add,
            rhs: Operand::Expr(rhs).into(),
        };
        let actual = from_str("1 + (2 * 3)").unwrap();
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should sum the result of each line";
        let expected = 26_457;
        let actual = one("input/18-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
