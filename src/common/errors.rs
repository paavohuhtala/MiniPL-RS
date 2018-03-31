use std::io::Error;
use std::ops::Range;
use std::ops::Try;

use diagnostics::file_context::FileContextSource;
use parsing::token::*;

// The error types form a hierarchy.
// Lower level errors can be implicitly casted to higher level errors, e.g
// CharStreamError::EndOfFile is automatically converted to
// LexerError::CharStreamError(EndOfFile)

pub trait ErrorWithContext {
  fn get_range(&self) -> &Range<usize>;
}

pub trait ErrorWithReason {
  fn get_reason(&self) -> Option<String>;
}

pub trait MiniPlError: ErrorWithContext + ErrorWithReason {}

impl MiniPlError {
  fn format(&self, context: &FileContextSource) -> String {
    let err = String::new();

    let range = self.get_range();

    let start_pos = context
      .decode_offset(range.start)
      .expect("Should be a valid offset.");

    let end_pos = if range.start == range.end {
      start_pos
    } else {
      context
        .decode_offset(range.end)
        .expect("Should be a valid offset.")
    };

    err
  }
}

pub struct ErrWithCtx<E: ErrorWithReason>(E, Range<usize>);

impl<E: ErrorWithReason> ErrorWithContext for ErrWithCtx<E> {
  fn get_range(&self) -> &Range<usize> {
    &self.1
  }
}

impl<E: ErrorWithReason> ErrorWithReason for ErrWithCtx<E> {
  fn get_reason(&self) -> Option<String> {
    self.0.get_reason()
  }
}

pub enum MiniPlResult<T> {
  Success(T),
  Errors(Vec<Box<MiniPlError>>),
}

impl<T> Try for MiniPlResult<T> {
  type Ok = T;
  type Error = Vec<Box<MiniPlError>>;

  fn into_result(self) -> Result<T, Self::Error> {
    match self {
      MiniPlResult::Success(value) => Ok(value),
      MiniPlResult::Errors(errs) => Err(errs),
    }
  }

  fn from_ok(value: T) -> Self {
    MiniPlResult::Success(value)
  }

  fn from_error(errors: Self::Error) -> Self {
    MiniPlResult::Errors(errors)
  }
}

pub trait TryRecover {
  fn try_recover(&mut self) -> bool;
}

#[derive(Debug)]
pub enum CharStreamError {
  EndOfFile,
}

#[derive(Debug)]
pub enum LexerError {
  OutOfTokens,
  UnknownToken,
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
