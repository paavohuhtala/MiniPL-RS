#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeName {
  IntType,
  StringType,
  BoolType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
  StringLiteral(String),
  IntLiteral(i32),
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

// https://eli.thegreenplace.net/2010/01/02/top-down-operator-precedence-parsing
impl Operator {
  pub fn get_precedence(self) -> u8 {
    use Operator::*;
    match self {
      Not => 30,
      Mul | Div => 20,
      Add | Sub | And => 10,
      LessThan | Equal => 5,
    }
  }

  pub fn get_arity(self) -> Arity {
    use Operator::*;
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
  Var,
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
  VarK,
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
      Token::Var => TokenKind::VarK,
    }
  }

  pub fn kind_equals(&self, other: TokenKind) -> bool {
    self.get_kind() == other
  }

  pub fn kinds_equal(&self, other: &Token) -> bool {
    self.kind_equals(other.get_kind())
  }
}
