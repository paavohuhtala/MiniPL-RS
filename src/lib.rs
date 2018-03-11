#![feature(slice_patterns)]

#[macro_use]
pub mod common;
pub mod parsing;
pub mod semantic;
pub mod runtime;

use parsing::char_stream::CharStream;
use parsing::lexer::BufferedLexer;
use parsing::parser::Parser;

use common::errors::*;
use semantic::type_checker::*;
use runtime::*;

#[derive(Debug)]
pub enum ExecutionError {
  ParserError(ParserError),
  TypeError(TypeError)
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
  // This is our compiler pipeline:

  // First, we'll convert the string into a character vector, so that we can
  // access individual characters in O(1) time.
  let chars: Vec<char> = source.chars().collect();

  // Then, we'll wrap the input into a stream-like type, to make it easier to use.
  let tokens = CharStream::new(&chars);

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
