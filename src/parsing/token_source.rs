use common::errors::*;
use common::types::*;

/// The interface for lexers.
pub trait TokenSource {
  /// Advances to the next token if there are more tokens available.
  fn advance(&mut self);

  /// Returns true if there are no more tokens available.
  fn reached_end(&self) -> bool;

  /// Tries to read the next token.
  fn peek(&mut self) -> Result<TokenWithCtx, LexerError>;

  /// Tries to read the next token, and advances to the next token on success.
  fn next(&mut self) -> Result<TokenWithCtx, LexerError>;
}
