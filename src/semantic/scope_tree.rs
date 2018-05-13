use std::collections::HashMap;

use parsing::ast::*;
use semantic::types::Symbol;

type ScopeKey = usize;

type StatementList<'a> = &'a [StatementWithCtx];

#[derive(Debug)]
pub struct Scope<'a> {
  pub key: ScopeKey,
  parent: ScopeKey,
  symbols: HashMap<String, Symbol>,
  code: StatementList<'a>,
}

impl<'a> Scope<'a> {
  fn _get_parent(&self) -> Option<ScopeKey> {
    if self.key == self.parent {
      None
    } else {
      Some(self.parent)
    }
  }
}

#[derive(Debug)]
pub struct ScopeTree<'a> {
  scopes: HashMap<ScopeKey, Scope<'a>>,
  next_scope_id: ScopeKey,
}

impl<'a> ScopeTree<'a> {
  fn next_scope_id(&mut self) -> ScopeKey {
    let id = self.next_scope_id;
    self.next_scope_id += 1;
    id
  }

  fn new_scope(&mut self, code: StatementList<'a>, parent: ScopeKey) -> Scope<'a> {
    Scope {
      key: self.next_scope_id(),
      parent,
      symbols: HashMap::new(),
      code,
    }
  }

  fn visit_statement(&mut self, statement: &'a StatementWithCtx, parent: ScopeKey) {
    let block_or_none = match statement.statement {
      Statement::For { ref run, .. } => Some(run),
      _ => None,
    };

    if let Some(block) = block_or_none {
      let scope = self.new_scope(&block, parent);
      let scope_key = scope.key;
      self.scopes.insert(scope.key, scope);

      for statement in block {
        self.visit_statement(&statement, scope_key);
      }
    }
  }

  pub fn from_program(program: Program<'a>) -> ScopeTree<'a> {
    let mut scope_tree = ScopeTree {
      scopes: HashMap::new(),
      next_scope_id: 1,
    };

    let global_scope = Scope {
      key: 0,
      parent: 0,
      symbols: HashMap::new(),
      code: program,
    };

    scope_tree.scopes.insert(0, global_scope);

    for statement in program {
      scope_tree.visit_statement(statement, 0);
    }

    println!("{:#?}", scope_tree);

    scope_tree
  }
}
