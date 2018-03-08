// Implements a Read+Seek -like interface over a string slice,
// with support for backtracking.

use std::cmp::min;

use common::errors::*;

#[derive(Debug)]
pub struct CharStream<'a> {
  chars: &'a [char],
  offset: usize,
}

impl<'a> CharStream<'a> {
  /// Creates a new char stream from a char slice.
  pub fn new(chars: &'a [char]) -> CharStream<'a> {
    CharStream { chars, offset: 0 }
  }

  pub fn advance(&mut self) {
    self.offset = min(self.chars.len(), self.offset + 1);
  }

  pub fn peek(&self) -> Result<char, CharStreamError> {
    if self.reached_end() {
      Err(CharStreamError::EndOfFile)
    } else {
      let ch = self.chars[self.offset];
      Ok(ch)
    }
  }

  pub fn reached_end(&self) -> bool {
    self.offset >= self.chars.len()
  }

  fn slice_at(&self, offset: usize, length: usize) -> &'a [char] {
    &self.chars[offset..offset + length]
  }

  pub fn advance_until<F>(&mut self, pred: F)
  where
    F: Fn(char) -> bool,
  {
    while let Ok(ch) = self.peek() {
      if pred(ch) {
        break;
      }
      self.advance();
    }
  }

  pub fn take_until<F>(&mut self, pred: F) -> &'a [char]
  where
    F: Fn(char) -> bool,
  {
    let offs = self.offset;
    self.advance_until(pred);
    self.slice_at(offs, self.offset - offs)
  }
}
