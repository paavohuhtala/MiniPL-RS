// This file provides small utility functions for writing lexer tests.
use common::types::*;
use common::types::Operator::*;
use common::types::BinaryOperator::*;
use common::errors::LexerError;

use parsing::char_stream::CharStream;
use parsing::lexer::BufferedLexer;
use parsing::token_stream::TokenStream;

pub fn add_op() -> Token {
  Token::Operator(BinaryOperator(Add))
}

pub fn equal_op() -> Token {
  Token::Operator(BinaryOperator(Equal))
}

pub fn number(i: i32) -> Token {
  Token::Literal(LiteralValue::IntLiteral(i))
}

pub fn string(s: &str) -> Token {
  Token::Literal(LiteralValue::StringLiteral(s.to_string()))
} 

pub fn variable(s: &str) -> Token {
  Token::Identifier(s.to_string())
}

pub fn create_lexer(input: &str) -> BufferedLexer {
  let stream = CharStream::new(input);
  BufferedLexer::new(stream)
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
  let mut lexer = create_lexer(input);
  let mut tokens: Vec<Token> = Vec::new();

  while !lexer.reached_end() {
    let token_with_ctx = lexer.next()?;
    tokens.push(token_with_ctx.token);
  }

  Ok(tokens)
}
