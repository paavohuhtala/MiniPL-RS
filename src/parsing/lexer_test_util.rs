// This file provides small utility functions for writing lexer tests.
use common::types::*;
use parsing::char_stream::CharStream;
use parsing::lexer::BufferedLexer;
use parsing::token_source::TokenSource;

pub fn add_op() -> Token {
  Token::Operator(Operator::Add)
}

pub fn equal_op() -> Token {
  Token::Operator(Operator::Equal)
}

pub fn number(i: i32) -> Token {
  Token::Literal(LiteralValue::IntLiteral(i))
}

pub fn variable(s: &str) -> Token {
  Token::Identifier(s.to_string())
}

pub fn lex(input: &str) -> Vec<Token> {
  let input_chars: Vec<char> = input.chars().collect();
  let stream = CharStream::new(&input_chars);
  let mut lexer = BufferedLexer::new(stream);

  let mut tokens = Vec::new();

  while !lexer.reached_end() {
    let token_with_ctx = lexer.next().unwrap();
    tokens.push(token_with_ctx.token);
  }

  tokens
}
