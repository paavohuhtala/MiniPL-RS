use common::errors::*;
use parsing::token::*;

/// The interface for lexers.
pub trait TokenStream {
  /// Advances to the next token if there are more tokens available.
  fn advance(&mut self);

  /// Returns true if there are no more tokens available.
  fn reached_end(&self) -> bool;

  /// Returns the offset in the input file.
  fn offset(&self) -> usize;

  /// Tries to read the next token.
  fn peek(&mut self) -> Result<TokenWithCtx, ErrWithCtx<LexerError>>;

  /// Tries to read the next token, and advances to the next token on success.
  fn next(&mut self) -> Result<TokenWithCtx, ErrWithCtx<LexerError>>;
}
