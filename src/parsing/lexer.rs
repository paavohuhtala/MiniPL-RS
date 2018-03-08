use common::types::*;
use common::errors::*;

use parsing::util::*;
use parsing::char_stream::*;
use parsing::token_source::TokenSource;

fn read_string_literal(input: &mut CharStream) -> Result<Token, LexerError> {
  if let Ok(ch) = input.peek() {
    if ch != '"' {
      return Err(LexerError::UnknownLexeme);
    }
  }

  input.advance();

  let contents = input.take_until(|ch| ch == '"');

  if input.reached_end() {
    return Err(LexerError::UnterminatedStringLiteral);
  }

  // If take_until didn't reach EOF, we know this char is a double quote.
  input.advance();

  let s = contents.iter().collect();

  Ok(Token::Literal(LiteralValue::StringLiteral(s)))
}

static RESERVED_WORDS: &'static [&str] = &["print", "int", "bool", "string"];

fn read_keyword_or_identifier(input: &mut CharStream) -> Result<Token, LexerError> {
  let chars = input.take_until(|c| !is_valid_in_identifier(c));
  match *chars {
    ['p', 'r', 'i', 'n', 't'] => Ok(Token::Print),
    ['i', 'n', 't'] => Ok(Token::Type(TypeName::IntType)),
    ['b', 'o', 'o', 'l'] => Ok(Token::Type(TypeName::BoolType)),
    ['s', 't', 'r', 'i', 'n', 'g'] => Ok(Token::Type(TypeName::StringType)),
    ['v', 'a', 'r'] => Ok(Token::Var),
    _ => {
      let name: String = chars.iter().collect();

      if RESERVED_WORDS.contains(&name.as_str()) {
        Err(LexerError::ReservedKeywordAsIdentifier)
      } else if !is_valid_identifier(&name) {
        Err(LexerError::UnknownLexeme)
      } else {
        Ok(Token::Identifier(name))
      }
    }
  }
}

fn parse_single_char_token(ch: char) -> Token {
  match ch {
    ';' => Token::Semicolon,
    '(' => Token::LParen,
    ')' => Token::RParen,
    '+' => Token::Operator(Operator::Add),
    '-' => Token::Operator(Operator::Sub),
    '*' => Token::Operator(Operator::Mul),
    '/' => Token::Operator(Operator::Div),
    '<' => Token::Operator(Operator::LessThan),
    '=' => Token::Operator(Operator::Equal),
    '&' => Token::Operator(Operator::And),
    '!' => Token::Operator(Operator::Not),
    _ => panic!("This should not happen."),
  }
}

fn read_number_literal(input: &mut CharStream) -> Result<Token, LexerError> {
  let digits = input.take_until(|ch| !is_number(ch));
  let digits_as_str: String = digits.iter().collect();

  match str::parse::<i32>(&digits_as_str) {
    Ok(i) => Ok(Token::Literal(LiteralValue::IntLiteral(i))),
    Err(_) => Err(LexerError::InvalidNumberLiteral),
  }
}

fn next_lexeme(input: &mut CharStream) -> Result<Token, LexerError> {
  // Skip whitespace
  input.advance_until(|ch| !is_whitespace(ch));

  if input.reached_end() {
    return Ok(Token::EndOfFile);
  }

  // TODO: Check for tokens.
  let first = input.peek()?;

  let token = match first {
    ';' | '+' | '-' | '*' | '/' | '<' | '=' | '&' | '!' => {
      input.advance();
      Ok(parse_single_char_token(first))
    }
    ':' => {
      input.advance();

      if input.peek()? == '=' {
        input.advance();
        Ok(Token::Assign)
      } else {
        Ok(Token::Colon)
      }
    }
    '0'...'9' => read_number_literal(input),
    '"' => read_string_literal(input),
    // TODO: handle expressions
    _ => read_keyword_or_identifier(input),
  };

  println!("Token: {:?}", token);

  token
}

pub struct BufferedLexer<'a> {
  stream: CharStream<'a>,
  tokens: Vec<Token>,
}

impl<'a> BufferedLexer<'a> {
  pub fn new(stream: CharStream<'a>) -> BufferedLexer<'a> {
    BufferedLexer {
      stream,
      tokens: Vec::new(),
    }
  }
}

impl<'a> TokenSource for BufferedLexer<'a> {
  fn advance(&mut self) {
    if self.tokens.is_empty() {
      self.stream.advance();
    } else {
      self.tokens.pop();
    }
  }

  fn reached_end(&self) -> bool {
    self.tokens.is_empty() && self.stream.reached_end()
  }

  fn peek(&mut self) -> Result<Token, LexerError> {
    if self.tokens.is_empty() {
      let next = next_lexeme(&mut self.stream)?;
      self.tokens.push(next.clone());
      Ok(next)
    } else {
      match self.tokens.last() {
        Some(token) => Ok(token.clone()),
        None => Err(LexerError::OutOfTokens),
      }
    }
  }

  fn next(&mut self) -> Result<Token, LexerError> {
    if self.tokens.is_empty() {
      next_lexeme(&mut self.stream)
    } else {
      Ok(self.tokens.pop().expect("This should never happen."))
    }
  }
}
