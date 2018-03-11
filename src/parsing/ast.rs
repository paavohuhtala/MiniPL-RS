use common::types::*;

#[derive(Debug)]
pub enum Expression {
  Literal(LiteralValue),
  Variable(String),
  // We have to heap allocate the sub expressions, because otherwise this type
  // wouldn't have a fixed (maximum) size.
  BinaryOp(BinaryOperator, Box<(Expression, Expression)>),
  UnaryOp(UnaryOperator, Box<Expression>),
}

#[derive(Debug)]
pub enum Statement {
  Declare {
    name: String,
    type_of: TypeName,
    initial: Option<Expression>,
  },
  Assign(String, Expression),
  For {
    variable: String,
    from: Expression,
    to: Expression,
    run: Vec<Statement>,
  },
  Print(Expression),
  Read(String),
  Assert(Expression),
}

pub type Program<'a> = &'a [Statement];
