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
  LessThan(BinaryExpr),
  Not(Box<Expression>),
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
