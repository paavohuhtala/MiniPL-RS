use std::collections::HashMap;

use common::types::{TypeName, Value};

use diagnostics::file_context::FileContextSource;

use parsing::ast::*;

use runtime::io::Io;

struct Variable {
  type_of: TypeName,
  value: Value,
}

pub struct Interpreter<'a, T: Io + 'a> {
  variables: HashMap<String, Variable>,
  ctx: &'a FileContextSource,
  io: &'a mut T,
}

impl<'a, T: Io> Interpreter<'a, T> {
  pub fn new(io: &'a mut T, ctx: &'a FileContextSource) -> Interpreter<'a, T> {
    Interpreter {
      io,
      ctx,
      variables: HashMap::new(),
    }
  }

  fn declare(&mut self, identifier: &str, type_of: TypeName, value: Value) {
    self
      .variables
      .insert(identifier.to_string(), Variable { type_of, value });
  }

  fn assign(&mut self, identifier: &str, value: Value) {
    let variable = self.variables.get_mut(identifier).unwrap();
    variable.value = value;
  }

  fn evaluate_binary_expression(&self, params: &(Expression, Expression)) -> (Value, Value) {
    let left = self.evaluate_expression(&params.0);
    let right = self.evaluate_expression(&params.1);
    (left, right)
  }

  fn evaluate_expression(&self, expression: &Expression) -> Value {
    use common::types::BinaryOperator::*;
    use common::types::UnaryOperator::*;
    use common::types::Value::*;
    use parsing::ast::Expression::*;

    match *expression {
      // `Into` casts the literal value into a runtime value
      Literal(ref value) => value.clone().into(),
      Variable(ref variable) => {
        let var = *self
          .variables
          .get(variable)
          .as_ref()
          .expect("Type checker will prevent the use of undeclared variables.");
        var.value.clone()
      }
      BinaryOp(ref op, ref params) => {
        let (left, right) = self.evaluate_binary_expression(params);
        match (*op, left, right) {
          (Add, IntV(a), IntV(b)) => IntV(a + b),
          (Add, StringV(a), StringV(b)) => StringV(a + &b),
          (Sub, IntV(a), IntV(b)) => IntV(a - b),
          (Mul, IntV(a), IntV(b)) => IntV(a * b),
          (Div, IntV(a), IntV(b)) => IntV(a / b),
          (Equal, a, b) => BoolV(a == b),
          (LessThan, a, b) => BoolV(a < b),
          (And, BoolV(a), BoolV(b)) => BoolV(a && b),
          _ => panic!("Type checker will prevent this."),
        }
      }
      UnaryOp(ref op, ref param) => {
        let inner = self.evaluate_expression(param);
        match (*op, inner) {
          (Not, BoolV(x)) => BoolV(!x),
          _ => panic!("Type checker will prevent this."),
        }
      }
    }
  }

  fn execute_statement(&mut self, statement: &StatementWithCtx) {
    match statement.statement {
      // We trust the type checker, so we don't have to check the type at runtime.
      Statement::Declare {
        ref name,
        ref type_of,
        ref initial,
        ..
      } => {
        let initial_value = initial
          .as_ref()
          .map(|expr| self.evaluate_expression(expr))
          .unwrap_or(type_of.get_default_value());
        self.declare(name, *type_of, initial_value);
      }
      Statement::Assign(ref name, ref value) => {
        let value = self.evaluate_expression(value);
        self.assign(name, value);
      }
      Statement::Print(ref expr) => {
        let value = self.evaluate_expression(expr);
        self.io.write(&value.to_string());
      }
      Statement::Read(ref name) => {
        let str_value = self.io.read_line();

        match self.variables[name] {
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
          Value::BoolV(false) => {
            let source_quote = self.ctx.get_source_quote(&statement.source_position);
            self
              .io
              .write(&format!("ASSERTION FAILED:\n{}", source_quote))
          }
          _ => panic!("Type checker will prevent this."),
        }
      }
      Statement::For {
        ref variable,
        ref from,
        ref to,
        ref run,
      } => {
        let from_value = self.evaluate_expression(from);
        let to_value = self.evaluate_expression(to);

        match (from_value, to_value) {
          (Value::IntV(from), Value::IntV(to)) => for i in from..(to + 1) {
            self.assign(variable, Value::IntV(i));

            for statement in run {
              self.execute_statement(&statement);
            }
          },
          _ => panic!("Type checker will prevent this"),
        }
      }
    }
  }

  pub fn execute(&mut self, program: Program) {
    for statement in program {
      self.execute_statement(&statement);
    }
  }
}
