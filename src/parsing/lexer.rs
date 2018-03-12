use common::types::*;
use common::errors::*;

use parsing::util::*;
use parsing::char_stream::*;
use parsing::token_stream::TokenStream;

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
  // This is just string comparison, but because string != char sequence, we
  // have to compare using slice patterns.
  match *chars {
    ['p', 'r', 'i', 'n', 't'] => Ok(Token::Print),
    ['r', 'e', 'a', 'd'] => Ok(Token::Read),
    ['i', 'n', 't'] => Ok(Token::Type(TypeName::IntType)),
    ['b', 'o', 'o', 'l'] => Ok(Token::Type(TypeName::BoolType)),
    ['s', 't', 'r', 'i', 'n', 'g'] => Ok(Token::Type(TypeName::StringType)),
    ['v', 'a', 'r'] => Ok(Token::Var),
    ['a', 's', 's', 'e', 'r', 't'] => Ok(Token::Assert),
    ['f', 'o', 'r'] => Ok(Token::For),
    ['i', 'n'] => Ok(Token::In),
    ['d', 'o'] => Ok(Token::Do),
    ['e', 'n', 'd'] => Ok(Token::End),
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
  use common::types::Token::*;
  use common::types::Operator::*;
  use common::types::BinaryOperator::*;
  use common::types::UnaryOperator::*;

  match ch {
    ';' => Semicolon,
    '(' => LParen,
    ')' => RParen,
    '+' => Operator(BinaryOperator(Add)),
    '-' => Operator(BinaryOperator(Sub)),
    '*' => Operator(BinaryOperator(Mul)),
    '/' => Operator(BinaryOperator(Div)),
    '<' => Operator(BinaryOperator(LessThan)),
    '=' => Operator(BinaryOperator(Equal)),
    '&' => Operator(BinaryOperator(And)),
    '!' => Operator(UnaryOperator(Not)),
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

fn next_lexeme(input: &mut CharStream) -> Result<TokenWithCtx, LexerError> {
  // Skip whitespace
  input.advance_until(|ch| !is_whitespace(ch));

  if input.reached_end() {
    return Ok(TokenWithCtx {
      offset: input.offset,
      token: Token::EndOfFile,
    });
  }

  let offset = input.offset;

  let with_ctx =
    |token: Result<Token, LexerError>| token.map(|token| TokenWithCtx { offset, token });

  let first = input.peek()?;

  let token = match first {
    ';' | '(' | ')' | '+' | '-' | '*' | '<' | '=' | '&' | '!' => {
      input.advance();
      with_ctx(Ok(parse_single_char_token(first)))
    }
    ':' => {
      input.advance();

      if input.peek()? == '=' {
        input.advance();
        with_ctx(Ok(Token::Assign))
      } else {
        with_ctx(Ok(Token::Colon))
      }
    }
    '.' => {
      input.advance();
      if input.peek()? == '.' {
        input.advance();
        with_ctx(Ok(Token::Range))
      } else {
        with_ctx(Err(LexerError::UnknownLexeme))
      }
    }
    '0'...'9' => with_ctx(read_number_literal(input)),
    '"' => with_ctx(read_string_literal(input)),
    '/' => {
      input.advance();
      let next = input.peek()?;

      if next == '/' {
        input.advance();
        // If this is a single line comment, skip until the next newline
        input.advance_until(|ch| ch == '\n');
        // Recursively call self to get the next token
        next_lexeme(input)
      } else if next == '*' {
        input.advance();

        let mut prev = input.peek()?;
        input.advance();

        loop {
          if input.reached_end() {
            return Err(LexerError::UnterminatedComment);
          }

          let next = input.peek()?;
          input.advance();

          if prev == '*' && next == '/' {
            break;
          }

          prev = next;
        }

        next_lexeme(input)
      } else {
        with_ctx(Ok(Token::Operator(Operator::BinaryOperator(
          BinaryOperator::Div,
        ))))
      }
    }
    _ => with_ctx(read_keyword_or_identifier(input)),
  };

  println!("[{}]: Token: {:?}", offset, token);
  token
}

pub struct BufferedLexer<'a> {
  stream: CharStream<'a>,
  tokens: Vec<TokenWithCtx>,
}

impl<'a> BufferedLexer<'a> {
  pub fn new(stream: CharStream<'a>) -> BufferedLexer<'a> {
    BufferedLexer {
      stream,
      tokens: Vec::new(),
    }
  }
}

impl<'a> TokenStream for BufferedLexer<'a> {
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

  fn peek(&mut self) -> Result<TokenWithCtx, LexerError> {
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

  fn next(&mut self) -> Result<TokenWithCtx, LexerError> {
    if self.tokens.is_empty() {
      next_lexeme(&mut self.stream)
    } else {
      Ok(self.tokens.pop().expect("This should never happen."))
    }
  }
}

#[cfg(test)]
mod tests {
  use parsing::lexer_test_util::*;
  use common::types::Token::*;

  #[test]
  pub fn basic_expression() {
    let tokens = lex("1 + 2 + x");
    assert_eq!(
      tokens,
      [number(1), add_op(), number(2), add_op(), variable("x")]
    );
  }

  #[test]
  pub fn basic_expression_without_space() {
    let tokens = lex("1+2+x");
    assert_eq!(
      tokens,
      [number(1), add_op(), number(2), add_op(), variable("x")]
    );
  }

  #[test]
  pub fn basic_lookahead() {
    let tokens = lex(": = :=");
    assert_eq!(tokens, [Colon, equal_op(), Assign]);
  }
}
