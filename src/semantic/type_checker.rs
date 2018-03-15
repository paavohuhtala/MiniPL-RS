use std::collections::HashMap;

use common::types::*;
use parsing::ast::*;

#[derive(Debug, PartialEq)]
pub enum TypeError {
  RedeclaredIdentifier(String),
  UndeclaredIdentifier(String),
  InvalidAssignment {
    name: String,
    was: TypeName,
    new_type: TypeName,
  },
  IncompatibleTypes {
    expected: TypeName,
    was: TypeName,
  },
  InvalidBinaryOp(BinaryOperator, TypeName, TypeName),
  InvalidUnaryOp(UnaryOperator, TypeName),
  PrintArgumentError(TypeName),
  ReadArgumentError(TypeName),
  AssertArgumentError(TypeName),
  AssignToImmutable(String),
}

struct Symbol {
  type_of: TypeName,
  is_mutable: bool,
}

struct TypeCheckingContext {
  symbols: HashMap<String, Symbol>,
}

impl TypeCheckingContext {
  fn get_literal_type(&self, literal: &LiteralValue) -> TypeName {
    use self::LiteralValue::*;
    match *literal {
      StringLiteral(_) => TypeName::StringType,
      IntLiteral(_) => TypeName::IntType,
    }
  }

  fn evaluate_binary_expression_type(
    &self,
    params: &(Expression, Expression),
  ) -> Result<(TypeName, TypeName), TypeError> {
    let left = self.evaluate_expression_type(&params.0)?;
    let right = self.evaluate_expression_type(&params.1)?;
    Ok((left, right))
  }

  fn evaluate_variable_type(&self, variable: &str) -> Result<TypeName, TypeError> {
    if let Some(symbol) = self.symbols.get(variable) {
      Ok(symbol.type_of)
    } else {
      Err(TypeError::UndeclaredIdentifier(variable.to_string()))
    }
  }

  fn evaluate_expression_type(&self, expression: &Expression) -> Result<TypeName, TypeError> {
    use self::Expression::*;
    use self::TypeError::*;
    use common::types::TypeName::*;
    use common::types::BinaryOperator::*;
    use common::types::UnaryOperator::*;

    match *expression {
      Literal(ref literal) => Ok(self.get_literal_type(literal)),
      Variable(ref variable) => self.evaluate_variable_type(variable),
      BinaryOp(ref op, ref params) => {
        let (left, right) = self.evaluate_binary_expression_type(params)?;
        match (*op, left, right) {
          (Add, IntType, IntType)
          | (Sub, IntType, IntType)
          | (Mul, IntType, IntType)
          | (Div, IntType, IntType) => Ok(IntType),

          (Add, StringType, StringType) => Ok(StringType),

          (Equal, x, y) if x == y => Ok(BoolType),
          (LessThan, x, y) if x == y => Ok(BoolType),
          (And, BoolType, BoolType) => Ok(BoolType),

          (op, left, right) => Err(InvalidBinaryOp(op, left, right)),
        }
      }
      UnaryOp(ref op, ref param) => {
        let inner = self.evaluate_expression_type(param)?;
        match (*op, inner) {
          (Not, BoolType) => Ok(BoolType),
          (op, inner) => Err(InvalidUnaryOp(op, inner)),
        }
      }
    }
  }

  fn assert_types_equal(expected: TypeName, is: TypeName) -> Result<(), TypeError> {
    if expected != is {
      Err(TypeError::IncompatibleTypes { expected, was: is })
    } else {
      Ok(())
    }
  }

  fn set_variable_mutability(&mut self, name: &str, is_mutable: bool) {
    let symbol = self
      .symbols
      .get_mut(name)
      .expect("Symbol should always be defined at this point.");
    symbol.is_mutable = is_mutable;
  }

  fn assert_mutable(&self, name: &str) -> Result<(), TypeError> {
    let symbol = self
      .symbols
      .get(name)
      .expect("Symbol should always be defined at this point");
    if !symbol.is_mutable {
      Err(TypeError::AssignToImmutable(name.to_string()))
    } else {
      Ok(())
    }
  }

  fn type_check_statement(&mut self, statement: &Statement) -> Result<(), TypeError> {
    match *statement {
      Statement::Declare {
        ref name,
        ref type_of,
        ref initial,
      } => {
        // If the variable already exists in the symbol table, report error.
        if self.symbols.get(name).is_some() {
          return Err(TypeError::RedeclaredIdentifier(name.to_string()));
        }

        // If the variable has been initialised, make sure it matches the type annotation.
        if let Some(ref initial_value) = *initial {
          let initial_value_type = self.evaluate_expression_type(initial_value)?;
          Self::assert_types_equal(*type_of, initial_value_type)?;
        }

        // Add the symbol to the symbol table.
        self.symbols.insert(
          name.to_string(),
          Symbol {
            type_of: *type_of,
            is_mutable: true,
          },
        );
        Ok(())
      }
      Statement::Assign(ref name, ref value) => {
        let variable_type = self.evaluate_variable_type(name)?;
        let value_type = self.evaluate_expression_type(value)?;
        Self::assert_types_equal(variable_type, value_type)?;
        self.assert_mutable(name)
      }
      Statement::Print(ref expr) => {
        // Only strings and ints can be printed.
        match self.evaluate_expression_type(expr)? {
          TypeName::IntType | TypeName::StringType => Ok(()),
          TypeName::BoolType => Err(TypeError::PrintArgumentError(TypeName::BoolType)),
        }
      }
      Statement::Read(ref name) => {
        // Make sure the variable exists, and is either an int or string.
        match self.evaluate_variable_type(name)? {
          TypeName::IntType | TypeName::StringType => Ok(()),
          TypeName::BoolType => Err(TypeError::ReadArgumentError(TypeName::BoolType)),
        }
      }
      Statement::Assert(ref expr) => match self.evaluate_expression_type(expr)? {
        TypeName::BoolType => Ok(()),
        other => Err(TypeError::AssertArgumentError(other)),
      },
      Statement::For {
        ref variable,
        ref from,
        ref to,
        ref run,
      } => {
        // Loop variable must be a mutable integer
        Self::assert_types_equal(TypeName::IntType, self.evaluate_variable_type(variable)?)?;
        self.assert_mutable(variable)?;

        Self::assert_types_equal(TypeName::IntType, self.evaluate_expression_type(from)?)?;
        Self::assert_types_equal(TypeName::IntType, self.evaluate_expression_type(to)?)?;

        self.set_variable_mutability(variable, false);

        for statement in run {
          self.type_check_statement(&statement.statement)?;
        }

        self.set_variable_mutability(variable, true);

        Ok(())
      }
    }
  }
}

pub fn type_check(program: &[StatementWithCtx]) -> Result<(), TypeError> {
  let mut context = TypeCheckingContext {
    symbols: HashMap::new(),
  };

  for statement in program {
    context.type_check_statement(&statement.statement)?;
  }

  Ok(())
}

#[cfg(test)]
#[macro_use]
mod tests {
  use super::*;
  use common::types::TypeName::*;
  use parsing::ast_test_util;
  use semantic::test_util::*;

  fn ctx() -> TypeCheckingContext {
    TypeCheckingContext {
      symbols: HashMap::new(),
    }
  }

  macro_rules! type_shorthand {
    (int) => (TypeName::IntType);
    (boolean) => (TypeName::BoolType);
    (string) => (TypeName::StringType);
  }

  macro_rules! operator_tests {
    {
      $($op: ident { $(($a_ok: ident, $b_ok: ident) -> $res: ident),* })*
    } => {
      $(
        #[test]
        fn $op() {
          let ctx = ctx();

          for a in &[IntType, StringType, BoolType] {
            for b in &[IntType, StringType, BoolType] {
              let result = ctx.evaluate_expression_type(
                &ast_test_util::$op(expr_of_type(*a), expr_of_type(*b))
              );
              $(
                if *a == type_shorthand!($a_ok) && *b == type_shorthand!($b_ok) {
                  assert_eq!(
                    Ok(type_shorthand!($res)),
                    result,
                    stringify!(($a_ok, $b_ok) -> $res)
                  );
                  continue;
                }
              )*;
              assert_match!(result => Err(_), "Should fail with types ({:?}, {:?}).", a, b);
            }
          }
        }
      )*
    };
  }

  operator_tests! {
    add {
      (int, int) -> int,
      (string, string) -> string
    }
    sub {
      (int, int) -> int
    }
    mul {
      (int, int) -> int
    }
    div {
      (int, int) -> int
    }
    eq {
      (int, int) -> boolean,
      (string, string) -> boolean,
      (boolean, boolean) -> boolean
    }
    lt {
      (int, int) -> boolean,
      (string, string) -> boolean,
      (boolean, boolean) -> boolean
    }
    and {
      (boolean, boolean) -> boolean
    }
  }
}
