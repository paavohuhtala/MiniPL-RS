pub enum TypeName {
  IntType,
  StringType,
  BoolType,
}

pub enum LiteralValue {
  StringLiteral(String),
  IntLiteral(i32),
}

pub enum BinaryOperator {
  Add,
  Sub,
  Mul,
  Div,
  LessThan,
  Equal,
  And,
}

pub enum UnaryOperator {
  Not,
}

pub enum Expression {
  Literal(LiteralValue),
  Variable(String),
  // We have to heap allocate the sub expressions, because otherwise this type
  // wouldn't have a fixed (maximum) size.
  BinaryOp(BinaryOperator, Box<(Expression, Expression)>),
  UnaryOp(UnaryOperator, Box<Expression>),
}

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
