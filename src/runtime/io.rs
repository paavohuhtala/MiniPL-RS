/// Provides console IO for the interpreter.
pub trait Io {
  fn read_line(&mut self) -> String;
  fn write_line(&mut self, s: &str);
}
