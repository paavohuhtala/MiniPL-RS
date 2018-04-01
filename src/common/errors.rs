use std::io::Error;

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

impl ErrorWithReason for CharStreamError {
  fn get_reason(&self) -> Option<String> {
    match *self {
      CharStreamError::EndOfFile => Some("Unexpected end of file.".to_string()),
    }
  }
}

#[derive(Debug, Clone)]
pub enum LexerError {
  OutOfTokens,
  UnknownToken(String),
  UnknownEscapeCode(String),
  UnterminatedStringLiteral,
  InvalidNumberLiteral,
  UnterminatedComment,
  CharStreamError(CharStreamError),
  IOError(String),
}

pub type LexerErrorWithCtx = ErrWithCtx<LexerError>;

impl ErrorWithReason for LexerError {
  fn get_reason(&self) -> Option<String> {
    use LexerError::*;
    match *self {
      OutOfTokens => Some("Out of input.".to_string()),
      UnknownToken(ref starting_with) => Some(format!(
        "Encountered an unknown token (starting with {}).",
        starting_with
      )),
      UnknownEscapeCode(ref escape_code) => Some(format!(
        "Unknown escape character in string literal: {}",
        escape_code
      )),
      UnterminatedStringLiteral => Some("Unterminated string literal.".to_string()),
      InvalidNumberLiteral => Some("Invalid number literal.".to_string()),
      UnterminatedComment => Some("Unterminated comment literal.".to_string()),
      CharStreamError(ref error) => Some(format!(
        "Character stream error: {}",
        error.get_reason().unwrap()
      )),
      IOError(ref error_msg) => Some(format!("IO error: {}", error_msg)),
    }
  }
}

#[derive(Debug, Clone)]
pub enum ParserError {
  InvalidBinaryExpression,
  UnknownStatement { first: TokenKind },
  UnexpectedToken { expected: TokenKind, was: TokenKind },
  LexerError(LexerError),
  MissingRParen,
  IncompleteExpression,
}

pub type ParserErrorWithCtx = ErrWithCtx<ParserError>;
pub type ParserErrors = Vec<ParserErrorWithCtx>;

impl ErrorWithReason for ParserError {
  fn get_reason(&self) -> Option<String> {
    match *self {
      ParserError::InvalidBinaryExpression => Some("Invalid expression.".to_string()),
      ParserError::UnknownStatement { first } => {
        Some(format!("Unknown starting token: {:?}", first))
      }
      ParserError::MissingRParen => Some(
        "Unbalanced parenthesis in expression (probably missing right parenthesis?)".to_string(),
      ),
      ParserError::IncompleteExpression => Some("Incomplete expression.".to_string()),
      ParserError::UnexpectedToken { expected, was } => Some(format!(
        "Unexpected token. Expected {:?}, was {:?}",
        expected, was
      )),
      ParserError::LexerError(ref lexer_error) => Some(format!(
        "Lexer error: {}",
        lexer_error.get_reason().unwrap()
      )),
    }
  }
}

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
