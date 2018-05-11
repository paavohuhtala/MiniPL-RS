#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use std::rc::Rc;

#[macro_use]
pub mod common;
pub mod diagnostics;
pub mod parsing;
pub mod runtime;
pub mod semantic;

use common::errors::*;
use common::logger::Logger;
use common::util::ResultExt;
use diagnostics::file_context::*;
use parsing::char_stream::CharStream;
use parsing::lexer::BufferedLexer;
use parsing::parser::Parser;
use runtime::*;
use semantic::type_checker::*;

#[derive(Debug)]
pub enum ExecutionError {
  ParserError(ParserErrorWithCtx),
  TypeError(TypeError),
}

impl From<ParserErrorWithCtx> for ExecutionError {
  fn from(err: ParserErrorWithCtx) -> ExecutionError {
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
  file_context: Option<Rc<FileContextSource>>,
) -> Result<(), Vec<ExecutionError>> {
  // If we don't have a file context source, construct a new one.
  let file_context =
    file_context.unwrap_or_else(|| Rc::new(FileContextSource::from_str(source, None)));

  // This is our compiler pipeline:

  // We'll wrap the source string into a stream-like type for easier use and O(1) indexing.
  let tokens = CharStream::new(source);

  // The lexer splits the stream into tokens, and buffers them to allow peeking and backtracking.
  let lexer = BufferedLexer::new(tokens, logger.clone());

  // The parser parses the token stream into an AST.
  let mut parser = Parser::new(lexer, logger.clone());

  // ... which we'll use to obtain the program AST.
  let program = parser.parse_program().map_err(|errors| {
    errors
      .into_iter()
      .map(ExecutionError::ParserError)
      .collect::<Vec<ExecutionError>>()
  })?;

  // Run the type checker.
  type_check(&program)
    .map_err(ExecutionError::TypeError)
    .vec_err()?;

  // If type checking was succesful, create a new interpreter and run the program.
  let mut interpreter = Interpreter::new(io, &file_context);
  interpreter.execute(&program);

  Ok(())
}
