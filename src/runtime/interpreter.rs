use std::collections::HashMap;

use common::types::Value;

use parsing::ast::*;

use runtime::io::Io;

pub struct Interpreter<T: Io> {
  variables: HashMap<String, Option<Value>>,
  io: T,
}

impl<T: Io> Interpreter<T> {
  pub fn new(io: T) -> Interpreter<T> {
    Interpreter {
      io,
      variables: HashMap::new(),
    }
  }

  fn assign(&mut self, identifier: &str, value: Option<Value>) {
    self.variables.insert(identifier.to_string(), value);
  }

  fn evaluate_binary_expression(&self, params: &(Expression, Expression)) -> (Value, Value) {
    let left = self.evaluate_expression(&params.0);
    let right = self.evaluate_expression(&params.1);
    (left, right)
  }

  fn evaluate_expression(&self, expression: &Expression) -> Value {
    match *expression {
      // Into casts the literal value into a runtime value
      Expression::Literal(ref value) => value.clone().into(),
      Expression::Variable(ref variable) => {
        let var = self
          .variables
          .get(variable)
          .expect("Type checker will prevent the use of undecleared variables.");
        var
          .clone()
          .expect("Type checker will prevent the use of uninitialised variables.")
      }
      Expression::Add(ref params) => {
        let (left, right) = self.evaluate_binary_expression(params);

        match (left, right) {
          (Value::IntV(a), Value::IntV(b)) => Value::IntV(a + b),
          (Value::StringV(a), Value::StringV(b)) => Value::StringV(a + &b),
          _ => panic!("Type checker will prevent this."),
        }
      }
      Expression::Equal(ref params) => {
        let (left, right) = self.evaluate_binary_expression(params);
        Value::BoolV(left == right)
      }
      ref _other => panic!("Not supported."),
    }
  }

  fn execute_statement(&mut self, statement: &Statement) {
    match *statement {
      // We trust the type checker, so we don't have to check the type at runtime.
      Statement::Declare {
        ref name,
        ref initial,
        ..
      } => {
        let initial_value = initial.as_ref().map(|expr| self.evaluate_expression(expr));
        self.assign(name, initial_value);
      }
      Statement::Print(ref expr) => {
        let value = self.evaluate_expression(expr);
        self.io.write_line(&value.to_string());
      }
      Statement::Assert(ref expr) => {
        let value = self.evaluate_expression(expr);
        match value {
          Value::BoolV(true) => return,
          Value::BoolV(false) => self.io.write_line(&format!("ASSERTION FAILED: {:?}", expr)),
          _ => panic!("Type checker will prevent this."),
        }
      }
      _ => panic!(),
    }
  }

  pub fn execute(&mut self, program: Program) {
    for statement in program {
      self.execute_statement(statement);
    }
  }
}
