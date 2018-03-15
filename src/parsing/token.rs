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
  Colon,
  Assign,
  Print,
  Read,
  Var,
  Assert,
  For,
  In,
  Range,
  Do,
  End,
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
  ColonK,
  AssignK,
  PrintK,
  ReadK,
  VarK,
  AssertK,
  ForK,
  InK,
  RangeK,
  DoK,
  EndK,
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
      Token::For => TokenKind::ForK,
      Token::In => TokenKind::InK,
      Token::Range => TokenKind::RangeK,
      Token::Do => TokenKind::DoK,
      Token::End => TokenKind::EndK,
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
