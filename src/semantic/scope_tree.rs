use std::collections::{HashMap, HashSet};

use common::types::TypeName;

pub type SymbolKey = usize;

#[derive(Clone, Debug)]
pub struct Symbol {
  pub key: SymbolKey,
  pub name: String,
  pub scope_key: ScopeKey,
  pub type_of: TypeName,
  pub is_mutable: bool,
}

pub type ScopeKey = usize;

#[derive(Debug)]
pub struct Scope {
  pub key: ScopeKey,
  parent: ScopeKey,
  children: HashSet<ScopeKey>,
  symbols_by_name: HashMap<String, SymbolKey>,
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
  symbols: HashMap<SymbolKey, Symbol>,
  scope_symbols_cache: HashMap<ScopeKey, HashMap<String, SymbolKey>>,
  next_scope_id: ScopeKey,
  next_symbol_id: ScopeKey,
}

impl ScopeTree {
  fn next_scope_id(&mut self) -> ScopeKey {
    let id = self.next_scope_id;
    self.next_scope_id += 1;
    id
  }

  fn next_symbol_id(&mut self) -> ScopeKey {
    let id = self.next_symbol_id;
    self.next_symbol_id += 1;
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
    let symbol_id = *self.get_symbols_in_scope(scope).get(symbol_name)?;
    self.symbols.get(&symbol_id)
  }

  pub fn get_symbol_mut(&mut self, scope: ScopeKey, symbol_name: &str) -> Option<&mut Symbol> {
    let symbol_id = *self.get_symbols_in_scope(scope).get(symbol_name)?;
    self.symbols.get_mut(&symbol_id)
  }

  pub fn add_new_scope(&mut self, parent_key: ScopeKey) -> ScopeKey {
    let key = self.next_scope_id();

    if key != parent_key {
      let parent = self.scopes.get_mut(&parent_key).unwrap();
      parent.children.insert(key);
    }

    let scope = Scope {
      key,
      symbols_by_name: HashMap::new(),
      children: HashSet::new(),
      parent: parent_key,
    };

    self.scopes.insert(key, scope);

    key
  }

  fn clear_caches(&mut self, scope_key: ScopeKey) {
    self.scope_symbols_cache.remove(&scope_key);

    // Is there any way to avoid the copy?
    let children = self.scopes[&scope_key].children.clone();

    for sub_scope_key in children {
      self.clear_caches(sub_scope_key);
    }
  }

  pub fn define_symbol(
    &mut self,
    scope_key: ScopeKey,
    name: &str,
    type_of: TypeName,
    is_mutable: bool,
  ) -> SymbolKey {
    {
      self.clear_caches(scope_key);
    }

    let key = self.next_symbol_id();

    let scope = self.scopes.get_mut(&scope_key).unwrap();

    let symbol = Symbol {
      key,
      scope_key,
      name: name.to_string(),
      type_of,
      is_mutable,
    };

    self.symbols.insert(symbol.key, symbol);
    scope.symbols_by_name.insert(name.to_string(), key);

    key
  }

  fn traverse_symbols_in_scope(&mut self, scope_key: ScopeKey) {
    let mut symbol_ids = HashMap::new();
    {
      let scope = &self.scopes[&scope_key]; // .get(&scope_key).unwrap();

      let mut scopes = vec![scope];

      fn traverse_upwards<'a>(
        scope_tree: &'a ScopeTree,
        scope: &Scope,
        scopes: &mut Vec<&'a Scope>,
      ) {
        if let Some(parent_scope) = scope_tree.get_parent(scope) {
          scopes.push(parent_scope);
          traverse_upwards(scope_tree, parent_scope, scopes);
        }
      }

      {
        traverse_upwards(self, scope, &mut scopes);
      }

      for (name, &symbol) in scopes.iter().rev().flat_map(|s| s.symbols_by_name.iter()) {
        // Ugly copy
        symbol_ids.insert(name.clone(), symbol);
      }
    }

    self.scope_symbols_cache.insert(scope_key, symbol_ids);
  }

  pub fn get_symbols_in_scope(&mut self, scope_key: ScopeKey) -> &HashMap<String, SymbolKey> {
    if !self.scope_symbols_cache.contains_key(&scope_key) {
      self.traverse_symbols_in_scope(scope_key)
    }

    &self.scope_symbols_cache[&scope_key]
  }

  pub fn get_symbols_in_scope_mut(
    &mut self,
    scope_key: ScopeKey,
  ) -> &mut HashMap<String, SymbolKey> {
    if !self.scope_symbols_cache.contains_key(&scope_key) {
      self.traverse_symbols_in_scope(scope_key)
    }

    self.scope_symbols_cache.get_mut(&scope_key).unwrap()
  }

  pub fn new() -> ScopeTree {
    let mut scope_tree = ScopeTree {
      scopes: HashMap::new(),
      symbols: HashMap::new(),
      scope_symbols_cache: HashMap::new(),
      next_scope_id: 0,
      next_symbol_id: 0,
    };

    // Add the global scope
    scope_tree.add_new_scope(0);

    scope_tree
  }
}
