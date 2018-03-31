use std::io::Error;

use diagnostics::file_context::FileContextSource;
use parsing::token::*;

// The error types form a hierarchy.
// Lower level errors can be implicitly casted to higher level errors, e.g
// CharStreamError::EndOfFile is automatically converted to
// LexerError::CharStreamError(EndOfFile)

pub trait ErrorWithContext {
  fn get_offset(&self) -> usize;
}

pub trait ErrorWithReason {
  fn get_reason(&self) -> Option<String>;
}

pub trait MiniPlError: ErrorWithContext + ErrorWithReason {}

impl MiniPlError {
  fn format(&self, context: &FileContextSource) -> String {
    let err = String::new();

    let offset = self.get_offset();

    let pos = context
      .decode_offset(offset)
      .expect("Should be a valid offset.");
    err
  }
}

#[derive(Debug)]
pub struct ErrWithCtx<E: ErrorWithReason>(pub E, pub usize);

impl<E: ErrorWithReason> ErrorWithContext for ErrWithCtx<E> {
  fn get_offset(&self) -> usize {
    self.1
  }
}

impl<E: ErrorWithReason> ErrorWithReason for ErrWithCtx<E> {
  fn get_reason(&self) -> Option<String> {
    self.0.get_reason()
  }
}

pub trait TryRecover {
  fn try_recover(&mut self) -> bool;
}

#[derive(Debug, Clone)]
pub enum CharStreamError {
  EndOfFile,
}

#[derive(Debug, Clone)]
pub enum LexerError {
  OutOfTokens,
  UnknownToken,
  UnknownEscapeCode(String),
  UnterminatedStringLiteral,
  InvalidNumberLiteral,
  ReservedKeywordAsIdentifier,
  UnterminatedComment,
  CharStreamError(CharStreamError),
  IOError(String)
}

pub type LexerErrorWithCtx = ErrWithCtx<LexerError>;

impl ErrorWithReason for CharStreamError {
  fn get_reason(&self) -> Option<String> {
    None
  }
}

impl ErrorWithReason for LexerError {
  fn get_reason(&self) -> Option<String> {
    None
  }
}

impl ErrorWithReason for ParserError {
  fn get_reason(&self) -> Option<String> {
    None
  }
}

#[derive(Debug, Clone)]
pub enum ParserError {
  MalformedStatement,
  InvalidBinaryExpression,
  UnknownStatement { first: TokenKind },
  UnexpectedToken { expected: TokenKind, was: TokenKind },
  LexerError(LexerError),
  MissingRParen,
  IncompleteExpression,
}

pub type ParserErrorWithCtx = ErrWithCtx<ParserError>;
pub type ParserErrors = Vec<ParserErrorWithCtx>;

pub trait AddCtxToError
where
  Self: ErrorWithReason + Sized,
{
  fn with_ctx(self, ctx: usize) -> ErrWithCtx<Self> {
    ErrWithCtx(self, ctx)
  }
}

impl<T: ErrorWithReason + Sized> AddCtxToError for T {}

pub trait AddCtxToResult<T, E>
where
  E: ErrorWithReason + Sized,
{
  fn with_ctx(self, ctx: usize) -> Result<T, ErrWithCtx<E>>;
}

impl<T, E> AddCtxToResult<T, E> for Result<T, E>
where
  E: ErrorWithReason + Sized,
{
  fn with_ctx(self, ctx: usize) -> Result<T, ErrWithCtx<E>> {
    self.map_err(|err| ErrWithCtx(err, ctx))
  }
}

impl From<ErrWithCtx<LexerError>> for ErrWithCtx<ParserError> {
  fn from(err: ErrWithCtx<LexerError>) -> ErrWithCtx<ParserError> {
    ErrWithCtx(err.0.into(), err.1)
  }
}

impl From<Error> for LexerError {
  fn from(err: Error) -> LexerError {
    LexerError::IOError(err.to_string())
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
