use std::io::Error;

use common::types::*;

// The error types form a hierarchy.
// Lower level errors can be implicitly casted to higher level errors, e.g
// CharStreamError::EndOfFile is automatically converted to
// LexerError::CharStreamError(EndOfFile)

#[derive(Debug)]
pub enum CharStreamError {
  EndOfFile,
}

#[derive(Debug)]
pub enum LexerError {
  OutOfTokens,
  UnknownLexeme,
  UnknownEscapeCode(String),
  UnterminatedStringLiteral,
  InvalidNumberLiteral,
  ReservedKeywordAsIdentifier,
  UnterminatedComment,
  CharStreamError(CharStreamError),
  IOError(Error),
}

#[derive(Debug)]
pub enum ParserError {
  MalformedStatement,
  UnexpectedToken { expected: TokenKind, was: TokenKind },
  LexerError(LexerError),
  MissingRParen,
  IncompleteExpression,
}

impl From<Error> for LexerError {
  fn from(err: Error) -> LexerError {
    LexerError::IOError(err)
  }
}

impl From<CharStreamError> for LexerError {
  fn from(err: CharStreamError) -> LexerError {
    LexerError::CharStreamError(err)
  }
}

impl From<LexerError> for ParserError {
  fn from(err: LexerError) -> ParserError {
    ParserError::LexerError(err)
  }
}
