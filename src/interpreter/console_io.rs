use std::io::{stdin, BufRead};

use interpreter::io::Io;

// Since ConsoleIo uses real stdin/stdout it can be a zero-sized type.
pub struct ConsoleIo;

impl Io for ConsoleIo {
  fn read_line(&mut self) -> String {
    let stdin = stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    line
  }

  fn write_line(&mut self, s: &str) {
    println!("{}", s);
  }
}
