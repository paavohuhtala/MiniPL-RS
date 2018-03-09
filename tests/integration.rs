extern crate miniplrs;

use miniplrs::runtime::Io;
use miniplrs::run_script;

struct TestIo {
  input: Vec<String>,
  output: Vec<String>,
}

impl TestIo {
  pub fn new(input: &[&str]) -> TestIo {
    TestIo {
      input: input.iter().map(|s| s.to_string()).collect(),
      output: Vec::new(),
    }
  }
}

impl Io for TestIo {
  fn write_line(&mut self, line: &str) {
    self.output.push(line.to_string());
  }

  fn read_line(&mut self) -> String {
    self.input.remove(0)
  }
}

macro_rules! assert_match {
  ($a:expr => $b:pat) => {
    match $a {
      $b => (),
      _ => {
        panic!("assertion failed: expected pattern {}, was {:?}", stringify!($b), $a);
      }
    }
  }
}

#[test]
pub fn run_hello_world() {
  let source = r#"
    print "Hello, world!";
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);

  assert_match!(result => Ok(()));
  assert_eq!(io.output, vec!["Hello, world!"]);
}

#[test]
pub fn run_hello_world_without_semicolon() {
  use miniplrs::ExecutionError;
  use miniplrs::common::errors::*;
  use miniplrs::common::types::*;

  let source = r#"
    print "Hello, world!"
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);

  assert_match!(result => Err(
    ExecutionError::ParserError(
      ParserError::UnexpectedToken {
        expected: TokenKind::SemicolonK,
        was: TokenKind::EndOfFileK
      }
    )
  ));
  assert_eq!(io.output.len(), 0);
}

#[test]
pub fn bool_operators() {
  let source = r#"
    var a : int := 10;
    var b : int := 20;
    var c : bool := !(a = b) = !(b = a);
    assert c;
  "#;

  let mut io = TestIo::new(&[]);
  let result = run_script(source, &mut io);

  assert_match!(result => Ok(()));
  assert_eq!(io.output.len(), 0);
}
