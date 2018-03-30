use common::errors::*;
use common::types::*;
use common::util::VecExt;

use parsing::ast::*;
use parsing::token::*;
use parsing::token_stream::TokenStream;

pub struct Parser<T: TokenStream> {
  lexer: T,
}

impl<T: TokenStream> Parser<T> {
  pub fn new(lexer: T) -> Parser<T> {
    Parser { lexer }
  }

  fn expect_eq(&mut self, token: &Token) -> Result<(), ParserError> {
    match self.lexer.peek()? {
      ref parsed_token if &parsed_token.token == token => {
        self.lexer.advance();
        Ok(())
      }
      parsed_token => Err(ParserError::UnexpectedToken {
        expected: token.get_kind(),
        was: parsed_token.token.get_kind(),
      }),
    }
  }

  fn expect_identifier(&mut self) -> Result<String, ParserError> {
    let token = self.lexer.peek()?.token;

    match token {
      Token::Identifier(name) => {
        self.advance()?;
        Ok(name.clone())
      }
      other => Err(ParserError::UnexpectedToken {
        expected: TokenKind::IdentifierK,
        was: other.get_kind(),
      }),
    }
  }

  fn expect_type_name(&mut self) -> Result<TypeName, ParserError> {
    let token = self.lexer.peek()?.token;

    match token {
      Token::Type(type_name) => {
        self.advance()?;
        Ok(type_name)
      }
      other => Err(ParserError::UnexpectedToken {
        expected: TokenKind::TypeK,
        was: other.get_kind(),
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

  fn parse_read_statement(&mut self) -> Result<Statement, ParserError> {
    self.expect_eq(&Token::Read)?;

    let identifier = self.expect_identifier()?;

    self.expect_eq(&Token::Semicolon)?;

    Ok(Statement::Read(identifier))
  }

  fn parse_decleration(&mut self) -> Result<Statement, ParserError> {
    self.expect_eq(&Token::Var)?;

    let name = self.expect_identifier()?;

    self.expect_eq(&Token::Colon)?;

    let type_of = self.expect_type_name()?;

    let initial_value = if self.lexer.peek()?.token == Token::Assign {
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

  fn parse_assignment(&mut self) -> Result<Statement, ParserError> {
    let identifier = self.expect_identifier()?;

    self.expect_eq(&Token::Assign)?;

    let value = self.parse_expression()?;

    self.expect_eq(&Token::Semicolon)?;

    Ok(Statement::Assign(identifier, value))
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
    fn create_node(operator: Operator, output: &mut Vec<Expression>) -> Result<(), ParserError> {
      let node = match operator {
        Operator::BinaryOperator(op) => {
          let right = output.pop().ok_or(ParserError::IncompleteExpression)?;
          let left = output.pop().ok_or(ParserError::IncompleteExpression)?;
          let args = Box::new((left, right));
          Expression::BinaryOp(op, args)
        }
        Operator::UnaryOperator(op) => {
          let inner = output.pop().ok_or(ParserError::IncompleteExpression)?;
          Expression::UnaryOp(op, Box::new(inner))
        }
      };

      output.push(node);
      Ok(())
    }

    loop {
      let next = self.lexer.peek()?.token;

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
          // If we encounter an error during the pop_while loop, we'll store the error in this.
          let mut result = Ok(());

          operators.pop_while(|op_op_lparen| {
            match **op_op_lparen {
              OpStackItem::Operator(op) => {
                result = create_node(op, &mut output);
                // If we encountered an error, stop looping.
                !result.is_err()
              }
              OpStackItem::LParen => {
                // We return false to stop the iteration.
                false
              }
            }
          });

          result?;

          // Pop the left parenthesis.
          operators.pop();
        }
        // When an operator is encountered, we need to make sure operator precedence holds.
        // This means that if previously added operator(s) have highers precedence, we must
        // handle them before adding this to the operator stack.
        Token::Operator(op) => {
          self.advance()?;
          // If we encounter an error during the pop_while loop, we'll store the error in this.
          let mut result = Ok(());

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
                  result = create_node(stack_op, &mut output);
                  // If we encountered an error, stop looping.
                  !result.is_err()
                }
              }
            }
          });

          // If there was an error, return early.
          result?;

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
          create_node(op, &mut output)?;
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

  pub fn parse_for(&mut self) -> Result<Statement, ParserError> {
    self.expect_eq(&Token::For)?;

    let variable = self.expect_identifier()?;

    self.expect_eq(&Token::In)?;

    let from = self.parse_expression()?;

    self.expect_eq(&Token::Range)?;

    let to = self.parse_expression()?;

    self.expect_eq(&Token::Do)?;

    let run = self.parse_statement_list()?;

    self.expect_eq(&Token::End)?;
    self.expect_eq(&Token::For)?;
    self.expect_eq(&Token::Semicolon)?;

    Ok(Statement::For {
      variable,
      from,
      to,
      run,
    })
  }

  pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
    let first = self.lexer.peek()?.token;
    match first {
      Token::Print => self.parse_print_statement(),
      Token::Read => self.parse_read_statement(),
      Token::Var => self.parse_decleration(),
      Token::Assert => self.parse_assertion(),
      Token::Identifier(_) => self.parse_assignment(),
      Token::For => self.parse_for(),
      _ => Err(ParserError::MalformedStatement),
    }
  }

  pub fn parse_statement_list(&mut self) -> Result<Vec<StatementWithCtx>, ParserError> {
    let mut statements = Vec::new();

    loop {
      let next = self.lexer.peek()?;
      // If we reached end of file OR the end keyword, stop parsing.
      if next.token == Token::EndOfFile || next.token == Token::End {
        break;
      }

      let start = self.lexer.offset();
      let statement = self.parse_statement()?;
      let end = self.lexer.offset();

      let with_ctx = StatementWithCtx {
        source_position: (start..end),
        statement,
      };
      println!(
        "[{:?}] Statement: {:?}",
        with_ctx.source_position, with_ctx.statement
      );
      statements.push(with_ctx);
    }

    Ok(statements)
  }

  pub fn parse_program(&mut self) -> Result<Vec<StatementWithCtx>, ParserError> {
    let program = self.parse_statement_list()?;
    self.expect_eq(&Token::EndOfFile)?;
    Ok(program)
  }
}

#[cfg(test)]
mod tests {
  use common::errors::ParserError::*;
  use parsing::parser_test_util::*;
  use parsing::token::TokenKind::*;

  #[test]
  fn parse_invalid_binary_expr() {
    let result = parse_expr("1 +");
    assert_match!(result => Err(IncompleteExpression));
  }

  #[test]
  fn reserved_keyword_var() {
    let result = parse_stmnt("var var : int := 10;");
    assert_match!(result => Err(UnexpectedToken { expected: _, was: VarK }));
  }

  #[test]
  fn reserved_keyword_int() {
    let result = parse_stmnt("var int : int := 10;");
    assert_match!(result => Err(UnexpectedToken { expected: _, was: TypeK }));
  }
}
