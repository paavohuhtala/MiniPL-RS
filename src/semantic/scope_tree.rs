use std::collections::{HashMap, HashSet};

use parsing::ast::*;
use semantic::types::Symbol;

pub type ScopeKey = usize;

#[derive(Debug)]
pub struct Scope {
  pub key: ScopeKey,
  parent: ScopeKey,
  children: HashSet<ScopeKey>,
  symbols: HashMap<String, Symbol>,
}

impl Scope {
  fn get_parent_key(&self) -> Option<ScopeKey> {
    if self.key == self.parent {
      None
    } else {
      Some(self.parent)
    }
  }
}

#[derive(Debug)]
pub struct ScopeTree {
  scopes: HashMap<ScopeKey, Scope>,
  scope_symbols_cache: HashMap<ScopeKey, HashMap<String, (ScopeKey, Symbol)>>,
  next_scope_id: ScopeKey,
}

impl ScopeTree {
  fn next_scope_id(&mut self) -> ScopeKey {
    let id = self.next_scope_id;
    self.next_scope_id += 1;
    id
  }

  fn get_parent(&self, scope: &Scope) -> Option<&Scope> {
    self.scopes.get(&scope.get_parent_key()?)
  }

  pub fn get_global_scope(&self) -> &Scope {
    self
      .scopes
      .get(&0)
      .expect("Global scope should always exist")
  }

  pub fn get_symbol(&mut self, scope: ScopeKey, symbol_name: &str) -> Option<&Symbol> {
    self
      .get_symbols_in_scope(scope)
      .get(symbol_name)
      .map(|(_, symbol)| symbol)

    /*self
      .scopes
      .get(&scope)
      .and_then(|s| s.symbols.get(symbol_name))*/
  }

  pub fn get_symbol_mut(&mut self, scope: ScopeKey, symbol_name: &str) -> Option<&mut Symbol> {
    let (scope, _) = *self.get_symbols_in_scope(scope).get(symbol_name)?;

    self
      .scopes
      .get_mut(&scope)
      .and_then(|s| s.symbols.get_mut(symbol_name))
  }

  pub fn add_new_scope(&mut self, parent_key: ScopeKey) -> ScopeKey {
    let key = self.next_scope_id();

    if key != parent_key {
      let parent = self.scopes.get_mut(&parent_key).unwrap();
      parent.children.insert(key);
    }

    let scope = Scope {
      key,
      symbols: HashMap::new(),
      children: HashSet::new(),
      parent: parent_key,
    };

    self.scopes.insert(key, scope);

    key
  }

  fn clear_caches(&mut self, scope_key: ScopeKey) {
    self.scope_symbols_cache.remove(&scope_key);

    // Is there any way to avoid the copy?
    let children = &self.scopes.get(&scope_key).unwrap().children.clone();

    for &sub_scope_key in children {
      self.clear_caches(sub_scope_key);
    }
  }

  pub fn define_symbol(&mut self, scope_key: ScopeKey, name: &str, symbol: Symbol) {
    {
      let scope = self.scopes.get_mut(&scope_key).unwrap();
      scope.symbols.insert(name.to_string(), symbol);
    }

    self.clear_caches(scope_key);
  }

  fn traverse_symbols_in_scope(&mut self, scope_key: ScopeKey) {
    let scope = self.scopes.get(&scope_key).unwrap();
    let mut symbols = HashMap::new();

    fn add_symbols(
      scope_tree: &ScopeTree,
      scope: &Scope,
      symbols: &mut HashMap<String, (ScopeKey, Symbol)>,
    ) {
      if let Some(parent_scope) = scope_tree.get_parent(scope) {
        add_symbols(scope_tree, parent_scope, symbols);
      }

      symbols.extend(
        scope
          .symbols
          .iter()
          .map(|(name, symbol)| (name.clone(), (scope.key, symbol.clone()))),
      );
    }

    add_symbols(self, scope, &mut symbols);

    self.scope_symbols_cache.insert(scope_key, symbols);
  }

  pub fn get_symbols_in_scope(
    &mut self,
    scope_key: ScopeKey,
  ) -> &HashMap<String, (ScopeKey, Symbol)> {
    if !self.scope_symbols_cache.contains_key(&scope_key) {
      self.traverse_symbols_in_scope(scope_key)
    }

    self.scope_symbols_cache.get(&scope_key).unwrap()
  }

  pub fn get_symbols_in_scope_mut(
    &mut self,
    scope_key: ScopeKey,
  ) -> &mut HashMap<String, (ScopeKey, Symbol)> {
    if !self.scope_symbols_cache.contains_key(&scope_key) {
      self.traverse_symbols_in_scope(scope_key)
    }

    self.scope_symbols_cache.get_mut(&scope_key).unwrap()
  }

  pub fn visit_statement(&mut self, statement: &StatementWithCtx, parent: ScopeKey) {
    let block_or_none = match statement.statement {
      Statement::For { ref run, .. } => Some(run),
      _ => None,
    };

    if let Some(block) = block_or_none {
      let scope_key = self.add_new_scope(parent);

      for statement in block {
        self.visit_statement(&statement, scope_key);
      }
    }
  }

  pub fn new() -> ScopeTree {
    let mut scope_tree = ScopeTree {
      scopes: HashMap::new(),
      scope_symbols_cache: HashMap::new(),
      next_scope_id: 0,
    };

    // Add the global scope
    scope_tree.add_new_scope(0);

    scope_tree
  }

  pub fn from_program<'a>(program: Program<'a>) -> ScopeTree {
    let mut scope_tree = ScopeTree::new();

    for statement in program {
      scope_tree.visit_statement(statement, 0);
    }

    println!("{:#?}", scope_tree);

    scope_tree
  }
}
