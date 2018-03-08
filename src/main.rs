#![feature(slice_patterns)]

use std::io::Read;
use std::path::Path;

mod common;
use common::types::*;

mod parsing;
use parsing::char_stream::*;
use parsing::lexer::*;
use parsing::ast::*;
use parsing::parser::*;

mod semantic;
use semantic::type_checker::type_check;

fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<char>, std::io::Error> {
  let mut input_file = std::fs::File::open(path)?;
  let mut input_source = String::new();
  input_file
    .read_to_string(&mut input_source)
    .expect("Can't read source file.");
  Ok(input_source.chars().collect())
}

fn main() {
  // Read the file into a character vector.
  let input_chars = read_file("./minipl/hello.pl").expect("File should be readable.");

  // This is our compiler pipeline:

  // Wraps the input into a stream-like type, to make it easier to use.
  let input_stream = CharStream::new(&input_chars);
  // Splits the stream into tokens, and buffers them to allow peeking and backtracking.
  let lexer = BufferedLexer::new(input_stream);
  // Parses the token stream into an AST.
  let mut parser = Parser::new(lexer);

  let statements = parser
    .parse_statement_list()
    .expect("Program should parse succesfully.");

  if let Err(type_error) = type_check(&statements) {
    println!("Type error: {:?}", type_error);
  }
}
