use std::io::{stdin, BufRead};

use runtime::io::Io;

// Since ConsoleIo uses real stdin/stdout it can be a zero-sized type.
pub struct ConsoleIo;

impl Io for ConsoleIo {
  fn read_line(&mut self) -> String {
    let stdin = stdin();
    // Take exclusive lock on stdin, split it into lines and read the first line
    let line = stdin
      .lock()
      .lines()
      .next()
      .expect("There should be a line of input.")
      .expect("Reading should succeed.");
    line
  }

  fn write_line(&mut self, s: &str) {
    println!("{}", s);
  }
}
