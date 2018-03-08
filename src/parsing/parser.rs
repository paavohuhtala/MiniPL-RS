use common::types::*;
use common::errors::*;

use parsing::ast::*;

use parsing::lexer::BufferedLexer;

pub struct Parser<'a> {
  lexer: BufferedLexer<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(lexer: BufferedLexer<'a>) -> Parser {
    Parser { lexer }
  }
}

impl Token {
  pub fn expect_identifier(&self) -> Result<String, ParserError> {
    match self {
      &Token::Identifier(ref identifier) => Ok(identifier.clone()),
      other => Err(ParserError::UnexpectedToken {
        expected: TokenKind::IdentifierK,
        was: other.get_kind(),
      }),
    }
  }

  pub fn expect_type(&self) -> Result<TypeName, ParserError> {
    match self {
      &Token::Type(ref type_name) => Ok(*type_name),
      other => Err(ParserError::UnexpectedToken {
        expected: TokenKind::TypeK,
        was: other.get_kind(),
      }),
    }
  }
}

type ParseResult<T> = Result<T, ParserError>;

impl<'a> Parser<'a> {
  fn expect_eq(&mut self, token: &Token) -> Result<(), ParserError> {
    match self.lexer.peek()? {
      ref parsed_token if parsed_token == token => {
        self.lexer.advance();
        Ok(())
      }
      parsed_token => Err(ParserError::UnexpectedToken {
        expected: token.get_kind(),
        was: parsed_token.get_kind(),
      }),
    }
  }

  fn expect_kind(&mut self, kind: TokenKind) -> Result<(), ParserError> {
    match self.lexer.peek()? {
      ref parsed_token if parsed_token.get_kind() == kind => {
        self.lexer.advance();
        Ok(())
      }
      parsed_token => Err(ParserError::UnexpectedToken {
        expected: kind,
        was: parsed_token.get_kind(),
      }),
    }
  }

  fn advance(&mut self) -> Result<(), ParserError> {
    // Zero out the value (replace with empty tuple) and cast the error with .into()
    self.lexer.next().map(|_| ()).map_err(|err| err.into())
  }

  fn parse_print_statement(&mut self) -> Result<Statement, ParserError> {
    self.expect_eq(&Token::Print)?;
    let value = self.parse_expression()?;
    self.expect_eq(&Token::Semicolon)?;
    Ok(Statement::Print(value))
  }

  fn parse_decleration(&mut self) -> Result<Statement, ParserError> {
    self.expect_eq(&Token::Var)?;

    let name = self.lexer.peek()?.expect_identifier()?;
    self.advance()?;

    self.expect_eq(&Token::Colon)?;

    let type_of = self.lexer.peek()?.expect_type()?;
    self.advance()?;

    let initial_value = if self.lexer.peek()? == Token::Assign {
      self.advance()?;

      let expression = self.parse_expression()?;
      Some(expression)
    } else {
      None
    };

    self.expect_eq(&Token::Semicolon)?;

    Ok(Statement::Declare {
      name,
      type_of,
      initial: initial_value,
    })
  }

  // Implements the shunting yard algorithm
  pub fn parse_expression(&mut self) -> Result<Expression, ParserError> {
    let mut output: Vec<Expression> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();

    fn push_node(operator: Operator, output: &mut Vec<Expression>) {
      let node = match operator.get_arity() {
        Arity::Binary => {
          let left = output.pop().unwrap();
          let right = output.pop().unwrap();
          let args = Box::new((left, right));

          match operator {
            Operator::Add => Expression::Add(args),
            Operator::Sub => Expression::Sub(args),
            _ => panic!("Not implemented yet."),
          }
        }
        Arity::Unary => panic!("Not implemented yet."),
      };

      output.push(node);
    }

    loop {
      let next = self.lexer.peek()?;

      match next {
        // Numbers and variables are pushed to the output stack
        Token::Literal(value) => {
          self.advance()?;
          output.push(Expression::Literal(value));
        }
        Token::Operator(op) => {
          self.advance()?;
          operators.push(op);
        }
        _ => break,
      }
    }

    for operator in operators.iter().rev() {
      push_node(*operator, &mut output);
    }

    println!("Expression: {:?}", output);

    if output.len() != 1 {
      Err(ParserError::MalformedStatement)
    } else {
      Ok(output.remove(0))
    }
  }

  pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
    let first = self.lexer.peek()?;
    match first {
      Token::Print => self.parse_print_statement(),
      Token::Var => self.parse_decleration(),
      _ => Err(ParserError::MalformedStatement),
    }
  }

  pub fn parse_statement_list(&mut self) -> Result<Vec<Statement>, ParserError> {
    let mut statements = Vec::new();

    loop {
      let statement = self.parse_statement()?;

      println!("Statement: {:?}", statement);

      statements.push(statement);

      if self.lexer.reached_end() {
        break;
      }
    }

    Ok(statements)
  }
}
