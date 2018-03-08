use common::types::*;
use common::errors::*;

use parsing::ast::*;
use parsing::token_source::TokenSource;

pub struct Parser<T: TokenSource> {
  lexer: T,
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

impl<T> Parser<T>
where
  T: TokenSource,
{
  pub fn new(lexer: T) -> Parser<T> {
    Parser { lexer }
  }

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

  // Parses expressions using a modified version of the shunting yard algorithm.
  pub fn parse_expression(&mut self) -> Result<Expression, ParserError> {
    let mut output: Vec<Expression> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();

    // We don't need to access this from outside, so this function can be local.
    fn create_node(operator: Operator, output: &mut Vec<Expression>) {
      let node = match operator.get_arity() {
        Arity::Binary => {
          let left = output.pop().unwrap();
          let right = output.pop().unwrap();
          let args = Box::new((left, right));

          match operator {
            Operator::Add => Expression::Add(args),
            Operator::Sub => Expression::Sub(args),
            Operator::Mul => Expression::Mul(args),
            Operator::Div => Expression::Div(args),
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
        // Literals are just pushed to the output stack
        Token::Literal(value) => {
          self.advance()?;
          output.push(Expression::Literal(value));
        }
        // When an operator is encountered, we need to make sure operator precedence holds.
        // This means that if previously added operator(s) have highers precedence, we must
        // handle them before adding this to the operator stack.
        Token::Operator(op) => {
          self.advance()?;

          // To get around lifetime limitations, we'll do this in two parts:

          let mut indices_to_pop = 0;

          // 1. Iterate from back to front.
          for op_to_pop in operators.iter().rev() {
            // If we encounter an operator with lower or equal precedence, stop.
            if op_to_pop.get_precedence() <= op.get_precedence() {
              break;
            }

            // Mark this operator to be popped, and create the AST node for it.
            indices_to_pop += 1;
            create_node(*op_to_pop, &mut output);
          }

          // 2. Drop `indices_to_pop` entries, from back to front.
          if indices_to_pop > 0 {
            let operators_length = operators.len();
            operators.drain(operators_length - indices_to_pop..operators_length);
            assert_eq!(operators_length - indices_to_pop, operators.len());
          }

          // Finally, push the operator to the stack.
          operators.push(op);
        }
        _ => break,
      }
    }

    for operator in operators.iter().rev() {
      create_node(*operator, &mut output);
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
