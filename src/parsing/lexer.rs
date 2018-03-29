use common::errors::*;
use common::types::*;

use parsing::char_stream::*;
use parsing::token::*;
use parsing::token_stream::TokenStream;
use parsing::util::*;

fn read_string_literal(input: &mut CharStream) -> Result<Token, LexerError> {
  if input.peek()? != '"' {
    return Err(LexerError::UnknownLexeme);
  }
  input.advance();

  let mut chars = Vec::new();

  loop {
    if input.reached_end() {
      return Err(LexerError::UnterminatedStringLiteral);
    }

    match input.next()? {
      '\\' => {
        let escape_char = match input.next()? {
          '\\' => '\\',
          '"' => '"',
          'r' => '\r',
          'n' => '\n',
          't' => '\t',
          other => return Err(LexerError::UnknownEscapeCode(other.to_string())),
        };
        chars.push(escape_char);
      }
      '"' => break,
      other => chars.push(other),
    };
  }

  let contents = chars.iter().collect();
  Ok(Token::Literal(LiteralValue::StringLiteral(contents)))
}

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

      if !is_valid_identifier(&name) {
        Err(LexerError::UnknownLexeme)
      } else {
        Ok(Token::Identifier(name))
      }
    }
  }
}

fn parse_single_char_token(ch: char) -> Token {
  use common::types::BinaryOperator::*;
  use common::types::Operator::*;
  use common::types::UnaryOperator::*;
  use parsing::token::Token::*;

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

  println!("[{}] Token: {:?}", offset, token);
  token
}

pub struct BufferedLexer {
  stream: CharStream,
  token: Option<TokenWithCtx>,
}

impl BufferedLexer {
  pub fn new(stream: CharStream) -> BufferedLexer {
    BufferedLexer {
      stream,
      token: None,
    }
  }
}

impl TokenStream for BufferedLexer {
  /// If the buffer contains a token, remove it. Otherwise advance the stream.
  fn advance(&mut self) {
    if self.token.is_none() {
      self.stream.advance();
    } else {
      self.token = None;
    }
  }

  /// Returns true if the buffer contains a token or the stream has characters remaining.
  fn reached_end(&self) -> bool {
    self.token.is_none() && self.stream.reached_end()
  }

  fn offset(&self) -> usize {
    match self.token {
      Some(ref token) => token.offset,
      _ => self.stream.offset,
    }
  }

  /// Tries to get the next token, either from the buffer or from the character stream.
  fn peek(&mut self) -> Result<TokenWithCtx, LexerError> {
    if self.token.is_some() {
      Ok(self.token.clone().unwrap())
    } else {
      let next = next_lexeme(&mut self.stream)?;
      self.token = Some(next.clone());
      Ok(next)
    }
  }

  /// Same as `peek()`, followed by `advance()`.
  fn next(&mut self) -> Result<TokenWithCtx, LexerError> {
    let token = self.peek()?;
    self.advance();
    Ok(token)
  }
}

#[cfg(test)]
mod tests {
  use common::errors::LexerError;
  use parsing::lexer_test_util::*;
  use parsing::token::Token::*;

  #[test]
  pub fn basic_expression() {
    let tokens = lex("1 + 2 + x").expect("Should parse.");
    assert_eq!(
      tokens,
      [number(1), add_op(), number(2), add_op(), variable("x")]
    );
  }

  #[test]
  pub fn basic_expression_without_space() {
    let tokens = lex("1+2+x").expect("Should parse.");
    assert_eq!(
      tokens,
      [number(1), add_op(), number(2), add_op(), variable("x")]
    );
  }

  #[test]
  pub fn basic_lookahead() {
    let tokens = lex(": = :=").expect("Should parse.");
    assert_eq!(tokens, [Colon, equal_op(), Assign]);
  }

  #[test]
  pub fn string_escape_codes() {
    let tokens = lex(r#""\r\n\\\"\t""#).expect("Should parse.");
    assert_eq!(tokens, [string("\r\n\\\"\t")]);
  }

  #[test]
  pub fn malformed_string() {
    let result = lex(r#""Hello, world!; stuff"#);
    assert_match!(result => Err(LexerError::UnterminatedStringLiteral));
  }
}
