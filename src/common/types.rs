use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeName {
  IntType,
  StringType,
  BoolType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
/// These values exist at parse time.
pub enum LiteralValue {
  StringLiteral(String),
  IntLiteral(i32),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
/// These values exist at runtime.
pub enum Value {
  IntV(i32),
  StringV(String),
  BoolV(bool),
}

impl TypeName {
  pub fn get_default_value(self) -> Value {
    match self {
      TypeName::IntType => Value::IntV(0),
      TypeName::StringType => Value::StringV("".to_string()),
      TypeName::BoolType => Value::BoolV(false),
    }
  }
}

impl Value {
  pub fn get_type(&self) -> TypeName {
    match *self {
      Value::IntV(_) => TypeName::IntType,
      Value::StringV(_) => TypeName::StringType,
      Value::BoolV(_) => TypeName::BoolType,
    }
  }
}

// Literals can be implicitly casted to runtime values.
impl From<LiteralValue> for Value {
  fn from(literal: LiteralValue) -> Value {
    match literal {
      LiteralValue::IntLiteral(i) => Value::IntV(i),
      LiteralValue::StringLiteral(s) => Value::StringV(s),
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Value::StringV(ref s) => write!(f, "{}", s),
      Value::IntV(i) => write!(f, "{}", i),
      Value::BoolV(b) => write!(f, "{}", if b { "true" } else { "false" }),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOperator {
  Add,
  Sub,
  Mul,
  Div,
  LessThan,
  Equal,
  And,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOperator {
  Not,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
  BinaryOperator(BinaryOperator),
  UnaryOperator(UnaryOperator),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Arity {
  Unary,
  Binary,
}

impl Operator {
  pub fn get_precedence(self) -> u8 {
    use self::BinaryOperator::*;
    use self::Operator::*;
    use self::UnaryOperator::*;

    match self {
      UnaryOperator(Not) => 3,
      BinaryOperator(op) => match op {
        Mul | Div => 2,
        Add | Sub | And => 1,
        LessThan | Equal => 0,
      },
    }
  }

  pub fn get_arity(self) -> Arity {
    use self::Operator::*;
    match self {
      UnaryOperator(_) => Arity::Unary,
      BinaryOperator(_) => Arity::Binary,
    }
  }
}
