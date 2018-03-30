use std::io::Error;

use diagnostics::file_context::FileContextSource;
use parsing::token::*;

// The error types form a hierarchy.
// Lower level errors can be implicitly casted to higher level errors, e.g
// CharStreamError::EndOfFile is automatically converted to
// LexerError::CharStreamError(EndOfFile)

pub trait MiniPlError {
  fn get_range(&self) -> (usize, usize);
  fn get_reason(&self) -> String;
}

impl MiniPlError {
  fn format(&self, context: &FileContextSource) -> String {
    let err = String::new();

    let (start_offs, end_offs) = self.get_range();

    let start_pos = context
      .decode_offset(start_offs)
      .expect("Should be a valid offset.");

    let end_pos = if start_offs == end_offs {
      start_pos
    } else {
      context
        .decode_offset(end_offs)
        .expect("Should be a valid offset.")
    };

    err
  }
}

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
