#![feature(slice_patterns)]

#[macro_use]
pub mod common;
pub mod diagnostics;
pub mod parsing;
pub mod semantic;
pub mod runtime;

use parsing::char_stream::CharStream;
use parsing::lexer::BufferedLexer;
use parsing::parser::Parser;

use common::errors::*;
use semantic::type_checker::*;
use runtime::*;

use diagnostics::file_context::*;

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
pub fn run_script<T: Io>(source: &str, io: &mut T) -> Result<(), ExecutionError> {
  let diagnostics_source = FileContextSource::from_str(source, None);

  // This is our compiler pipeline:

  // We'll wrap the source string into a stream-like type for easier use and O(1) indexing.
  let tokens = CharStream::new(source);

  // The lexer splits the stream into tokens, and buffers them to allow peeking and backtracking.
  let lexer = BufferedLexer::new(tokens);

  // The parser parses the token stream into an AST.
  let mut parser = Parser::new(lexer);
  // ... which we'll use to obtain the program AST.
  let program = parser.parse_program()?;

  // Run the type checker.
  type_check(&program)?;

  // If type checking was succesful, create a new interpreter and run the program.
  let mut interpreter = Interpreter::new(io);
  interpreter.execute(&program);

  Ok(())
}
