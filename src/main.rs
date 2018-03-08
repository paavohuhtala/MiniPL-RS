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
  let input_chars = read_file("./minipl/hello.pl").expect("File should be readable.");
  let input_stream = CharStream::new(&input_chars);

  let lexer = BufferedLexer::new(input_stream);
  let mut parser = Parser::new(lexer);

  let statements = parser
    .parse_statement_list()
    .expect("Program should parse succesfully.");

  if let Err(type_error) = type_check(&statements) {
    println!("Type error: {:?}", type_error);
  }
}
