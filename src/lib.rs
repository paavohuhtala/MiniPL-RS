#![feature(slice_patterns)]

use std::rc::Rc;

#[macro_use]
pub mod common;
pub mod diagnostics;
pub mod parsing;
pub mod runtime;
pub mod semantic;

use common::errors::*;
use common::logger::Logger;
use diagnostics::file_context::*;
use parsing::char_stream::CharStream;
use parsing::lexer::BufferedLexer;
use parsing::parser::Parser;
use runtime::*;
use semantic::type_checker::*;

#[derive(Debug)]
pub enum ExecutionError {
  ParserError(ParserError),
  TypeError(TypeError),
}

impl From<ParserError> for ExecutionError {
  fn from(err: ParserError) -> ExecutionError {
    ExecutionError::ParserError(err)
  }
}

impl From<TypeError> for ExecutionError {
  fn from(err: TypeError) -> ExecutionError {
    ExecutionError::TypeError(err)
  }
}

/// Run a script using the given IO handler (e.g `ConsoleIo`).
pub fn run_script<T: Io>(
  source: &str,
  io: &mut T,
  logger: Rc<Logger>,
) -> Result<(), ExecutionError> {
  let file_context = FileContextSource::from_str(source, None);

  // This is our compiler pipeline:

  // We'll wrap the source string into a stream-like type for easier use and O(1) indexing.
  let tokens = CharStream::new(source);

  // The lexer splits the stream into tokens, and buffers them to allow peeking and backtracking.
  let lexer = BufferedLexer::new(tokens, logger.clone());

  // The parser parses the token stream into an AST.
  let mut parser = Parser::new(lexer, logger.clone());

  // ... which we'll use to obtain the program AST.
  let program = parser.parse_program()?;

  // Run the type checker.
  type_check(&program)?;

  // If type checking was succesful, create a new interpreter and run the program.
  let mut interpreter = Interpreter::new(io, &file_context);
  interpreter.execute(&program);

  Ok(())
}
