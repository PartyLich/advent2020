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

/// parse an Expression from a string
fn from_str(string: &str) -> Result<Expression, String> {
    todo!();
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
    fn part_one() {
        let msg = "should sum the result of each line";
        let expected = 26_457;
        let actual = one("input/18-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
