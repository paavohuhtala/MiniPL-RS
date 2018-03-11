use common::types::*;
use common::types::LiteralValue::*;

use parsing::ast::*;
use parsing::ast::Expression::*;
use parsing::ast::Statement::*;

pub fn add(a: Expression, b: Expression) -> Expression {
  Add(Box::new((a, b)))
}

pub fn sub(a: Expression, b: Expression) -> Expression {
  Sub(Box::new((a, b)))
}

pub fn mul(a: Expression, b: Expression) -> Expression {
  Mul(Box::new((a, b)))
}

pub fn div(a: Expression, b: Expression) -> Expression {
  Div(Box::new((a, b)))
}

pub fn eq(a: Expression, b: Expression) -> Expression {
  Equal(Box::new((a, b)))
}

pub fn lt(a: Expression, b: Expression) -> Expression {
  LessThan(Box::new((a, b)))
}

pub fn and(a: Expression, b: Expression) -> Expression {
  And(Box::new((a, b)))
}

pub fn int(i: i32) -> Expression {
  Literal(IntLiteral(i))
}

pub fn string(s: &str) -> Expression {
  Literal(StringLiteral(s.to_string()))
}

pub fn print(expr: Expression) -> Statement {
  Print(expr)
}

pub fn declare(variable: &str, type_of: TypeName, initial: Option<Expression>) -> Statement {
  Declare {
    name: variable.to_string(),
    type_of,
    initial,
  }
}
