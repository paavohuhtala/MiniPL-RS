use common::types::*;
use common::errors::*;
use common::util::VecExt;

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

impl<T: TokenSource> Parser<T> {
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
    #[derive(PartialEq)]
    enum OpStackItem {
      Operator(Operator),
      LParen,
    }

    let mut output: Vec<Expression> = Vec::new();
    let mut operators: Vec<OpStackItem> = Vec::new();

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
            Operator::Equal => Expression::Equal(args),
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
        Token::Identifier(identifier) => {
          self.advance()?;
          output.push(Expression::Variable(identifier));
        }
        Token::LParen => {
          self.advance()?;
          operators.push(OpStackItem::LParen);
        }
        Token::RParen => {
          self.advance()?;
          operators.pop_while(|op_op_lparen| {
            match **op_op_lparen {
              OpStackItem::Operator(op) => {
                create_node(op, &mut output);
                true
              }
              OpStackItem::LParen => {
                // We return false (even though we are going to pop this later) in order
                // to stop the iteration.
                false
              }
            }
          });

          // Pop the left parenthesis.
          operators.pop();
        }
        // When an operator is encountered, we need to make sure operator precedence holds.
        // This means that if previously added operator(s) have highers precedence, we must
        // handle them before adding this to the operator stack.
        Token::Operator(op) => {
          self.advance()?;

          // Go through the operator stack, and while there are operators with higher precedence
          // pop them from the stack and add them to the AST.
          // The closure parameter is a predicate with side effects - normally that could be
          // an issue, but since the iterator is consumed immediately it should be fine.
          operators.pop_while(|op_op_lparen| {
            match **op_op_lparen {
              OpStackItem::LParen => false,
              OpStackItem::Operator(stack_op) => {
                // When we encounter an operator with lower or equal precedence, stop.
                if stack_op.get_precedence() <= op.get_precedence() {
                  false
                } else {
                  // Mark this operator to be popped, and create the AST node for it.
                  create_node(stack_op, &mut output);
                  true
                }
              }
            }
          });

          // Finally, push the operator to the stack.
          operators.push(OpStackItem::Operator(op));
        }
        _ => break,
      }
    }

    for op_or_lparen in operators.iter().rev() {
      match *op_or_lparen {
        OpStackItem::LParen => return Err(ParserError::MissingRParen),
        OpStackItem::Operator(op) => {
          create_node(op, &mut output);
        }
      }
    }

    println!("Expression: {:?}", output);

    if output.len() != 1 {
      Err(ParserError::MalformedStatement)
    } else {
      Ok(output.remove(0))
    }
  }

  pub fn parse_assertion(&mut self) -> Result<Statement, ParserError> {
    self.expect_eq(&Token::Assert)?;
    let assertion = self.parse_expression()?;
    self.expect_eq(&Token::Semicolon)?;
    Ok(Statement::Assert(assertion))
  }

  pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
    let first = self.lexer.peek()?;
    match first {
      Token::Print => self.parse_print_statement(),
      Token::Var => self.parse_decleration(),
      Token::Assert => self.parse_assertion(),
      _ => Err(ParserError::MalformedStatement),
    }
  }

  pub fn parse_statement_list(&mut self) -> Result<Vec<Statement>, ParserError> {
    let mut statements = Vec::new();

    loop {
      // If we reached end of file, stop parsing.
      if self.lexer.peek()? == Token::EndOfFile {
        break;
      }

      let statement = self.parse_statement()?;

      println!("Statement: {:?}", statement);

      statements.push(statement);

      if self.lexer.reached_end() {
        break;
      }
    }

    self.expect_eq(&Token::EndOfFile)?;

    Ok(statements)
  }
}
