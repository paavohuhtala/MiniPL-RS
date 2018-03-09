use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeName {
  IntType,
  StringType,
  BoolType,
}

#[derive(Debug, PartialEq, Clone)]
/// These values exist at parse time.
pub enum LiteralValue {
  StringLiteral(String),
  IntLiteral(i32),
}

#[derive(Debug, PartialEq, Clone)]
/// These values exist at runtime.
pub enum Value {
  IntV(i32),
  StringV(String),
  BoolV(bool),
}

impl Value {
  pub fn get_type(&self) -> TypeName {
    match *self {
      Value::IntV(_) => TypeName::IntType,
      Value::StringV(_) => TypeName::StringType,
      Value::BoolV(_) => TypeName::BoolType
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
  LessThan,
  Equal,
  And,
  Not,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Arity {
  Unary,
  Binary,
}

impl Operator {
  pub fn get_precedence(self) -> u8 {
    use self::Operator::*;
    match self {
      Not => 3,
      Mul | Div => 2,
      Add | Sub | And => 1,
      LessThan | Equal => 0,
    }
  }

  pub fn get_arity(self) -> Arity {
    use self::Operator::*;
    match self {
      Add | Sub | Mul | Div | LessThan | Equal | And => Arity::Binary,
      Not => Arity::Unary,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Identifier(String),
  Literal(LiteralValue),
  Type(TypeName),
  Operator(Operator),
  Semicolon,
  LParen,
  RParen,
  Colon,
  Assign,
  Print,
  Read,
  Var,
  Assert,
  EndOfFile,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
  IdentifierK,
  LiteralK,
  TypeK,
  OperatorK,
  SemicolonK,
  LParenK,
  RParenK,
  ColonK,
  AssignK,
  PrintK,
  ReadK,
  VarK,
  AssertK,
  EndOfFileK,
}

impl Token {
  pub fn get_kind(&self) -> TokenKind {
    match *self {
      Token::Identifier(_) => TokenKind::IdentifierK,
      Token::Literal(_) => TokenKind::LiteralK,
      Token::Type(_) => TokenKind::TypeK,
      Token::Operator(_) => TokenKind::OperatorK,
      Token::Semicolon => TokenKind::SemicolonK,
      Token::Colon => TokenKind::ColonK,
      Token::LParen => TokenKind::LParenK,
      Token::RParen => TokenKind::RParenK,
      Token::Assign => TokenKind::AssignK,
      Token::Print => TokenKind::PrintK,
      Token::Read => TokenKind::ReadK,
      Token::Var => TokenKind::VarK,
      Token::Assert => TokenKind::AssertK,
      Token::EndOfFile => TokenKind::EndOfFileK,
    }
  }

  pub fn kind_equals(&self, other: TokenKind) -> bool {
    self.get_kind() == other
  }

  pub fn kinds_equal(&self, other: &Token) -> bool {
    self.kind_equals(other.get_kind())
  }
}
