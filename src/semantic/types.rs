use common::types::*;

#[derive(Clone, Debug)]
pub struct Symbol {
  pub type_of: TypeName,
  pub is_mutable: bool,
}
