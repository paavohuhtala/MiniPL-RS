use common::types::*;

#[derive(Debug)]
pub enum Expression {
  Literal(LiteralValue),
  Variable(String),
  // We have to heap allocate the sub expressions, because otherwise this type wouldn't have a fixed (maximum) size.
  Add(Box<(Expression, Expression)>),
  Sub(Box<(Expression, Expression)>),
  Mul(Box<(Expression, Expression)>),
  Div(Box<(Expression, Expression)>),
}

#[derive(Debug)]
pub enum Statement {
  Declare {
    name: String,
    type_of: TypeName,
    initial: Option<Expression>,
  },
  Assign,
  For,
  Read,
  Print(Expression),
  Assert,
}

pub type Program = Vec<Statement>;
