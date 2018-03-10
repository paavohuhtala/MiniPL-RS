use std::collections::HashMap;

use common::types::{TypeName, Value};

use parsing::ast::*;

use runtime::io::Io;

struct Variable {
  type_of: TypeName,
  value: Option<Value>,
}

pub struct Interpreter<'a, T: Io + 'a> {
  variables: HashMap<String, Variable>,
  io: &'a mut T,
}

impl<'a, T: Io> Interpreter<'a, T> {
  pub fn new(io: &'a mut T) -> Interpreter<'a, T> {
    Interpreter {
      io,
      variables: HashMap::new(),
    }
  }

  fn declare(&mut self, identifier: &str, type_of: TypeName, value: Option<Value>) {
    self
      .variables
      .insert(identifier.to_string(), Variable { type_of, value });
  }

  fn assign(&mut self, identifier: &str, value: Value) {
    let variable = self.variables.get_mut(identifier).unwrap();
    variable.value = Some(value);
  }

  fn evaluate_binary_expression(&self, params: &(Expression, Expression)) -> (Value, Value) {
    let left = self.evaluate_expression(&params.0);
    let right = self.evaluate_expression(&params.1);
    (left, right)
  }

  fn evaluate_expression(&self, expression: &Expression) -> Value {
    match *expression {
      // `Into` casts the literal value into a runtime value
      Expression::Literal(ref value) => value.clone().into(),
      Expression::Variable(ref variable) => {
        let var = *self
          .variables
          .get(variable)
          .as_ref()
          .expect("Type checker will prevent the use of undecleared variables.");
        var
          .value
          .as_ref()
          .expect("Type checker will prevent the use of uninitialised variables.")
          .clone()
      }
      Expression::Add(ref params) => {
        match self.evaluate_binary_expression(params) {
          (Value::IntV(a), Value::IntV(b)) => Value::IntV(a + b),
          (Value::StringV(a), Value::StringV(b)) => Value::StringV(a + &b),
          _ => panic!("Type checker will prevent this."),
        }
      },
      Expression::Sub(ref params) => {
        match self.evaluate_binary_expression(params) {
          (Value::IntV(a), Value::IntV(b)) => Value::IntV(a - b),
          _ => panic!("Type checker will prevent this."),
        }
      }
      Expression::Mul(ref params) => {
        match self.evaluate_binary_expression(params) {
          (Value::IntV(a), Value::IntV(b)) => Value::IntV(a * b),
          _ => panic!("Type checker will prevent this."),
        }
      }
      Expression::Div(ref params) => {
        match self.evaluate_binary_expression(params) {
          (Value::IntV(a), Value::IntV(b)) => Value::IntV(a / b),
          _ => panic!("Type checker will prevent this."),
        }
      }
      Expression::Equal(ref params) => {
        let (left, right) = self.evaluate_binary_expression(params);
        Value::BoolV(left == right)
      }
      Expression::LessThan(ref params) => {
        let (left, right) = self.evaluate_binary_expression(params);
        Value::BoolV(left < right)
      }
      Expression::And(ref params) => {
        match self.evaluate_binary_expression(params) {
          (Value::BoolV(a), Value::BoolV(b)) => Value::BoolV(a && b),
          _ => panic!("Type checker will prevent this.")          
        }
      }
      Expression::Not(ref param) => match self.evaluate_expression(param) {
        Value::BoolV(b) => Value::BoolV(!b),
        _ => panic!("Type checker will prevent this."),
      }
    }
  }

  fn execute_statement(&mut self, statement: &Statement) {
    match *statement {
      // We trust the type checker, so we don't have to check the type at runtime.
      Statement::Declare {
        ref name,
        ref type_of,
        ref initial,
        ..
      } => {
        let initial_value = initial.as_ref().map(|expr| self.evaluate_expression(expr));
        self.declare(name, *type_of, initial_value);
      }
      Statement::Assign(ref name, ref value) => {
        let value = self.evaluate_expression(value);
        self.assign(name, value);
      }
      Statement::Print(ref expr) => {
        let value = self.evaluate_expression(expr);
        self.io.write_line(&value.to_string());
      }
      Statement::Read(ref name) => {
        let str_value = self.io.read_line();

        match *self.variables.get(name).unwrap() {
          Variable {
            type_of: TypeName::StringType,
            ..
          } => {
            self.assign(name, Value::StringV(str_value));
          }
          Variable {
            type_of: TypeName::IntType,
            ..
          } => {
            let as_int = str::parse(&str_value).unwrap();
            self.assign(name, Value::IntV(as_int));
          }
          _ => panic!("Type checker will handle this"),
        }
      }
      Statement::Assert(ref expr) => {
        let value = self.evaluate_expression(expr);
        match value {
          Value::BoolV(true) => return,
          Value::BoolV(false) => self.io.write_line(&format!("ASSERTION FAILED: {:?}", expr)),
          _ => panic!("Type checker will prevent this."),
        }
      },
      Statement::For { ref variable, ref from, ref to, ref run } => {
        let from_value = self.evaluate_expression(from);
        let to_value = self.evaluate_expression(to);

        match (from_value, to_value) {
          (Value::IntV(from), Value::IntV(to)) => {
            for i in from .. (to + 1) {
              // TODO: Mark variable as immutable.
              self.assign(variable, Value::IntV(i));

              for statement in run {
                self.execute_statement(statement);
              }
            }
          },
          _ => panic!("Type checker will prevent this")
        }
      }
    }
  }

  pub fn execute(&mut self, program: Program) {
    for statement in program {
      self.execute_statement(statement);
    }
  }
}
