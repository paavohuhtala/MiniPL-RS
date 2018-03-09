use common::types::*;

// We have to heap allocate the sub expressions, because otherwise this type
// wouldn't have a fixed (maximum) size.
type BinaryExpr = Box<(Expression, Expression)>;

#[derive(Debug)]
pub enum Expression {
  Literal(LiteralValue),
  Variable(String),
  Add(BinaryExpr),
  Sub(BinaryExpr),
  Mul(BinaryExpr),
  Div(BinaryExpr),
  Equal(BinaryExpr),
  Not(Box<Expression>)
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
  Assert(Expression),
}

pub type Program<'a> = &'a [Statement];
