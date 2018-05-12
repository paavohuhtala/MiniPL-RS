use common::types::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Identifier(String),
  Literal(LiteralValue),
  Type(TypeName),
  Operator(Operator),
  Semicolon,
  LParen,
  RParen,
  LCurly,
  RCurly,
  Colon,
  Assign,
  Print,
  Read,
  Var,
  Assert,
  For,
  In,
  Range,
  EndOfFile,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenWithCtx {
  pub offset: usize,
  pub token: Token,
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
  LCurlyK,
  RCurlyK,
  ColonK,
  AssignK,
  PrintK,
  ReadK,
  VarK,
  AssertK,
  ForK,
  InK,
  RangeK,
  EndOfFileK,
}

impl Token {
  pub fn get_kind(&self) -> TokenKind {
    use self::Token::*;
    use self::TokenKind::*;
    match *self {
      Identifier(_) => IdentifierK,
      Literal(_) => LiteralK,
      Type(_) => TypeK,
      Operator(_) => OperatorK,
      Semicolon => SemicolonK,
      Colon => ColonK,
      LParen => LParenK,
      RParen => RParenK,
      LCurly => LCurlyK,
      RCurly => RCurlyK,
      Assign => AssignK,
      Print => PrintK,
      Read => ReadK,
      Var => VarK,
      Assert => AssertK,
      For => ForK,
      In => InK,
      Range => RangeK,
      EndOfFile => EndOfFileK,
    }
  }

  pub fn kind_equals(&self, other: TokenKind) -> bool {
    self.get_kind() == other
  }

  pub fn kinds_equal(&self, other: &Token) -> bool {
    self.kind_equals(other.get_kind())
  }
}
