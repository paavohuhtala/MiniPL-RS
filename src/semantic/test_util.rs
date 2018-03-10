use common::types::*;
use common::types::LiteralValue::*;
use common::types::TypeName::*;
use parsing::ast::*;
use parsing::ast::Expression::*;

pub fn add(a: Expression, b: Expression) -> Expression {
  Add(Box::new((a, b)))
}

pub fn sub(a: Expression, b: Expression) -> Expression {
  Sub(Box::new((a, b)))
}

pub fn expr_of_type(t: TypeName) -> Expression {
  match t {
    BoolType => Equal(Box::new((expr_of_type(IntType), expr_of_type(IntType)))),
    IntType => Literal(IntLiteral(0)),
    StringType => Literal(StringLiteral("".to_string()))
  }
}
