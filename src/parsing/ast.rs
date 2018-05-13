use std::ops::Range;

use common::types::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Expression {
  Literal(LiteralValue),
  Variable(String),
  // We have to heap allocate the sub expressions, because otherwise this type
  // wouldn't have a fixed (maximum) size.
  BinaryOp(BinaryOperator, Box<(Expression, Expression)>),
  UnaryOp(UnaryOperator, Box<Expression>),
}

#[derive(PartialEq, Eq, Debug)]
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
    run: Block,
  },
  Print(Expression),
  Read(String),
  Assert(Expression),
}

#[derive(PartialEq, Eq, Debug)]
pub struct StatementWithCtx {
  pub source_position: Range<usize>,
  pub statement: Statement,
}

pub type Block = Vec<StatementWithCtx>;

pub type Program<'a> = &'a [StatementWithCtx];
