pub trait VecExt<T> {
  /// Pop items from a vector while the predicate returns true.
  fn pop_while<F>(&mut self, predicate: F)
  where
    // I don't understand this lifetime signature, but compiler told me to put it here.
    for<'r> F: FnMut(&'r &T) -> bool;
}

impl<T> VecExt<T> for Vec<T> {
  fn pop_while<F>(&mut self, predicate: F)
  where
    for<'r> F: FnMut(&'r &T) -> bool,
  {
    let operators_to_pop = self.iter().rev().take_while(predicate).count();

    // And if there were any, remove them.
    if operators_to_pop > 0 {
      let operators_length = self.len();
      self.drain(operators_length - operators_to_pop..operators_length);
      assert_eq!(operators_length - operators_to_pop, self.len());
    }
  }
}

pub trait ResultExt<T, E> {
  fn vec_err(self) -> Result<T, Vec<E>>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
  fn vec_err(self) -> Result<T, Vec<E>> {
    self.map_err(|err| vec![err])
  }
}
