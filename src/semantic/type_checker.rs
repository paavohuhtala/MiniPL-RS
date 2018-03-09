use std::collections::HashMap;

use common::types::*;
use parsing::ast::*;

#[derive(Debug)]
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
  PrintArgumentError(TypeName),
  AssertArgumentError(TypeName)
}

struct TypeCheckingContext {
  symbols: HashMap<String, TypeName>,
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

  fn evaluate_expression_type(&self, expression: &Expression) -> Result<TypeName, TypeError> {
    use self::Expression::*;
    match *expression {
      Literal(ref literal) => Ok(self.get_literal_type(literal)),
      Variable(ref variable) => {
        if let Some(symbol_type) = self.symbols.get(variable) {
          Ok(*symbol_type)
        } else {
          Err(TypeError::UndeclaredIdentifier(variable.to_string()))
        }
      }
      Add(ref param_box) | Sub(ref param_box) | Mul(ref param_box) | Div(ref param_box) => {
        let (left, right) = self.evaluate_binary_expression_type(param_box)?;
        Self::assert_types_equal(left, right)?;
        Ok(left)
      },
      Equal(ref param_box) => {
        let (left, right) = self.evaluate_binary_expression_type(param_box)?;
        Self::assert_types_equal(left, right)?;
        Ok(TypeName::BoolType)
      },
      Not(ref param_box) => {
        let inner = self.evaluate_expression_type(param_box)?;
        Self::assert_types_equal(TypeName::BoolType, inner)?;
        Ok(TypeName::BoolType)
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

  fn type_check_statement(&mut self, statement: &Statement) -> Result<(), TypeError> {
    match statement {
      &Statement::Declare {
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
        self.symbols.insert(name.to_string(), *type_of);
        Ok(())
      }
      &Statement::Print(ref expr) => {
        // Only strings and ints can be printed.
        match self.evaluate_expression_type(expr)? {
          TypeName::IntType | TypeName::StringType => Ok(()),
          TypeName::BoolType => Err(TypeError::PrintArgumentError(TypeName::BoolType)),
        }
      },
      &Statement::Assert(ref expr) => {
        match self.evaluate_expression_type(expr)? {
          TypeName::BoolType => Ok(()),
          other => Err(TypeError::AssertArgumentError(other))
        }
      }
      other => panic!("Unsupported statement: {:?}", other),
    }
  }
}

pub fn type_check(program: &[Statement]) -> Result<(), TypeError> {
  let mut context = TypeCheckingContext {
    symbols: HashMap::new(),
  };

  for statement in program {
    context.type_check_statement(statement)?;
  }

  Ok(())
}
