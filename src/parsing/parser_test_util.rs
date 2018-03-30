use std::rc::Rc;

use common::errors::ParserError;
use common::logger::NullLogger;

use parsing::ast::{Expression, Statement};
use parsing::lexer::BufferedLexer;
use parsing::parser::Parser;

use parsing::lexer_test_util::create_lexer;

pub fn create_parser(src: &str) -> Parser<BufferedLexer> {
  Parser::new(create_lexer(src), Rc::new(NullLogger))
}

pub fn parse_stmnt(src: &str) -> Result<Statement, ParserError> {
  let mut parser = create_parser(src);
  parser.parse_statement()
}

pub fn parse_expr(src: &str) -> Result<Expression, ParserError> {
  let mut parser = create_parser(src);
  parser.parse_expression()
}
